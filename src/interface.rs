// Clap is used to make the command line interface
use clap::{Parser, ValueEnum};

/// A Rust program to solve the Travelling Salesman Problem. It uses a steady state evolutionary algorithm
/// and assumes its given XML files detailing the costs associated with travel between each city.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Which crossover type to use:
    #[arg(value_enum, default_value_t = CrossoverOperator::Fix, short, long)]
    pub crossover_operator: CrossoverOperator,
    /// Which mutation type to use:
    #[arg(value_enum, default_value_t = MutationOperator::Single, short, long)]
    pub mutation_operator: MutationOperator,
    /// Population size: Minimum 10.
    #[arg(value_parser = clap::value_parser!(u64).range(10..), default_value_t = 50, short, long)]
    pub population_size: u64,
    /// Tournament size: Minimum 2. Cannot exceed population size
    #[arg(value_parser = clap::value_parser!(u32).range(2..), default_value_t = 5, short, long)]
    pub tournament_size: u32,
    /// Number of Runs: Minumum 1.
    #[arg(value_parser = clap::value_parser!(u32).range(1..), default_value_t = 1, short, long)]
    pub number_runs: u32,
}

/// Enumerate that represents the possible state of the mutation type
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum MutationOperator {

    /// Runs inversion mutation on the chromosomes
    Inversion,

    /// Runs single swap mutation on the chromosomes
    Single,

    /// Runs multiple swap mutation on the chromosomes
    Multiple,
}

/// Enumerate that represents the possible state of the crossover type
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum CrossoverOperator {

    /// Runs crossover with fix on the chromosomes
    Fix,

    /// Runs ordered crossover on the chromosomes
    Ordered,
}