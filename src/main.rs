// Here I am importing my program
use tsp_coursework::country::*;
use tsp_coursework::simulation::*;

// Here I am importing my external dependancies
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Crossover operator: 0 = crossover with fix, 1 = ordered crossover.
    #[arg(value_parser = clap::value_parser!(u8).range(0..=1), default_value_t = 0, short, long)]
    crossover_operator: u8,
    /// Mutation operator: 0 = inversion, 1 = single swap mutation, 2 = multiple swap mutation
    #[arg(value_parser = clap::value_parser!(u8).range(0..=2), default_value_t = 1, short, long)]
    mutation_operator: u8,
    /// Population size: Minimum 10, Default 50.
    #[arg(value_parser = clap::value_parser!(u64).range(10..), default_value_t = 50, short, long)]
    population_size: u64,
    /// Tournament size: Minimum 2, Default 5.
    #[arg(value_parser = clap::value_parser!(u32).range(2..), default_value_t = 5, short, long)]
    tournament_size: u32,
}

fn main() {
    let cli = Cli::parse();

    let burma = Country::new(false);
    let brazil = Country::new(true);

    let burma_simulation = Simulation::new(burma.graph, cli.crossover_operator, cli.mutation_operator, cli.population_size, cli.tournament_size);

    println!("The best Chromosome after 10,000 Generations is {:?}", burma_simulation.population.best_chromosome);


}
