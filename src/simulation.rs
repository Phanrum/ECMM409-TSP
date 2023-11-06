//! This module defines the structure [`Simulation`] and methods for the Simulation of the [`Population`].

use indicatif::ProgressBar;

use color_eyre::{Result, eyre::ContextCompat};

use crate::NUMBER_OF_GENERATIONS;

use super::{
    chromosome::Chromosome, 
    country::Country, 
    interface::{
        MutationOperator, 
        CrossoverOperator
    }, 
    population::Population
};

use rand::{thread_rng, seq::SliceRandom};

// Chrono is used to get the current time and date
use chrono::prelude::*;

// Plotters is used to create plots of the data
use plotters::prelude::*;

/// The `Simulation` type, which contains all the information needed to run the simultation
pub struct Simulation {
    /// Data for the country
    pub country_data: Country,
    /// The actual population of chromosomes for the simulation
    pub population: Population,
    /// Crossover operator: 0 = crossover with fix, 1 = ordered crossover.
    pub crossover_operator: CrossoverOperator,
    /// Mutation operator: 0 = inversion, 1 = single swap mutation, 2 = multiple swap mutation
    pub mutation_operator: MutationOperator,
    /// Population size: Minimum 10, Default 50.
    pub population_size: u64,
    /// Tournament size: Minimum 2, Default 5.
    pub tournament_size: u32,
    /// Number of generations to run simulation for.
    pub generations: u32,
    /// A vector containing the best Chromosome of a generation
    pub best_chromosome: Vec<Chromosome>,
    /// A vector containing the worse Chromosome of a generation
    pub worst_chromosome: Vec<Chromosome>,
    /// A vector containing the average cost of a generation
    pub average_cost: Vec<f64>,
}

/// Implement Methods on the [`Simulation`] type
impl Simulation {
    /// This function creates a new [`Simulation`] with a random [`Population`]
    pub fn new(
        country_data: Country,
        crossover_operator: CrossoverOperator,
        mutation_operator: MutationOperator,
        population_size: u64,
        tournament_size: u32,
    ) -> Result<Self> {
        let new_population = Population::new(population_size, &country_data.graph)?;

        // Allocate these veectors now with the correct capacity so they dont keep reallocating as they grow.
        // They are + 1 because the population starts with these all having one value in them already
        let mut best_chromosome: Vec<Chromosome> =
            Vec::with_capacity(NUMBER_OF_GENERATIONS + 1);
        let mut worst_chromosome: Vec<Chromosome> =
            Vec::with_capacity(NUMBER_OF_GENERATIONS + 1);
        let mut average_cost: Vec<f64> = Vec::with_capacity(NUMBER_OF_GENERATIONS + 1);

        best_chromosome.push(new_population.best_chromosome.clone());
        worst_chromosome.push(new_population.worst_chromosome.clone());
        average_cost.push(new_population.average_population_cost);

        Ok(Simulation {
            country_data,
            population: new_population,
            crossover_operator,
            mutation_operator,
            population_size,
            tournament_size,
            generations: NUMBER_OF_GENERATIONS as u32,
            best_chromosome,
            worst_chromosome,
            average_cost,
        })
    }

    /// This function will run the simulation
    pub fn run(&mut self, progress_bar: ProgressBar) -> Result<()> {
        // Create counter variable
        let mut i: u32 = 1;

        // Loop through this for as many generations as required
        while i < self.generations {
            // Update the population with new children generated from crossover
            self.population.selection_and_replacement(
                self.tournament_size,
                self.crossover_operator,
                self.mutation_operator,
                &self.country_data.graph,
            )?;

            // Update all the stats
            self.best_chromosome
                .push(self.population.best_chromosome.clone());
            self.worst_chromosome
                .push(self.population.worst_chromosome.clone());
            self.average_cost
                .push(self.population.average_population_cost);

            // Increment the counter variable
            i += 1;

            // Change the message displayed to show the current generation
            progress_bar.set_message(format!("Generation {}", i));
            // Set the position of the progress bar to the current generation
            progress_bar.set_position(i as u64);
        }
        // Change message displayed to show that the countries simulation is finished
        progress_bar.finish_with_message(format!("{} Done", self.country_data.name));
        Ok(())
    }

    /// Define function to plot a graph of the best chromosome each generation
    pub fn plot(data: &Vec<Simulation>, id: String) -> Result<()> {
        // Current date and time
        let time: DateTime<Utc> = Utc::now();

        // Generate unique path for plot to be saved to using date, time and id
        let name: String = format!(
            "results/chart-{}-({}).png",
            time.format("%Y-%m-%d-%H-%M-%S"),
            id
        );

        // Create root structure for charts with a specified size, coordinate 
        // range and path and give it a white background
        let root = BitMapBackend::new(name.as_str(), (1920, 1080)).into_drawing_area();
        root.fill(&WHITE)?;

        // Set maximum height for y axis
        let mut y_max: f32 = 0.0;

        // Loop through simulations in data
        for i in data {

            // Define the worst cost as the worst chromosome from the 
            // first generation of the Simulations Population
            let worst = i.worst_chromosome
                .first()
                .wrap_err("Cannot access Chromosome data in Simulation")?;

            // If this worst cost is higher than current one, replace it
            if worst.cost as f32 > y_max {
                y_max = worst.cost as f32
            }
        }

        // Adds 10% to the height of the Y axis
        y_max *= 1.1;

        // Write caption for plot
        let caption = format!(
            "TSP of dataset {}, Population size: {}, Tournament size: {}, Mutation: {:?}, Crossover: {:?}",
            id, 
            data.first().unwrap().population_size, 
            data.first().unwrap().tournament_size,
            data.first().unwrap().mutation_operator,
            data.first().unwrap().crossover_operator,
        );

        // Create a chart for the graph to be drawn on
        let mut chart = ChartBuilder::on(&root)
            .margin(10)
            .caption(caption, ("sans-serif", 30).into_font())
            .margin(10)
            .x_label_area_size(50)
            .y_label_area_size(50)
            .build_cartesian_2d(0f32..NUMBER_OF_GENERATIONS as f32, 0f32..y_max)?;

        // Add a mesh object to chart
        chart.configure_mesh()
            .x_labels(5)
            .x_desc("Generations Passed")
            .y_labels(5)
            .y_desc("Average cost")
            .draw()?;

        // Get array of colours
        let colours = [BLACK, BLUE, CYAN, GREEN, MAGENTA, RED, YELLOW];


        for sim in data {

            // Create vector for x & y coordinates from country data
            let country_coords = sim
                .average_cost
                .iter()
                .enumerate()
                .map(|(x, y)| (x as f32, *y as f32))
                .collect::<Vec<(f32, f32)>>();

            let colour =  colours.choose(&mut thread_rng()).wrap_err("Could not pick colour for line plot")?;

            // Draw country data as a line graoh on chart
            chart.draw_series(LineSeries::new(country_coords, colour))?;
        }

        // Take root and present all charts, then outut final plot
        root.present()?;

        // Return OK if Function runs without error
        Ok(())
}




}
