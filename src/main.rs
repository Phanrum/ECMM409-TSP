// Here I am importing my program
use tsp_coursework::country::*;
use tsp_coursework::simulation::*;

// Here I am importing my external dependancies
// Clap is used to make the command line interface
use clap::Parser;
// Plotters is used to create plots of the data
use plotters::prelude::*;

use indicatif::{MultiProgress, ProgressBar, ProgressState, ProgressStyle};
use std::fmt::Write;

/// This is hardcoded for the course requirement
const NUMBER_OF_GENERATIONS: usize = 10000;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let multi_bar = MultiProgress::new();
    let bar_style = ProgressStyle::with_template("[{elapsed_precise}] [{wide_bar:.cyan/blue}] [{percent}%] ({eta}) {msg}")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-");

    let brazil_bar = multi_bar.add(ProgressBar::new(NUMBER_OF_GENERATIONS as u64));
    let burma_bar = multi_bar.add(ProgressBar::new(NUMBER_OF_GENERATIONS as u64));

    brazil_bar.set_style(bar_style.clone());
    burma_bar.set_style(bar_style);

    let burma = Country::new(false);
    let brazil = Country::new(true);

    let mut brazil_simulation = Simulation::new(brazil, cli.crossover_operator, cli.mutation_operator, cli.population_size, cli.tournament_size);
    let mut burma_simulation = Simulation::new(burma, cli.crossover_operator, cli.mutation_operator, cli.population_size, cli.tournament_size);
    brazil_simulation.run(brazil_bar);
    burma_simulation.run(burma_bar);


    // Create a drawing area with a specified size and coordinate range
    let root = BitMapBackend::new("chart.png", (1920, 1080)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("WOOOOO CHART", ("sans-serif", 50).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..10000f32, 0f32..100000f32)?;

    chart
        .configure_mesh()
                // We can customize the maximum number of labels allowed for each axis
        .x_labels(5)
        .y_labels(5)
        .draw()?;

    // And we can draw something in the drawing area

    let coords = brazil_simulation.best_chromosome
            .iter()
            .enumerate()
            .map(|(x, y)| (x as f32, y.cost as f32))
            .collect::<Vec<(f32, f32)>>();

    chart.draw_series(LineSeries::new(
        coords,
        &RED,
    ))?;

    root.present()?;

    println!("The best Chromosome in Brazil {:?}", brazil_simulation.population.best_chromosome.cost);
    println!("The worst Chromosome in Burma {:?}", burma_simulation.population.worst_chromosome.cost);

    Ok(())
}
