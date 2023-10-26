use crate::chromosome::Chromosome;
use crate::country::Graph;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Clone)]
pub struct Population {
    // The ID of the population
    pub id: u64,
    // The number of individuals for this population.
    pub num_of_individuals: u64,
    // The actual population (vector of individuals).
    pub population: Vec<Chromosome>,
    // The average cost of this population
    pub average_cost: f64,
}

impl Population {
    // A Function to generate a new population based off the size of the population and the cost data
    pub fn new(id: u64, population_size: u64, graph: &Graph) -> Self {
        // Initialise mutable counter variable as 0
        let mut i: u64 = 0;

        // Initialise vector of chromosomes
        let mut population: Vec<Chromosome> = vec![];
        
        // Loop whilst counter is less than population size
        while i < population_size {

            // Add a new chromosome to vector "population"
            population.push(Chromosome::generation(graph));

            // Increment counter
            i += 1;
        }

        // Find average cost of new Population
        let average_cost = Population::find_average_cost(&population);
        
        // Return new Population
        Self { id, num_of_individuals: population_size, population, average_cost }
    }

    // A Function to find and return the average cost of a population given a vector of that populations chromosomes
    pub fn find_average_cost(population: &Vec<Chromosome>) -> f64 {
        // Create mutable variable
        let mut average_cost: f64 = 0.0;

        // Iterate through the population, adding the cost of each chromosome divided by the number of chromosomes to average_cost
        population.iter().for_each(|x| average_cost += (*x).cost / population.len() as f64);

        // Return average_cost
        average_cost
    }

    // A Function to implement the Replace Weakest algorithm
    pub fn replacement(&mut self, child: Chromosome) {
        // Iterate over the population and find the index of the most expensive chromosome
        let index = self.population
                                    .iter()
                                    .enumerate()
                                    // find most expensive chromosome
                                    .max_by(|(_,x), (_,y)| (*x).partial_cmp(y).unwrap())
                                    // strip chromosome from iter, leaving only index
                                    .map(|(a,_)| a)
                                    .unwrap();

        
        // Get the most expensive chromosome from the population
        if let Some(worst_chromosome) = self.population.get_mut(index) {

            // Check that the cost of the worse chromosome is actually greater than the cost of the child
            if worst_chromosome.cost >= child.cost  {

                // Replace the worst chromosome with the child
                *worst_chromosome = child
            }
        }
    }

    pub fn tournament(&mut self, tournament_size: u32, crossover_operator: u8, mutation_operator: u8, graph: &Graph) {
        // Create a Tournament population by randomly selecting "Tournament_size" number of chromosomes from the population
        let mut tournament_population: Vec<Chromosome> = self.population
                                                                .choose_multiple(&mut thread_rng(), tournament_size as usize)
                                                                .cloned()
                                                                .collect();

        // Sort our tournament_population (using the custom implementation of PartialOrd) by cost - this restults in lowest cost first
        tournament_population.sort_by(|x, y| x.partial_cmp(y).unwrap());

        // Remove the first index (and therefore cheapest chromosome) from the tournament population
        let first_parent = tournament_population.remove(0);

        // Remove new first index, as that is the second cheapest chromosome
        let second_parent = tournament_population.remove(0);

        // Use crossover to generate two children from the parents
        let (mut first_child, mut second_child) = first_parent.crossover(&second_parent, crossover_operator, graph);

        // Apply mutation to the two children
        first_child.mutation(mutation_operator, graph);
        second_child.mutation(mutation_operator, graph);

        // Run replacement function with first child first
        self.replacement(first_child);
        // Re-run replacement function with second child
        self.replacement(second_child);
    }
}
