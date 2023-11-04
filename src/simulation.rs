//! This module defines the structure [`Simulation`] and methods for the Simulation of the [`Population`].

use indicatif::ProgressBar;

use super::{chromosome::Chromosome, country::Country, population::Population};

/// The `Simulation` type, which contains all the information needed to run the simultation
pub struct Simulation {
    /// Data for the country
    pub country_data: Country,
    /// The actual population of chromosomes for the simulation
    pub population: Population,
    /// Crossover operator: 0 = crossover with fix, 1 = ordered crossover.
    pub crossover_operator: u8,
    /// Mutation operator: 0 = inversion, 1 = single swap mutation, 2 = multiple swap mutation
    pub mutation_operator: u8,
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
        crossover_operator: u8,
        mutation_operator: u8,
        population_size: u64,
        tournament_size: u32,
    ) -> Self {
        let new_population = Population::new(population_size, &country_data.graph);

        // Allocate these veectors now with the correct capacity so they dont keep reallocating as they grow.
        // They are + 1 because the population starts with these all having one value in them already
        let mut best_chromosome: Vec<Chromosome> =
            Vec::with_capacity(crate::NUMBER_OF_GENERATIONS + 1);
        let mut worst_chromosome: Vec<Chromosome> =
            Vec::with_capacity(crate::NUMBER_OF_GENERATIONS + 1);
        let mut average_cost: Vec<f64> = Vec::with_capacity(crate::NUMBER_OF_GENERATIONS + 1);

        best_chromosome.push(new_population.best_chromosome.clone());
        worst_chromosome.push(new_population.worst_chromosome.clone());
        average_cost.push(new_population.average_population_cost);

        Simulation {
            country_data,
            population: new_population,
            crossover_operator,
            mutation_operator,
            population_size,
            tournament_size,
            generations: crate::NUMBER_OF_GENERATIONS as u32,
            best_chromosome,
            worst_chromosome,
            average_cost,
        }
    }

    /// This function will run the simulation
    pub fn run(&mut self, progress_bar: ProgressBar) {
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
            );

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
    }
}
