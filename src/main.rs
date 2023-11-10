// Importing some of my programs modules
use tsp_coursework::{
        country::Country, 
        interface::*, 
        simulation::Simulation, 
        NUMBER_OF_GENERATIONS
    };

// Importing some modules from the standard library
use std::{
    collections::HashMap,
    fmt::Write,
    sync::mpsc,
    thread, 
};

// Here I am importing my external dependancies:
// Clap is used to make the command line interface
use clap::Parser;
// Indicatif is used to create progress bars for the terminal
use indicatif::{
        MultiProgress, 
        ProgressBar, 
        ProgressState, 
        ProgressStyle
    };
// Colour_Eyre is used to neatly propagate errors
use color_eyre::Result;


/// Main function for this program
fn main() -> Result<()> {
    // Setup color_eyre so errors output nicely
    color_eyre::install()?;

    // Create varible of type CLI and parse in info from command line
    let cli = Cli::parse();

    // Compare given tournament size and population size
    match cli.tournament_size.cmp(&(cli.population_size as u32)) {
        // Do nothing if the user selects a tournament size lower than the population size
        std::cmp::Ordering::Less => (),
        // If the user selects a tournament size equal to the population size, warn them
        std::cmp::Ordering::Equal => {
            println!("Warning: Selected Tournament Size is equal to the population size");
        },
        // If the user selects a tournament size greater than the population size,
        // exit the program with an error message
        std::cmp::Ordering::Greater => {
            panic!("ERROR: Selected Tournament Size is greater than the population size")
        },
    }



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
    let input_data: Vec<Country> = Country::new()?;

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
            let progress_bar = multi_bar.add(ProgressBar::new(NUMBER_OF_GENERATIONS as u64));
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
        Simulation::plot(data, cli.plot_operator, cli.statistic_plotted, cli.number_runs, key.clone()).expect("Plotting of Simulation failed");
        true
    });

    // End program
    Ok(())
}
