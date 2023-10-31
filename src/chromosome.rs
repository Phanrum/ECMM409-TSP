//! This module defines the structure and methods for each [`Chromosome`] in the [`Population`][pop].
//! 
//! [pop]: crate::population::Population

use crate::country::Graph;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::iter::zip;

/// This defines the chromosome in the population, it has a vector "route" which contains the city numbers in the order they're visited
#[derive(Clone, Debug)]
pub struct Chromosome {
    pub route: Vec<u32>,
    pub cost: f64,
}

/// Implements [`PartialEq`] for Chromosome so two chromosomes can be tested for equality or lack thereof
impl PartialEq for Chromosome {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

/// Implements [`PartialOrd`] for Chromosome so that two chromosomes can be correctly compared on cost.
/// Rust will not implement Ordering for floats, therefore I have to cast them to intergers for the comparison.
/// All costs in the XML file were given in scientific notation but fortunatly all expand out to intergers so this is possible
impl PartialOrd for Chromosome {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((self.cost as usize).cmp(&(other.cost as usize)))
    }
}

/// Implement functions for Chromosome type
impl Chromosome {

    /// Function to create a [`Chromosome`] from given route vector and cost
    /// Only useful for testing as in all other cases we need random generation, 
    /// use [`generation`]
    /// 
    /// [`generation`]: Chromosome::generation
    pub fn new (route: Vec<u32>, cost: f64) -> Self {
        Self { route, cost }
    }

    /// Function to randomly generate a [`Chromosome`]
    pub fn generation(graph: &Graph) -> Self {
        // Takes a reference to the number of cities (which is the length of the graph vector) and return Self with a randomised route through those citites
        // The route is the order the city appears in the vector whilst the number of the city relates to its index in the Graph struct

        // Calculated the number of cities from the length of the vertex matrix
        let num_cities = graph.vertex.len();

        // Create a vector the length of the number of the cities, initialised as a range from 0 to num_cities -1, i.e 0,1,2,3.....
        let mut vec: Vec<u32> = (0..num_cities as u32).collect();
        // Randomly shuffle the sequence of this vector
        // thread_rng() is a handle to a thread-local CSPRNG with periodic seeding from an interface to the operating system’s random number source
        vec.shuffle(&mut thread_rng());

        let fitness: f64 = Chromosome::fitness(&vec, graph);
        // Return this vector as the route in the Chromosome
        Self {
            route: vec,
            cost: fitness,
        }
    }

    /// Function to mutate a [`Chromosome`]s genes using multiple different methods
    pub fn mutation(&mut self, mutation_operator: u8, graph: &Graph) {
        match mutation_operator {
            // Inversion
            0 => todo!(),
            // Single Swap
            1 => {
                // Select which genes to swap randomly
                let first_gene: usize = thread_rng().gen_range(0..self.route.len());
                let mut second_gene: usize = thread_rng().gen_range(0..self.route.len());

                // If the second gene is the same as the first, regenerate it
                while second_gene == first_gene {
                    second_gene = thread_rng().gen_range(0..self.route.len());
                }

                // Swap the first gene with the second gene
                self.route.swap(first_gene, second_gene);
                // Update the cost of the Chromosome
                let _ = std::mem::replace(&mut self.cost, Chromosome::fitness(&self.route, graph));
            }
            // Multiple Swap
            2 => {
                let first_gene: usize = thread_rng().gen_range(0..self.route.len());
                let mut second_gene: usize = thread_rng().gen_range(0..self.route.len());

                // If the second gene is the same as the first, regenerate it
                while second_gene == first_gene {
                    second_gene = thread_rng().gen_range(0..self.route.len());
                }

                let third_gene: usize = thread_rng().gen_range(0..self.route.len());
                let mut fourth_gene: usize = thread_rng().gen_range(0..self.route.len());

                // If the second gene is the same as the first, regenerate it
                while fourth_gene == third_gene {
                    fourth_gene = thread_rng().gen_range(0..self.route.len());
                }

                // Swap the first gene with the second gene
                self.route.swap(first_gene, second_gene);
                // Swap the third gene with the fourth gene
                self.route.swap(third_gene, fourth_gene);

                // Update the cost of the Chromosome
                let _ = std::mem::replace(&mut self.cost, Chromosome::fitness(&self.route, graph));
            },
            
            // No other options are possible as Clap's Value Parser will reject them
            _ => unreachable!(),
        }
    }

    /// Function to fix a crossover, taking the child and slices from both parents
    fn fix_crossover(child: &mut Vec<u32>, crossover_point: usize) {
 
        // Create a list containing every gene
        let master_list: Vec<u32> = (0..child.len() as u32).collect();

        // Only child.len() - crossover_point genes are swapped so thats the maximun number that could be duplicated
        let mut missing_gene: Vec<u32> = Vec::with_capacity(child.len() - crossover_point);

        // Iterate over the master_list and add each missing gene to missing_gene
        master_list
            .iter()
            .filter(|x| !child.contains(*x))
            .for_each(|x| missing_gene.push(*x));

        // Check if there are any duplicates before dong the expensive computation below
        if !master_list.is_empty() {

            // Create a list for the index of the first duplicated gene
            let mut duplicate_index: Vec<u32> = Vec::with_capacity(child.len() - crossover_point);

            // Iterate through child
            for (i, x) in child.iter().enumerate() {
                // For each gene in child, iterate over child again
                for (j, y) in child.iter().enumerate() {
                    // if the elements are the same and the index of the outer loop is 
                    // than that of the inner, add outer loop index to duplicate_index
                    if x.eq(y) && i.lt(&j) {
                        duplicate_index.push(i as u32);
                    }
                }
            }
        
            // Zips each element from duplicate_index with its counterpart in missing_gene into an iterator of tuples
            let replacement = zip(duplicate_index, missing_gene);
    
            // Loop through replacement
            for (index, gene) in replacement {
                // Replace old gene in child at index with gene
                child.as_mut_slice()[index as usize] = gene
            }
        }
    }


    /// Function to perform crossover on two [`Chromosome`]s and return the children
    pub fn crossover(&self, other: &Chromosome, crossover_operator: u8, graph: &Graph) -> (Chromosome, Chromosome) {
        match crossover_operator {
            // Crossover with Fix
            0 => {
                // define fist parent as Chromosome this function is cast on and second parent as Chromosome passed into function
                let first_parent = &self.route;
                let second_parent = &other.route;

                // Select crossover point, if 1 all but first gene is swapped, if self.route.len() - 1 last gene is swapped
                let crossover_point: usize = thread_rng().gen_range(1..self.route.len());

                // Here we split the parent vector into two slices and assign whats left of the midpoint to _parent_prefix and whats right (inclusive) to _crossover
                let (first_parent_prefix, first_parent_suffix) = first_parent.split_at(crossover_point);
                let (second_parent_prefix, second_parent_suffix) = second_parent.split_at(crossover_point);
                
                // Use .concat() method to flatten slice. _parent is on the left side and _crossover is onthe right side to preserve order
                let mut first_child = [first_parent_prefix, second_parent_suffix].concat();
                let mut second_child = [second_parent_prefix, first_parent_suffix].concat();

                // Use previously defined fix_crossover function to fix the crossover should any genes be repeated in the child
                Chromosome::fix_crossover(&mut first_child, crossover_point);
                Chromosome::fix_crossover(&mut second_child, crossover_point);

                // Calculate fitness of the children
                let first_child_fitness = Chromosome::fitness(&first_child, graph);
                let second_child_fitness = Chromosome::fitness(&second_child, graph);

                // Return both children in a tuple
                (Chromosome {route: first_child, cost: first_child_fitness}, 
                Chromosome {route: second_child, cost: second_child_fitness})
            }
            // Ordered Crossover
            1 => todo!(),
            // No other options are possible as Clap's Value Parser will reject them
            _ => unreachable!(),
        }
    }

    /// Function to calculate the cost of a [`Chromosome`]
    pub fn fitness(route: &[u32], graph: &Graph) -> f64 {
        let mut cost: f64 = 0.0;

        // Loop over all elements in chromosome
        for (i, x) in route.iter().enumerate() {

            // Cost function include travel from the last city back to the first (or in this representation first to last)
            // This accounts for that
            if i == 0 {
                // Find last city
                let prev = route.iter().last().unwrap();

                // Loop through each city in country
                for (index, vert) in graph.vertex.iter().enumerate() {
                    // Loop over each edge between all other cities and this one
                    for edge in vert {
                        // If the city is the last city and the edge is the connection between the last and the first
                        if index == *prev as usize && edge.destination_city == *x {
                            // Add this cost to the cost variable
                            cost += edge.cost
                        }
                    }
                }
            } else {

                // Loop through each city in the country
                for (index, vert) in graph.vertex.iter().enumerate() {
                    // Loop through each edge between all other cities and this one
                    for edge in vert {
                        // If the city is the previous city in the route and edge is the connection to the current city in the route
                        if index == route[i - 1] as usize && edge.destination_city == *x {
                            // Add this cost to the cost variable
                            cost += edge.cost
                        }
                    }
                }
            }
        }
        // Return cost
        cost
    }
}

#[cfg(test)]
mod test {
    use crate::country::Country;
    use super::*;

    const SRC: &str = r#"<travellingSalesmanProblemInstance>
    <name>burma14</name>
    <source>TSPLIB</source>
    <description>14-Staedte in Burma (Zaw Win)</description>
    <doublePrecision>15</doublePrecision>
    <ignoredDigits>5</ignoredDigits>
    <graph>
      <vertex>
        <edge cost="1.530000000000000e+02">1</edge>
        <edge cost="5.100000000000000e+02">2</edge>
        <edge cost="7.060000000000000e+02">3</edge>
      </vertex>
      <vertex>
        <edge cost="1.530000000000000e+02">0</edge>
        <edge cost="4.220000000000000e+02">2</edge>
        <edge cost="6.640000000000000e+02">3</edge>
      </vertex>
      <vertex>
        <edge cost="5.100000000000000e+02">0</edge>
        <edge cost="4.220000000000000e+02">1</edge>
        <edge cost="2.890000000000000e+02">3</edge>
      </vertex>
      <vertex>
        <edge cost="7.060000000000000e+02">0</edge>
        <edge cost="6.640000000000000e+02">1</edge>
        <edge cost="2.890000000000000e+02">2</edge>
      </vertex>
    </graph>
    </travellingSalesmanProblemInstance>"#;

    #[test]
    fn check_fitness(){

        let burma_small: Country = serde_xml_rs::from_str(SRC).unwrap();
        let route = vec![2, 0, 1, 3];
        let cost = 289.0 + 510.0 + 153.0 + 664.0;
        let test_chromosome = Chromosome::new(route, cost);

        assert_eq!(cost, Chromosome::fitness(&test_chromosome.route, &burma_small.graph), "my cost calculated {} and functions cost {}", cost, Chromosome::fitness(&test_chromosome.route, &burma_small.graph));
    }

    #[test]
    fn check_crossover_fix() {

        // first parent = [2, 1, 0, 3]
        // second parent = [0, 2, 3, 1]
            

        let child_original: Vec<u32> = vec![2,3,0,1];
        
        let mut child_mut: Vec<u32> = vec![2,1,0,1];

        let crossover_point = 3;

        Chromosome::fix_crossover(&mut child_mut, crossover_point);
        assert_eq!(child_mut, child_original, "expected: {:?} actual: {:?}", child_original, child_mut);

    }

    #[test]
    fn check_crossover() {

        // crossover point 3
        // p1 [2, 1, 0, 3]
        // p2 [0, 2, 3, 1]

        // c1 [2, 2, 0, 1]
        // c2 [0, 2, 0, 3]

        let burma_small: Country = serde_xml_rs::from_str(SRC).unwrap();
        let parent_one = Chromosome::generation(&burma_small.graph);
        let parent_two = Chromosome::generation(&burma_small.graph);

        let (child_one, child_two) = parent_one.crossover(&parent_two, 0, &burma_small.graph);

        println!("first child: {:?} second child: {:?} first parent: {:?} second parent: {:?}", child_one, child_two, parent_one, parent_two)
    }
}