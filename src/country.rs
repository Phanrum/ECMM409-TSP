use std::fs;
use serde::Deserialize;
use serde_xml_rs;

// This struct defines the datatype of an Edge, which is the cost to get to a city as a float
#[derive(Debug, Deserialize)]
pub struct Edge {
    pub cost: f64,
    #[serde(rename = "$value")]
    pub destination_city: u8,
}

// This struct defines the Vertex, which is a Vector containing all the edges of a specific city
#[derive(Debug, Deserialize)]
pub struct Vertex {
    #[serde(rename = "edge")]
    pub edges: Vec<Edge>,
}

// This struct defines the graph, which is a Vector of all the Vertexs
#[derive(Debug, Deserialize)]
pub struct Graph {
    pub vertex: Vec<Vertex>,
}

// This struct defines the root data structure containing all the information from the XML file
// Attributes are used to rename these fields during deserialization so they match those in the XML file
#[derive(Debug, Deserialize)]
#[serde(rename = "travellingSalesmanProblemInstance")]
#[serde(rename_all = "camelCase")]
pub struct Country {
    pub name: String,
    pub source: String,
    pub description: String,
    pub double_precision: f64,
    pub ignored_digits: i32,
    pub graph: Graph,
}

impl Country {
    // Function to create the root structure for each countries XML file
    // Due to there only being two files I have hardcoded Brazil as True and Burma as False so that no errors with intergers or strings can occur
    pub fn new(country: bool) -> Self {
        match country {
            true => {
                // Imports the XML file as a String 
                let src = fs::read_to_string("data/brazil58.xml").unwrap();
                // Convert String to &str and use serde_xml_rs to deserialize into my struct Country
                serde_xml_rs::from_str(src.as_str()).unwrap()
            }

            false => {
                // Imports the XML file as a String 
                let src = fs::read_to_string("data/burma14.xml").unwrap();
                // Convert String to &str and use serde_xml_rs to deserialize into my struct Country
                serde_xml_rs::from_str(src.as_str()).unwrap()
            }
        }
        // import xml file and read into Country with from_str
    }
}
