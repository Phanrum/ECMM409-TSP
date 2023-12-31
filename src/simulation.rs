//! This module defines the structure [`Simulation`] and methods for the Simulation of the [`Population`].

use color_eyre::{Result, eyre::ContextCompat};
use chrono::prelude::*;
use indicatif::ProgressBar;
use plotters::prelude::*;

use super::{
    chromosome::Chromosome, 
    country::Country, 
    interface::*,
    population::Population,
    NUMBER_OF_GENERATIONS
};

/// The `Simulation` type, which contains all the information needed to run the simulation
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

        // Allocate these vectors now with the correct capacity so they don't keep reallocating as they grow.
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
    pub fn plot(
        data: &Vec<Simulation>, 
        plot_operator: PlotOperator, 
        statistic_plotted: PlotStatistic,
        number_runs: u32, 
        id: String
    ) -> Result<()> {
        // Check if a results directory exists
        match std::fs::metadata("results") {
            Ok(_) => (),
            // If it doesn't, create it
            Err(_) => std::fs::create_dir("results")?,
        }

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
        let caption: String = format!(
            "TSP of dataset {}, Ran {} times, Population size: {}, Tournament size: {}, Mutation: {:?}, Crossover: {:?}",
            id, 
            number_runs,
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


        let mut data_simplified: Vec<Vec<f64>> = Vec::with_capacity(data.capacity());

         match statistic_plotted {
            PlotStatistic::Average => {
                // Iterate over data
                data.iter()
                    // For each Simulation in data, push its average_cost field to data_simplified
                    .for_each(|sim| data_simplified.push(sim.average_cost.clone()))

            },
            PlotStatistic::Best => {
                // Iterate over data
                data.iter().for_each(|sim| {
                    data_simplified
                        // Iterate over the best chromosome field in the Simulation, collect its costs into a vector
                        // and push this vector to data_simplified
                        .push({sim
                            .best_chromosome
                            .iter()
                            .map(|chromo| chromo.cost)
                            .collect::<Vec<f64>>()
                        })
                })
            },
            PlotStatistic::Worst => {
                // Iterate over data
                data.iter().for_each(|sim| {
                    data_simplified
                        // Iterate over the worst chromosome field in the Simulation, collect its costs into a vector
                        // and push this vector to data_simplified
                        .push({sim
                            .worst_chromosome
                            .iter()
                            .map(|chromo| chromo.cost)
                            .collect::<Vec<f64>>()
                        })
                })
            },
        };

        // Pattern match on specified plot type
        match plot_operator {
            
            PlotOperator::Average => {
                // Create vector for average co-ords with the length 
                // equal to the length of the first Simulations average_cost
                let mut average_coords: Vec<f32> = vec![0.0; data_simplified[0].len()];

                // Loop over every array in data_simplified
                data_simplified.iter().for_each(|array| {
                    // Loop over every element in the array
                    array.iter().enumerate().for_each(|(index, value)| {
                        // Get value of array at index, divide it by 
                        // number of arrays and add it to value at index in average_coords
                        average_coords[index] += (*value as f32) / (data_simplified.len() as f32)
                    })
                });

                // plotters requires coordinates to be in the form (f32, f32) 
                let output: Vec<(f32, f32)> = average_coords
                    // Iterate over average_coords
                    .iter_mut()
                    // Get index of co-ords, elements are now (usize, f32)
                    .enumerate()
                    // Convert index from usize to f32, elements are now (f32, f32)
                    .map(|(i, x)| (i as f32, *x))
                    // Collect elements into new 
                    .collect::<Vec<(f32, f32)>>();

                // Get final cost of average Simulation
                let average_final = output.last().wrap_err("Chromosome data not found")?.1;
    
                // Draw country data as a line graph on chart
                chart.draw_series(LineSeries::new(output, RED.mix(0.9).stroke_width(2)))?;

                println!("Last cost of {} best simulation: {}", id, average_final);

                // Take root and present all charts, then output final plot
                root.present()?;
            },

            PlotOperator::Best => {
                
                let country_coords: Vec<(f32, f32)> = data_simplified
                    .iter()
                    .min_by(|x, y| { x.last()
                        .unwrap()
                        .partial_cmp(y
                            .last().unwrap()
                        ).unwrap()
                    }).wrap_err("Could not find Chromosome data in Simulation")?
                    .iter()
                    .enumerate()
                    .map(|(x, y)| (x as f32, *y as f32))
                    .collect::<Vec<(f32, f32)>>();

                // Get final cost of best Simulation
                let best_final = country_coords.last().wrap_err("Chromosome data not found")?.1;

                // Draw country data as a line graph on chart
                chart.draw_series(LineSeries::new(country_coords, RED.mix(0.9).stroke_width(2)))?;

                println!("Last cost of {} best simulation: {}", id, best_final);

                // Take root and present all charts, then output final plot
                root.present()?;

            },

            PlotOperator::Worst => {
                
                let country_coords: Vec<(f32, f32)> = data_simplified
                    .iter()
                    .max_by(|x, y| { x.last()
                        .unwrap()
                        .partial_cmp(y
                            .last().unwrap()
                        ).unwrap()
                    }).wrap_err("Could not find Chromosome data in Simulation")?
                    .iter()
                    .enumerate()
                    .map(|(x, y)| (x as f32, *y as f32))
                    .collect::<Vec<(f32, f32)>>();

                // Get final cost of worst Simulation
                let worst_final = country_coords.last().wrap_err("Chromosome data not found")?.1;

                // Draw country data as a line graph on chart
                chart.draw_series(LineSeries::new(country_coords, RED.mix(0.9).stroke_width(2)))?;

                println!("Last cost of {} worst simulation: {}",id , worst_final);

                // Take root and present all charts, then output final plot
                root.present()?;
            },

            PlotOperator::Range => {

                let worst_coords: Vec<(f32, f32)> = data_simplified
                    .iter()
                    .max_by(|x, y| { x.last()
                        .unwrap()
                        .partial_cmp(y
                            .last().unwrap()
                        ).unwrap()
                    }).wrap_err("Could not find Chromosome data in Simulation")?
                    .iter()
                    .enumerate()
                    .map(|(x, y)| (x as f32, *y as f32))
                    .collect::<Vec<(f32, f32)>>();

                // Get final cost of worst Simulation
                let worst_final = worst_coords.last().wrap_err("Chromosome data not found")?.1;


                let best_coords: Vec<(f32, f32)> = data_simplified
                    .iter()
                    .min_by(|x, y| { x.last()
                        .unwrap()
                        .partial_cmp(y
                            .last().unwrap()
                        ).unwrap()
                    }).wrap_err("Could not find Chromosome data in Simulation")?
                    .iter()
                    .enumerate()
                    .map(|(x, y)| (x as f32, *y as f32))
                    .collect::<Vec<(f32, f32)>>();

                // Get final cost of best Simulation
                let best_final = best_coords.last().wrap_err("Chromosome data not found")?.1;

                // Create vector for average co-ords with the length 
                // equal to the length of the first Simulations average_cost
                let mut average_coords: Vec<f32> = vec![0.0; data_simplified[0].len()];

                // Loop over every array in data_simplified
                data_simplified.iter().for_each(|array| {
                    // Loop over every element in the array
                    array.iter().enumerate().for_each(|(index, value)| {
                        // Get value of array at index, divide it by 
                        // number of arrays and add it to value at index in average_coords
                        average_coords[index] += (*value as f32) / (data_simplified.len() as f32)
                    })
                });

                // plotters requires coordinates to be in the form (f32, f32) 
                let output: Vec<(f32, f32)> = average_coords
                    // Iterate over average_coords
                    .iter_mut()
                    // Get index of co-ords, elements are now (usize, f32)
                    .enumerate()
                    // Convert index from usize to f32, elements are now (f32, f32)
                    .map(|(i, x)| (i as f32, *x))
                    // Collect elements into new vector
                    .collect::<Vec<(f32, f32)>>();

                // Get final cost of average Simulation
                let average_final = output.last().wrap_err("Chromosome data not found")?.1;

                // Draw Worst Chromosome data as a line graph on chart
                chart.draw_series(LineSeries::new(worst_coords, RED.mix(0.9).stroke_width(2)))?
                    .label("Worst Simulation")
                    .legend(|(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], RED.mix(0.9).filled()));

                // Draw Average Chromosome data as a line graph on chart
                chart.draw_series(LineSeries::new(output, BLUE.mix(0.9).stroke_width(2)))?
                    .label("Average Simulation")
                    .legend(|(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], BLUE.mix(0.9).filled()));

                // Draw Best Chromosome data as a line graph on chart
                chart.draw_series(LineSeries::new(best_coords, GREEN.mix(0.9).stroke_width(2)))?
                    .label("Best Simulation")
                    .legend(|(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], GREEN.mix(0.9).filled()));

                // Draw legend on graph
                chart.configure_series_labels()
                    .background_style(&WHITE.mix(0.8))
                    .border_style(&BLACK)
                    .draw()?;

                println!("Last cost of {} worst simulation: {}",id , worst_final);
                println!("Last cost of {} best simulation: {}", id, best_final);
                println!("Last cost of {} average simulation: {}", id, average_final);

                // Take root and present all charts, then output final plot
                root.present()?;
            },

            PlotOperator::DisplayAll => {
                // Loop over every Simulation in data
                for (index, array) in data_simplified.iter().enumerate() {

                    // Create vector for x & y coordinates from country data
                    let country_coords: Vec<(f32, f32)> = array
                        .iter()
                        .enumerate()
                        .map(|(x, y)| (x as f32, *y as f32))
                        .collect::<Vec<(f32, f32)>>();
        
                    // Randomly select colour for the line
                    let colour =  Palette99::pick(index).mix(0.9);

                    // Get final cost of Simulation
                    let country_final = country_coords.last().wrap_err("Chromosome data not found")?.1;

                    // Draw country data as a line graph on chart
                    chart.draw_series(LineSeries::new(country_coords, colour.stroke_width(2)))?
                        .label(format!("Simulation {}", index + 1))
                        .legend(move |(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], colour.filled()));

                    // Output final cost
                    println!("Last cost of {} simulation {}: {}", id, index + 1, country_final);
                }

                // Draw legend on graph
                chart.configure_series_labels()
                    .background_style(&WHITE.mix(0.8))
                    .border_style(&BLACK)
                    .draw()?;

                // Take root and present all charts, then output final plot
                root.present()?;
            },
        };

        // Return OK if Function runs without error
        Ok(())
    }
}
