//! This module defines the structure and methods for each [`Chromosome`] in the [`Population`].
//! 
//! [`Population`]: crate::population::Population

use super::{
    country::Graph, 
    interface::{
        MutationOperator, 
        CrossoverOperator
    }
};

use rand::{thread_rng, Rng, seq::{SliceRandom, index}};
use std::cmp::Ordering;
use color_eyre::{eyre::ContextCompat, Result};

/// This defines a chromosome in the population, it has a vector "route" which contains the city numbers in the order they're visited
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
    pub fn new(route: Vec<u32>, cost: f64) -> Self {
        Self { route, cost }
    }

    /// Function to randomly generate a [`Chromosome`]
    pub fn generation(graph: &Graph) -> Result<Self> {
        // Takes a reference to the number of cities (which is the length of the graph vector) and return Self with a randomised route through those citites
        // The route is the order the city appears in the vector whilst the number of the city relates to its index in the Graph struct

        // Calculated the number of cities from the length of the vertex matrix
        let num_cities: usize = graph.vertex.len();

        // Create a vector the length of the number of the cities, initialised as a range from 0 to num_cities -1, i.e 0,1,2,3.....
        let mut vec: Vec<u32> = (0..num_cities as u32).collect();
        // Randomly shuffle the sequence of this vector
        // thread_rng() is a handle to a thread-local CSPRNG with periodic seeding from an interface to the operating system’s random number source
        vec.shuffle(&mut thread_rng());

        let fitness: f64 = Chromosome::fitness(&vec, graph)?;
        // Return this vector as the route in the Chromosome
        Ok(Self {
            route: vec,
            cost: fitness,
        })
    }

    /// Function to use inversion mutation on a [`Chromosome`]
    /// Like rust .. format first index is inclusive and second_index is exclusive
    /// Therefore it must be ensured that they are not the same
    pub fn inversion(&mut self, first_index: usize, second_index: usize) {
        // Create an empty vector with preallocated capacity to improve performace
        let mut new_route: Vec<u32> = Vec::with_capacity(self.route.len());

        // Split the old route into a slice containing all genes before first_index and a slice containing the rest
        let (first_slice, remainder) = self.route.as_slice().split_at(first_index);

        // Split the remainder into a slice containing all genes before second_index and a slice containing those after
        let (centre, second_slice) = remainder.split_at(second_index - first_slice.len());

        // Use .concat() method to flatten two slices together.
        let mut subslice: Vec<u32> = [first_slice, second_slice].concat();

        // Invert the slice
        subslice.reverse();

        // Rebuild the route, using extend_from_slice to append genes in order
        new_route.extend_from_slice(&subslice[0..first_slice.len()]);
        new_route.extend_from_slice(centre);
        new_route.extend_from_slice(&subslice[first_slice.len()..]);

        // Replace the old route with the new one
        let _ = std::mem::replace(&mut self.route, new_route);
    }

    /// Function to mutate a [`Chromosome`]s genes using multiple different methods
    pub fn mutation(&mut self, mutation_operator: MutationOperator, graph: &Graph) -> Result<()> {
        // Pattern match off enum MutationOperator
        match mutation_operator {
            // Inversion
            MutationOperator::Inversion => {
                // Select which  to swap randomly
                let first_index: usize = thread_rng().gen_range(1..=self.route.len());
                let mut second_index: usize = thread_rng().gen_range(1..=self.route.len());
                
                // If the second index is the same as the first, regenerate it
                while second_index == first_index {
                    second_index = thread_rng().gen_range(0..self.route.len());
                }

                match first_index.cmp(&second_index) {
                    // If the first index is lower, use that to create the first slice
                    Ordering::Less => {
                        // Run inversion on chromosome
                        Chromosome::inversion(self, first_index, second_index);
                    
                        // Update the cost of the Chromosome
                        let _ = std::mem::replace(&mut self.cost, Chromosome::fitness(&self.route, graph)?);
                        Ok(())
                    },
                    // If the second index is lower, use that to create the first slice
                    Ordering::Greater => {
                        // Run inversion on chromosome
                        Chromosome::inversion(self, second_index, first_index);

                        // Update the cost of the Chromosome
                        let _ = std::mem::replace(&mut self.cost, Chromosome::fitness(&self.route, graph)?);
                        Ok(())
                    },
                    // Unreachable due to while loop above
                    Ordering::Equal => unreachable!()
                }
            },
            // Single Swap
            MutationOperator::Single => {
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
                let _ = std::mem::replace(&mut self.cost, Chromosome::fitness(&self.route, graph)?);
                Ok(())
            },
            // Multiple Swap
            MutationOperator::Multiple => {
                // Randomly sample 4 distinct indices from 0..self.route.len(), and return them in random order (fully shuffled).
                let results = index::sample(&mut thread_rng(), self.route.len(), 4).into_vec();

                // Swap the first gene with the second gene
                self.route.swap(results[0], results[1]);
                // Swap the third gene with the fourth gene
                self.route.swap(results[2], results[3]);

                // Update the cost of the Chromosome
                let _ = std::mem::replace(&mut self.cost, Chromosome::fitness(&self.route, graph)?);
                Ok(())
            },
        }
    }

    /// Function to fix a crossover, taking the child and slices from both parents
    pub fn fix_crossover(child: &mut Vec<u32>, crossover_point: usize) {
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
            let replacement = std::iter::zip(duplicate_index, missing_gene);
    
            // Loop through replacement
            for (index, gene) in replacement {
                // Replace old gene in child at index with gene
                child.as_mut_slice()[index as usize] = gene
            }
        }
    }

    /// Function to return the ordered crossover of two parents given the indicies to take the crossover slices 
    /// 
    /// An ordered crossover is taking two slices from the parent and keeping those genes the same in the child,
    /// but then reordering the genes outside those slices into the order they appear in the second parent
    pub fn ordered_crossover(
        first_parent: &&[u32], 
        second_parent: &&[u32], 
        crossover_points: &[usize]
    ) -> Result<Vec<u32>> {
        // Define first and second slice using the crossover points
        let first_slice: &[u32] = first_parent
            .get(crossover_points[0]..=crossover_points[1])
            .wrap_err("Error, could not optain Chromosome data")?;
        let second_slice: &[u32] = first_parent
            .get(crossover_points[2]..=crossover_points[3])
            .wrap_err("Error, could not optain Chromosome data")?;

        // Set each value to maximum of u32 for pattern matching
        let mut child: Vec<u32> = vec![u32::MAX; first_parent.len()];

        // Loop through the first slice and add its values to the child at the correct index
        for (index, value) in first_slice.iter().enumerate() {
            child[index + crossover_points[0]] = *value
        }

        // Loop through the second slice and add its values to the child at the correct index
        for (index, value) in second_slice.iter().enumerate() {
            child[index + crossover_points[2]] = *value
        }

        // Create a vector of all the elements in first parent that are not in first_slice or second_slice
        let remainder = first_parent
            .iter()
            .filter(|x| !first_slice.contains(x) && !second_slice.contains(x))
            .copied()
            .collect::<Vec<u32>>();

        // Create a vector to hold the order the remainder elements shoukd be added back with
        let mut replacement: Vec<(usize, u32)> = Vec::with_capacity(remainder.len());

        // For each missing value in remainder, find it index in second parent and add that to relacement
        for value in remainder {
            replacement.push(
                second_parent
                    .iter()
                    .copied()
                    .enumerate()
                    .filter(|(_, x)| x.eq(&value))
                    .last()
                    .wrap_err("Error: Could not obtain Chromosome data")?
            );
        }

        // Sort this vector by its indicies
        replacement.sort_by(|(i, _), (j, _)| i.partial_cmp(j).unwrap());

        // Loop over each gene in replacement
        for (_, x) in replacement.iter() {

            // Ensure gene has not already been added
            if !child.contains(x) {

                // Find first position in child with an unassigned gene (unassigned when the value is u32::MAX)
                let index: usize = child
                    .iter()
                    .position(|y| *y == u32::MAX)
                    .wrap_err("Error: Could not obtain Chromosome data")?;

                // Replace the unassigned gene in child with the new gene
                let _ = std::mem::replace(&mut child[index], *x);
            }
        }
        Ok(child)
    }

    /// Function to perform crossover on two [`Chromosome`]s and return the children
    /// 
    /// A crossover_operator of 0 results in a Crossover with fix
    /// A crossover_operator of 1 results in a Ordered Crossover
    /// NOTE: If the Chromosome is of length u32::MAX (4294967295) then this operation will have undefined behaviour
    pub fn crossover(
        &self, 
        other: &Chromosome, 
        crossover_operator: CrossoverOperator, 
        graph: &Graph
    ) -> Result<(Chromosome, Chromosome)> {

        // Pattern match on specified crossover type
        match crossover_operator {
            // Crossover with Fix
            CrossoverOperator::Fix => {
                // Define the fist parent as Chromosome this function is cast on and the second parent as Chromosome passed into function
                let first_parent: &&[u32] = &self.route.as_slice();
                let second_parent: &&[u32] = &other.route.as_slice();

                // Select crossover point, if 1 all but first gene is swapped, if self.route.len() - 1 last gene is swapped
                let crossover_point: usize = thread_rng().gen_range(1..self.route.len());

                // Here we split the parent vector into two slices and assign whats left of the midpoint to _parent_prefix and whats right (inclusive) to _crossover
                let (first_parent_prefix, first_parent_suffix) = first_parent.split_at(crossover_point);
                let (second_parent_prefix, second_parent_suffix) = second_parent.split_at(crossover_point);
                
                // Use .concat() method to flatten slice. _parent is on the left side and _crossover is onthe right side to preserve order
                let mut first_child: Vec<u32> = [first_parent_prefix, second_parent_suffix].concat();
                let mut second_child: Vec<u32> = [second_parent_prefix, first_parent_suffix].concat();

                // Use previously defined fix_crossover function to fix the crossover should any genes be repeated in the child
                Chromosome::fix_crossover(&mut first_child, crossover_point);
                Chromosome::fix_crossover(&mut second_child, crossover_point);

                // Calculate fitness of the children
                let first_child_fitness: f64 = Chromosome::fitness(&first_child, graph)?;
                let second_child_fitness: f64 = Chromosome::fitness(&second_child, graph)?;

                // Return both Chromosomes in a tuple
                Ok((
                    Chromosome {
                        route: first_child, 
                        cost: first_child_fitness,
                    },   
                    Chromosome {
                        route: second_child, 
                        cost: second_child_fitness,
                    }
                ))
            },
            // Ordered Crossover
            CrossoverOperator::Ordered => {
                // define the fist parent as Chromosome this function is cast on and the second parent as Chromosome passed into function
                let first_parent: &&[u32] = &self.route.as_slice();
                let second_parent: &&[u32] = &other.route.as_slice();

                // Select 4 crossover points so that two slices can be taken from the parent, sort them so slices dont overlap
                let mut crossover_points: Vec<usize> = index::sample(&mut thread_rng(), self.route.len(), 4).into_vec();
                crossover_points.sort();

                let first_child: Vec<u32> = Chromosome::ordered_crossover(first_parent, second_parent, &crossover_points)?;
                let second_child: Vec<u32> = Chromosome::ordered_crossover(second_parent, first_parent, &crossover_points)?;

                // Calculate fitness of the children
                let first_child_fitness: f64 = Chromosome::fitness(&first_child, graph)?;
                let second_child_fitness: f64 = Chromosome::fitness(&second_child, graph)?;

                // Return both Chromosomes in a tuple
                Ok((
                    Chromosome {
                        route: first_child, 
                        cost: first_child_fitness,
                    },   
                    Chromosome {
                        route: second_child, 
                        cost: second_child_fitness,
                    }
                ))
            },
        }
    }

    /// Function to calculate the cost of a [`Chromosome`]
    pub fn fitness(route: &[u32], graph: &Graph) -> Result<f64> {
        let mut cost: f64 = 0.0;

        // Loop over all elements in chromosome
        for (i, x) in route.iter().enumerate() {

            // Cost function include travel from the last city back to the first (or in this representation first to last)
            // This accounts for that
            if i == 0 {
                // Find last city
                let prev: &u32 = route.iter()
                    .last()
                    .wrap_err("Error: Could not obtain Chromosome data")?;

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
        Ok(cost)
    }
}