//! This module defines [`Population`] and all its methods


use super::{chromosome::Chromosome, country::Graph};


use rand::{thread_rng, seq::SliceRandom};

/// The struct defines the population
#[derive(Clone)]
pub struct Population {
    /// The number of individuals for this population.
    pub population_size: u64,
    /// The actual population (vector of individuals).
    pub population_data: Vec<Chromosome>,
    /// The average cost of this population
    pub average_population_cost: f64,
    /// The best Chromosome in the population
    pub best_chromosome: Chromosome,
    /// The worst Chromosome in this population
    pub worst_chromosome: Chromosome,
}

/// Implements methods on `Population`
impl Population {
    /// A Function to generate a new population of [`Chromosome`]s based off the size of the population and the cost data
    pub fn new(population_size: u64, country_data: &Graph) -> Self {
        // Initialise mutable counter variable as 0
        let mut i: u64 = 0;

        // Initialise vector of chromosomes
        let mut population_data: Vec<Chromosome> = vec![];
        
        // Loop whilst counter is less than population size
        while i < population_size {

            // Add a new chromosome to vector "population"
            population_data.push(Chromosome::generation(country_data));

            // Increment counter
            i += 1;
        }

        // Find best Chromosome in population
        let best_chromosome = Population::find_best_chromosome(&population_data);

        // Find worst Chromosome in the population
        let worst_chromosome = Population::find_worst_chromosome(&population_data);

        // Find average cost of new Population
        let average_population_cost = Population::find_average_cost(&population_data);

        // Return new Population
        Self { 
            population_size, 
            population_data, 
            average_population_cost,
            best_chromosome,
            worst_chromosome,
        }
    }

    /// A Function to find and return the average cost of a population given a vector of that populations chromosomes
    pub fn find_average_cost(population_data: &[Chromosome]) -> f64 {
        // Create mutable variable
        let mut average_cost: f64 = 0.0;

        // Iterate through the population, adding the cost of each chromosome divided by the number of chromosomes to average_cost
        population_data.iter().for_each(|x| average_cost += x.cost / population_data.len() as f64);

        // Return average_cost
        average_cost
    }

    /// A function to find the worst Chromosome in the population
    pub fn find_worst_chromosome(population_data: &[Chromosome]) -> Chromosome {
        let worst = population_data
            .iter()
            .max_by(|x, y| x.partial_cmp(y).unwrap())
            .unwrap();
        worst.to_owned()
    }

    /// A function to find the best Cromosome in the population
    pub fn find_best_chromosome(population_data: &[Chromosome]) -> Chromosome {
        let best = population_data
            .iter()
            .min_by(|x, y| x.partial_cmp(y).unwrap())
            .unwrap();
        best.to_owned()
    }

    /// A Function to implement the Replace Weakest algorithm
    pub fn replacement(&mut self, child: Chromosome) {
        // Iterate over the population_data and find the index of the most expensive chromosome
        let worst_chromosome: (usize, Chromosome) = self.population_data
            .iter()
            .enumerate()
            // find most expensive chromosome
            .max_by(|(_,x), (_,y)| x.partial_cmp(y).unwrap())
            .map(|(i, x)| (i, x.to_owned()))
            // strip chromosome from iter, leaving only index
            .unwrap();

        
        // Check that the cost of the worse chromosome is actually greater than the cost of the child
        if worst_chromosome.1.cost >= child.cost {

            // Replace the worst chromosome with the child
            let _ = std::mem::replace( &mut self.population_data[worst_chromosome.0], child);
        }
    }

    /// This function takes a tournament size, randomly picks that many chromosomes from 
    /// the population and returns the best ones
    pub fn run_tournament(&self, tournament_size: u32) -> Chromosome {
        // Create a Tournament population by randomly selecting "Tournament_size" number of chromosomes from the population
        let mut tournament_population: Vec<Chromosome> = self.population_data
            .choose_multiple(&mut thread_rng(), tournament_size as usize)
            .cloned()
            .collect();

        // Sort our tournament_population (using the custom implementation of PartialOrd) by cost - this restults in lowest cost first
        tournament_population.sort_by(|x, y| x.partial_cmp(y).unwrap());

        // Remove and return the first index (and therefore cheapest chromosome) from the tournament population
        tournament_population.remove(0)
    }

    /// This function runs a tournament twice to obtain two parents, then it creates two children from those
    /// parents. It will take the first child and if it is better than the worst chromosome in the population
    /// it will replace it. Then it will do the same with the second child.
    pub fn selection_and_replacement(&mut self, tournament_size: u32, crossover_operator: u8, mutation_operator: u8, country_data: &Graph) {

        // Select first and second parents using tournaments
        let first_parent = Population::run_tournament(&self, tournament_size);
        let second_parent = Population::run_tournament(&self, tournament_size);

        // Use crossover to generate two children from the parents
        let (mut first_child, mut second_child) = first_parent.crossover(&second_parent, crossover_operator, country_data);

        // Apply mutation to the two children
        first_child.mutation(mutation_operator, country_data);
        second_child.mutation(mutation_operator, country_data);

        // Run replacement function with first child first
        self.replacement(first_child);
        // Re-run replacement function with second child
        self.replacement(second_child);

        // Update old population stats with new ones
        let _ = std::mem::replace(&mut self.average_population_cost, Population::find_average_cost(&self.population_data));
        let _ = std::mem::replace(&mut self.best_chromosome, Population::find_best_chromosome(&self.population_data));
        let _ = std::mem::replace(&mut self.worst_chromosome, Population::find_worst_chromosome(&self.population_data));
    }
}