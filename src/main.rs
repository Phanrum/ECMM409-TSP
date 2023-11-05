// Importing my programs modules
use tsp_coursework::{country::Country, simulation::Simulation};
// Importing thread and sync to allow for multithreading
use std::{thread, sync::mpsc};
// Import HashMap
use std::collections::HashMap;
// Here I am importing my external dependancies:
// Clap is used to make the command line interface
use clap::Parser;
// Indicatif is used to create progress bars for the terminal
// Import Write from standard library to output custom key
use indicatif::{MultiProgress, ProgressBar, ProgressState, ProgressStyle};
use std::fmt::Write;
// Colour_Eyre is used to neatly propagate errors
use color_eyre::Result;

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
    /// Number of Runs: Minumum 1, Default 1. Note that double this value will be the number of threads used
    #[arg(value_parser = clap::value_parser!(u32).range(1..), default_value_t = 1, short, long)]
    number_runs: u32,
}

/// Main function for this program
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

    // Get Countries data from the data directory
    let input_data = Country::new()?;

    // Create vector for Simulations 
    let mut output_data: Vec<Simulation> = Vec::with_capacity(input_data.capacity() * cli.number_runs as usize);

    // Create Multi-producer, single-consumer channel
    let (tx, rx) = mpsc::channel();

    // Create a vector to hold the thread handlers
    let mut threads = Vec::with_capacity(input_data.len() * cli.number_runs as usize);

    // Loop for number of runs specified
    for _ in 0..cli.number_runs {

        // Loop over each seperate file in the directory
        for country in &input_data {

            // Clone transmitter so the thread will have a unique one
            let thread_tx = tx.clone();

            // Clone the country data because only one thread can have access to a value at a time
            let country_data = (*country).clone();

            // Create a new progress bar for this operation and add styling
            let progress_bar = multi_bar.add(ProgressBar::new(crate::NUMBER_OF_GENERATIONS as u64));
            progress_bar.set_style(bar_style.clone());

            // Generate a Thread to build and run the simulation
            let thread = thread::spawn(move || -> Result<()> {

                // Create a Simulation type
                let mut simulation = Simulation::new(
                    country_data,
                    cli.crossover_operator,
                    cli.mutation_operator,
                    cli.population_size,
                    cli.tournament_size,
                )?;

                // Run the Simulation
                simulation.run(progress_bar)?;

                // Transmit the simulation back to main
                thread_tx.send(simulation)?;

                // Exit thread
                Ok(())
            });

            // Push the Thread Handler to the threads vector
            threads.push(thread)
        }
    }

    // The number of threads spawned is the number of files multiplied by the number of runs specified
    // Loop for this value and push the result of each one to the output_data vector
    for _ in 0..cli.number_runs * input_data.len() as u32 {
        output_data.push(rx.recv()?);
    }

    // Loop through the vector of thread handlers and close each thread
    for thread in threads {
        thread.join().expect("Threads panicked")?;
    }

    // Create a hashmap to store all the simulations by their names
    let mut ordered_data: HashMap<String, Vec<Simulation>> = HashMap::with_capacity(output_data.capacity());

    // Loop over each Simulation in output_data
    for sim in output_data {
        ordered_data
            // Get the entry of the key, where the key is the name out the country used
            .entry(sim.country_data.name.clone())
            // If that key doesnt exist yet, create it and set its entry to be an empty vector
            .or_default()
            // Push the Simulation into the entry
            .push(sim);
    }

    // For each Simulation in ordered_data create a plot for it
    ordered_data.retain(|key: &String, data: &mut Vec<Simulation>| {
        Simulation::plot(data, key.clone()).expect("Plotting of Simulation failed");
        true
    } );

    // End program
    Ok(())
}
