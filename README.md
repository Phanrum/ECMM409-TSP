# ECMM409-TSP
A Rust program to solve the Travelling Salesman Problem. It uses a steady state evolutionary algorithm and assumes its given an XML file detailing the costs associated with travel between each city.

# Instalation instructions

This program requries rust installed to compile, however the executable file does not require rust

Please note, this program uses a plotter called [plotters](https://github.com/plotters-rs/plotters) which has certain dependencies.  

## Dependencies

### Ubuntu Linux
`cmake pkg-config libfreetype6-dev libfontconfig1-dev`

### Fedora Linux
`cmake fontconfig-devel`

### Windows/OSX
No dependancies required

## Compiling Instructions

To compile, in your terminal run the command:

`cargo build --release`

This `--release` flag is important as it makes the compiler optimise the program, allowing it to run significantly faster - with the trade off being slightly longer
compile times.

This will create a `target` directory with a `release` subdirectory, the binary will be located in this subdirectory called `tsp-coursework`

# Running instructions

The binary must be located in a directory containing a subdirectory called `data` which contains the xml files.

You can then run the help command with:

`./tsp-coursework -h`

or 

`./tsp-coursework --help`

which will display all the flags possible to change the behaviour of the program.

Simply running

`./TSP`

will use the defaults, which are:
- Population size of 50
- Tournament size of 5
- Single Swap Mutation
- Crossover with fix

but up to all of these can be changed.

## Examles

`./TSP -p 10000 -t 1000 -c 1 -m 2`

Program will run with:
- Population of 10,000
- Tournament Size of 1,000
- Ordered Crossover
- Multiple Swap Mutation

`./TSP -p 1000 -c 0`

---

Program will run with:
- Population of 1,000
- Default Tournament size of 5
- Crossover with fix
- Default Single Swap Mutation