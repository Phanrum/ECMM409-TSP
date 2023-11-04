use tsp_coursework::*;

const SRC: &str = r#"<travellingSalesmanProblemInstance>
<name>burma14</name>
<source>TSPLIB</source>
<description>14-Staedte in Burma (Zaw Win)</description>
<doublePrecision>15</doublePrecision>
<ignoredDigits>5</ignoredDigits>
<graph>
<vertex>
    <edge cost="1.530000000000000e+02">1</edge>
    <edge cost="5.100000000000000e+02">2</edge>
    <edge cost="7.060000000000000e+02">3</edge>
</vertex>
<vertex>
    <edge cost="1.530000000000000e+02">0</edge>
    <edge cost="4.220000000000000e+02">2</edge>
    <edge cost="6.640000000000000e+02">3</edge>
</vertex>
<vertex>
    <edge cost="5.100000000000000e+02">0</edge>
    <edge cost="4.220000000000000e+02">1</edge>
    <edge cost="2.890000000000000e+02">3</edge>
</vertex>
<vertex>
    <edge cost="7.060000000000000e+02">0</edge>
    <edge cost="6.640000000000000e+02">1</edge>
    <edge cost="2.890000000000000e+02">2</edge>
</vertex>
</graph>
</travellingSalesmanProblemInstance>"#;

#[test]
fn check_fitness(){

    let burma_small: country::Country = serde_xml_rs::from_str(SRC).unwrap();
    let route = vec![2, 0, 1, 3];
    let cost = 289.0 + 510.0 + 153.0 + 664.0;
    let test_chromosome = chromosome::Chromosome::new(route, cost);

    assert_eq!(cost, chromosome::Chromosome::fitness(&test_chromosome.route, &burma_small.graph), 
        "my cost calculated {} and functions cost {}", 
        cost, chromosome::Chromosome::fitness(&test_chromosome.route, &burma_small.graph));
}

#[test]
fn check_crossover() {

    // crossover point 3
    // p1 [2, 1, 0, 3]
    // p2 [0, 2, 3, 1]

    // c1 [2, 2, 0, 1]
    // c2 [0, 2, 0, 3]

    let burma_small: country::Country = serde_xml_rs::from_str(SRC).unwrap();
    let parent_one = chromosome::Chromosome::generation(&burma_small.graph);
    let parent_two = chromosome::Chromosome::generation(&burma_small.graph);

    let (child_one, child_two) = parent_one.crossover(&parent_two, 0, &burma_small.graph);

    println!("first child: {:?} second child: {:?} first parent: {:?} second parent: {:?}", child_one, child_two, parent_one, parent_two)
}

#[test]
fn check_mutation() {
    let burma_small: country::Country = serde_xml_rs::from_str(SRC).unwrap();
    let route = vec![0,1,2,3,4,5];
    let fitness = chromosome::Chromosome::fitness(&route, &burma_small.graph);

    let mut chromo = chromosome::Chromosome::new(route, fitness);

    chromo.mutation(1, &burma_small.graph);

    todo!()
}

#[test]
fn check_ordered_crossover() {
    todo!()
}