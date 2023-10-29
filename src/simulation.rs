//! This module defines the structure [`Simulation`] and methods for the Simulation of the [`Population`].

use crate::country::Graph;
use crate::population::Population;

/// This is hardcoded for the course requirement
const NUMBER_OF_GENERATIONS: usize = 10000;

/// The `Simulation` type, which contains all the information needed to run the simultation 
pub struct Simulation {
    /// Data for the country
    pub country_data: Graph,
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
    /// A vector containing the best cost of a generation
    pub best_cost: Vec<f64>,
    /// A vector containing the average cost of a geberation
    pub average_cost: Vec<f64>,
    /// A vector containing the worse cost of a geberation
    pub worst_cost: Vec<f64>,
}

/// Implement Methods on the [`Simulation`] type
impl Simulation {

    /// This function creates a new [`Simulation`] with a random [`Population`]
    fn new(country_data: Graph, crossover_operator: u8, mutation_operator: u8, population_size: u64, tournament_size: u32) -> Self {

        let new_population = Population::new(population_size, &country_data);

        // Allocate this now with the correct capacity so they dont keep reallocating as they grow
        let mut best_cost: Vec<f64> = Vec::with_capacity(NUMBER_OF_GENERATIONS);
        let mut worst_cost: Vec<f64> = Vec::with_capacity(NUMBER_OF_GENERATIONS);
        let mut average_cost: Vec<f64> = Vec::with_capacity(NUMBER_OF_GENERATIONS);

        best_cost.push(new_population.best_population_cost);
        worst_cost.push(new_population.worst_population_cost);
        average_cost.push(new_population.average_population_cost);

        Simulation { 
            country_data, 
            population: Population { 
                population_size, 
                population: new_population.population, 
                average_population_cost: new_population.average_population_cost,
                best_population_cost: new_population.best_population_cost,
                worst_population_cost: new_population.worst_population_cost,
            },
            crossover_operator, 
            mutation_operator, 
            population_size,
            tournament_size, 
            generations: NUMBER_OF_GENERATIONS as u32,
            best_cost,
            worst_cost,
            average_cost,
            }
    }

    /// This function will run the simulation
    fn run(&mut self) -> () {

        let mut i: u32 = 0;

        while i < self.generations {
            
            self.population.selection_and_replacement(self.tournament_size, self.crossover_operator, self.mutation_operator, &self.country_data);

            i += 1;
        }

    }
}