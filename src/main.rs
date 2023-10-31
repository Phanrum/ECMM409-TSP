// Here I am importing my program
use tsp_coursework::country::*;
use tsp_coursework::simulation::*;

use std::fmt::Write;
use std::thread;

// Here I am importing my external dependancies
// Clap is used to make the command line interface
use clap::Parser;
// Plotters is used to create plots of the data
use plotters::prelude::*;
// Indicatif is used to create progress bars for the terminal
use indicatif::{MultiProgress, ProgressBar, ProgressState, ProgressStyle};
// Chrono is used to get the current time and date
use chrono::prelude::*;

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

fn plot(country: &Simulation, id: usize) -> Result<(), Box<dyn std::error::Error>> {
    // Current date and time
    let time: DateTime<Utc> = Utc::now();
    let name: String = format!(
        "results/chart-{}-({}).png",
        time.format("%Y-%m-%d-%H-%M-%S"),
        id
    );
    // Create a drawing area with a specified size and coordinate range
    let root = BitMapBackend::new(name.as_str(), (1920, 1080)).into_drawing_area();
    root.fill(&WHITE)?;

    let y_max = country
        .best_chromosome
        .iter()
        .max_by(|x, y| x.partial_cmp(y).unwrap())
        .unwrap()
        .cost;

    let caption = format!(
        "TSP in {}, using a population of {} with a tournament size of {}",
        country.country_data.name, country.population_size, country.tournament_size
    );

    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .caption(caption, ("sans-serif", 50).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..10000f32, 0f32..y_max as f32)?;

    chart.configure_mesh().x_labels(5).y_labels(5).draw()?;

    let country_coords = country
        .best_chromosome
        .iter()
        .enumerate()
        .map(|(x, y)| (x as f32, y.cost as f32))
        .collect::<Vec<(f32, f32)>>();

    chart.draw_series(LineSeries::new(country_coords, &RED))?;

    root.present()?;
    Ok(())
}

fn main() {
    let cli = Cli::parse();

    let multi_bar = MultiProgress::new();
    let bar_style = ProgressStyle::with_template(
        "[{elapsed_precise}] [{wide_bar:.cyan/blue}] [{percent}%] ({eta}) {msg}",
    )
    .unwrap()
    .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
        write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
    })
    .progress_chars("#>-");

    let brazil_bar = multi_bar.add(ProgressBar::new(crate::NUMBER_OF_GENERATIONS as u64));
    brazil_bar.set_style(bar_style.clone());

    let burma_bar = multi_bar.add(ProgressBar::new(crate::NUMBER_OF_GENERATIONS as u64));
    burma_bar.set_style(bar_style);

    let brazil_thread = thread::spawn(move || {
        let brazil = Country::new(true);
        let mut brazil_simulation = Simulation::new(
            brazil,
            cli.crossover_operator,
            cli.mutation_operator,
            cli.population_size,
            cli.tournament_size,
        );
        brazil_simulation.run(brazil_bar);
        brazil_simulation
    });

    let burma_thread = thread::spawn(move || {
        let burma = Country::new(false);
        let mut burma_simulation = Simulation::new(
            burma,
            cli.crossover_operator,
            cli.mutation_operator,
            cli.population_size,
            cli.tournament_size,
        );
        burma_simulation.run(burma_bar);
        burma_simulation
    });

    let finished_brazil = brazil_thread.join().unwrap();
    let finished_burma = burma_thread.join().unwrap();

    plot(&finished_brazil, 1).unwrap();
    plot(&finished_burma, 2).unwrap();

    println!(
        "The best Chromosome in Brazil {:?}",
        finished_brazil.population.best_chromosome.cost
    );
    println!(
        "The best Chromosome in Burma {:?}",
        finished_burma.population.best_chromosome.cost
    );
}
