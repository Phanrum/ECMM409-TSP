// Here I am importing my program
use tsp_coursework::country::*;
//use tsp_coursework::chromosome::*;
use tsp_coursework::population::*;

// Here I am importing my external dependancies
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Crossover operator: 0 = crossover with fix, 1 = ordered crossover.
    #[arg(value_parser = clap::value_parser!(u8).range(0..=1), default_value_t = 0, short, long)]
    crossover: u8,
    /// Mutation operator: 0 = inversion, 1 = single swap mutation, 2 = multiple swap mutation
    #[arg(value_parser = clap::value_parser!(u8).range(0..=2), default_value_t = 1, short, long)]
    mutation: u8,
    /// Population size: Minimum 10, Default 50.
    #[arg(value_parser = clap::value_parser!(u64).range(10..), default_value_t = 50, short, long)]
    population: u64,
    /// Tournament size: Minimum 2, Default 5.
    #[arg(value_parser = clap::value_parser!(u32).range(2..), default_value_t = 5, short, long)]
    tournament: u32,
}

fn main() {
    let cli = Cli::parse();

    let Burma = Country::new(false);
    let Brazil = Country::new(true);

    println!("There are {} cities in Burma", Burma.graph.vertex.len());
    println!("There are {} cities in Brazil", Brazil.graph.vertex.len());
}
