// Importing my programs modules
use tsp_coursework::{country::Country, simulation::Simulation};

// Importing thread to allow for multithreading
use std::thread;

// Here I am importing my external dependancies:
// Clap is used to make the command line interface
use clap::Parser;
// Plotters is used to create plots of the data
use plotters::prelude::*;
// Indicatif is used to create progress bars for the terminal
// Import Write from standard library to output custom key
use indicatif::{MultiProgress, ProgressBar, ProgressState, ProgressStyle};
use std::fmt::Write;
// Chrono is used to get the current time and date
use chrono::prelude::*;
// Colour_Eyre is used to neatly propagate errors
use color_eyre::{Result, eyre::ContextCompat};

/// This is hardcoded for the course requirement
const NUMBER_OF_GENERATIONS: usize = 10000;

/// A Rust program to solve the Travelling Salesman Problem. It uses a steady state evolutionary algorithm
///  and assumes its given XML files detailing the costs associated with travel between each city.
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

/// Define function to plot a graph of the best chromosome each generation
fn plot(country: &Simulation, id: usize) -> Result<()> {
    // Current date and time
    let time: DateTime<Utc> = Utc::now();

    // Generate unique path for plot to be saved to using date, time and id
    let name: String = format!(
        "results/chart-{}-({}).png",
        time.format("%Y-%m-%d-%H-%M-%S"),
        id
    );

    // Create root structure for charts with a specified size, coordinate 
    // range and path and give it a white background
    let root = BitMapBackend::new(name.as_str(), (1920, 1080)).into_drawing_area();
    root.fill(&WHITE)?;

    // Get highest value for y axis
    let y_max = country
        .best_chromosome
        .iter()
        .max_by(|x, y| x.partial_cmp(y).unwrap())
        .wrap_err("Cannot find best chromosome in Simulation, Simulation data empty")?
        .cost;

    // Write caption for plot
    let caption = format!(
        "TSP in {}, using a population of {} with a tournament size of {}",
        country.country_data.name, country.population_size, country.tournament_size
    );

    // Create a chart for the graph to be drawn on
    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .caption(caption, ("sans-serif", 50).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..10000f32, 0f32..y_max as f32)?;

    // Add a mesh object to chart
    chart.configure_mesh().x_labels(5).y_labels(5).draw()?;

    // Create vector for x & y coordinates from country data
    let country_coords = country
        .best_chromosome
        .iter()
        .enumerate()
        .map(|(x, y)| (x as f32, y.cost as f32))
        .collect::<Vec<(f32, f32)>>();

    // Draw country data as a line graoh on chart
    chart.draw_series(LineSeries::new(country_coords, &RED))?;

    // Take root and present all charts, then outut final plot
    root.present()?;

    // Return OK if Function runs without error
    Ok(())
}

fn main() -> Result<()> {
    // Setup color_eyre so errors output nicely
    color_eyre::install()?;

    // Create varible of type CLI and parse in info from command line
    let cli = Cli::parse();

    // Create object to manage multiple progress bars
    let multi_bar = MultiProgress::new();

    // Define progress bars style
    let bar_style = ProgressStyle::with_template(
        "[{elapsed_precise}] [{wide_bar:.cyan/blue}] [{percent}%] ({eta}) {msg}",
    )?
    // Create custom Key to show eta for the task
    .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
        write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
    })
    // Set characters to be used for Progress bar
    .progress_chars("#>-");

    // Define progress bar for Brazil
    let brazil_bar = multi_bar.add(ProgressBar::new(crate::NUMBER_OF_GENERATIONS as u64));
    // Set style for Brazils progress bar
    brazil_bar.set_style(bar_style.clone());

    // Define progress bar for Burma
    let burma_bar = multi_bar.add(ProgressBar::new(crate::NUMBER_OF_GENERATIONS as u64));
    // Set style for Brazils progress bar
    burma_bar.set_style(bar_style);

    // Spawn thread to run simulation for Brazil
    let brazil_thread = thread::spawn(move || {
        // Set brazil variable to Brazils data if there are no errors
        let Ok(brazil) = Country::new(true) else {
            // If there is an error, quit program with message
            panic!("Error: Cannot create Country")
        };

        // Create a Simulation type for Bazil
        let brazil_simulation = Simulation::new(
            brazil,
            cli.crossover_operator,
            cli.mutation_operator,
            cli.population_size,
            cli.tournament_size,
        );

        // Pattern Mmtching on errors
        match brazil_simulation {

            // If there are no errors, run the simulation and return the data
            Ok(mut simulation) => {
                simulation.run(brazil_bar).unwrap();
                simulation
            }

            // If there is an error, quit the program and display the error
            Err(report) => {
                panic!("{}", report)
            }
        }
    });

    // Spawn thread to run simulation for Burma
    let burma_thread = thread::spawn(move || {
        // Set burma variable to Burmas data if there are no errors
        let Ok(burma) = Country::new(false) else {

            // If there is an error, quit program with message
            panic!("Error: Cannot create Country")
        };

        // Create a Simulation type for Burma
        let burma_simulation = Simulation::new(
            burma,
            cli.crossover_operator,
            cli.mutation_operator,
            cli.population_size,
            cli.tournament_size,
        );

        // Pattern Mmtching on errors
        match burma_simulation {

            // If there are no errors, run the simulation and return the data
            Ok(mut simulation) => {
                simulation.run(burma_bar).unwrap();
                simulation
            }

            // If there is an error, quit the program and display the error
            Err(report) => {
                panic!("{}", report)
            }
        }
    });

    // Return simulation data from threads
    let finished_brazil = brazil_thread.join().unwrap();
    let finished_burma = burma_thread.join().unwrap();

    // Use simulation data to create graphs
    plot(&finished_brazil, 1)?;
    plot(&finished_burma, 2)?;

    println!(
        "The best Chromosome in Brazil {:?}",
        finished_brazil.population.best_chromosome.cost
    );
    println!(
        "The best Chromosome in Burma {:?}",
        finished_burma.population.best_chromosome.cost
    );

    // Return OK if Function runs without error
    Ok(())
}
