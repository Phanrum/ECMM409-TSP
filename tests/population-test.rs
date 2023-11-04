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
fn test_manual() {
    let burma_small: country::Country = serde_xml_rs::from_str(SRC).unwrap();

    let mut test_pop = population::Population::new(10, &burma_small.graph).unwrap();

    println!("This is the test pop before: {:?}", test_pop.population_data);
    println!("This is the test pop average before: {:?}", test_pop.average_population_cost);

    let parent_1 = test_pop.run_tournament(5);
    
    let parent_2 = test_pop.run_tournament(5);

    println!("parents selected are {:?} and {:?}", parent_1, parent_2);
    
    let (mut first_child, mut second_child) = parent_1.crossover(&parent_2, 0, &burma_small.graph).unwrap();

    println!("children selected are {:?} and {:?}", first_child, second_child);

    first_child.mutation(1, &burma_small.graph).unwrap();
    second_child.mutation(1, &burma_small.graph).unwrap();

    println!("children mutated are {:?} and {:?}", first_child, second_child);

    test_pop.replacement(first_child);

    test_pop.replacement(second_child);

    println!("This is the test pop after: {:?}", test_pop.population_data);
    println!("This is the test pop average after: {:?}", test_pop.average_population_cost);
}

#[test]
fn test_auto() {

    let burma_small: country::Country = serde_xml_rs::from_str(SRC).unwrap();

    let mut test_pop = population::Population::new(10, &burma_small.graph).unwrap();

    println!("This is the test pop average before: {:?}", test_pop.average_population_cost);

    test_pop.selection_and_replacement(5, 0, 1, &burma_small.graph).unwrap();

    println!("This is the test pop average after: {:?}", test_pop.average_population_cost);
}
