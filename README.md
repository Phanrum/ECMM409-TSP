# ECMM409-TSP
A Rust program to solve the Travelling Salesman Problem. It uses a steady state evolutionary algorithm and assumes its given an XML file detailing the costs associated with travel between each city.

# Installation instructions

This program requires rust installed to compile, however the executable file does not require rust to run.
As I do not have a MAC, I was unable to cross-compile this program to OSX. IF you do not have a windows or linux machine,
please manually complile the program via the instructions below.

## Dependencies

Please note, this program uses a plotter called [plotters](https://github.com/plotters-rs/plotters) which has certain dependencies listed below.  

### Ubuntu Linux
`cmake pkg-config libfreetype6-dev libfontconfig1-dev`

### Fedora Linux
`cmake fontconfig-devel`

### Windows/OSX
No dependencies required


## Compiling Instructions

To compile, in your terminal run the command:

`cargo build --release`

This `--release` flag is important as it makes the compiler optimise the program, allowing it to run significantly faster - with the trade off being slightly longer
compile times.

This will create a `target` directory with a `release` sub-directory, the binary will be located in this sub-directory called `tsp-coursework`



# Running instructions

The binary must be located in a directory containing a sub-directory called `data` which contains the XML files.

You can then run the help command with:

`./tsp-coursework -h`

or 

`./tsp-coursework --help`

which will display all the flags possible to change the behaviour of the program.

Simply running

`./tsp-coursework`

will use the defaults as described below, create a `results` folder and output any graphs into that.


## Flags explained

The program has defaults for all aspects, however all of these can be changed with the flags below.

### `-h`

This will display a condensed help page for the program

### `--help`

This will display a more extensive help page for the program

### `-c` or `--crossover-operator`

**This flag has the options:**

#### `fix` or `F`
**This is the programs default flag.**

The program will use a crossover with fix to create child chromosomes.

#### `ordered` or `O`

The program will use an ordered crossover to create child chromosomes.

### `-m` or `--mutation-operator`

**This flag has the options:**

#### `inversion` or `I`

The program will use inversion mutation to mutate chromosomes.

#### `single` or `S`
**This is the programs default flag.**

The program will use single swap mutation to mutate chromosomes.

#### `multiple` or `M`

The program will use multiple swap mutation to mutate chromosomes.

### `-p` or `--population-size`

**Default population size is `50`**

**Minimum population size is `10`**

This selects the size of the population of chromosomes for the program to use.
This flag expects a number equal to or greater than 10 to be supplied.

### `-t` or `--tournament-size`

**Default tournament size is `5`**

**Minimum tournament size is `1`**

This selects the size of the tournament used.
This flag expects a number equal to or greater than 1 and less than or equal to the tournament size to be supplied.

### `-n` or `--number-runs`

**Default and Minimum is `1`**

This selects how many simulations of each dataset to run simultaneously.
This flag expects a number equal to or greater than 1 to be supplied.

### `-o` or `--output-type`

**This flag has the options:**

#### `average` or `A`
**This is the programs default flag.**

If multiple simulations have been run at once, this flag will average all their results together.

#### `display-all` or `D`

If multiple simulations have been run at once, this flag will output each result as a seperate line on the graph.

#### `best` or `B`

Will only output the simulation that returns the best cost at the end of the program.

#### `worst` or `W`

Will only output the simulation that returns the worst cost at the end of the program.

#### `range` or `R`

Will output the best simulation, worst simulation and average simulation.

### `s` or `statistic-plotted`
**This flag has the options:**

#### `average` or `A`
**This is the programs default flag.**

Will plot the average cost of each generation in a simulation.

#### `best` or `B`

Will plot the best cost found in each generation in a simulation.

#### `worst` or `W`

Will plot the worst cost found in each generation in a simulation.


# Documentation

This code is extensively commented throughout, however if you wish to read through the library for this code more comfortably then `Cargo` helpfully allows that.
Unfortunately you wont be able to use command to read through main.rs, as this is meant for libraries not applications.

Running:

`cargo doc --no-deps --open`

Will open a page on your browsers containing all my libraries functions with their comments rendered in markdown. You can also see the underlying source code of 
any function, impl block, struct etc with the *source* button located to their right.

If you wish to read through the documentation including the dependencies I brought in for this project, you can run

`cargo doc --open`

My dependencies were:
- chrono
- clap
- color-eyre
- indicatif
- plotters
- rand
- serde
- serde-xml-rs

All the other pages are the dependencies of my dependencies

