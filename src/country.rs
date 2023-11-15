//! This module creates the structure [`Country`] and methods to import data from
//! an XML file and deserialize into a [`Country`] so that it can be used.

use std::{fs,slice};

use serde::Deserialize;
use serde_xml_rs;
use color_eyre::{eyre::WrapErr, Result};

/// This Struct defines the datatype of an Edge, which is the cost to get to a city as a float
#[derive(Clone, Debug, Deserialize)]
pub struct Edge {
    pub cost: f64,
    #[serde(rename = "$value")]
    pub destination_city: u32,
}

/// This Struct defines the Vertex, which is a Vector containing all the edges of a specific city
#[derive(Clone, Debug, Deserialize)]
pub struct Vertex {
    #[serde(rename = "edge")]
    pub edges: Vec<Edge>,
}

/// Implements Trait IntoIterator for Vertex so that it can be converted to an iterator - allowing for it to be looped through
impl<'a> IntoIterator for &'a Vertex {
    type Item = &'a Edge;
    type IntoIter = slice::Iter<'a, Edge>;

    fn into_iter(self) -> Self::IntoIter {
        self.edges.iter()
    }
}

/// This Struct defines the graph, which is a Vector of all the Vertexs
#[derive(Clone, Debug, Deserialize)]
pub struct Graph {
    pub vertex: Vec<Vertex>,
}

/// This Struct defines the root data structure containing all the information from the XML file
/// Attributes are used to rename these fields during deserialization so they match those in the XML file
#[derive(Clone, Debug, Deserialize)]
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

/// Implement methods on `Country`
impl Country {
    /// Function to create the root structure for each countries XML file
    /// that is found in the data directory
    pub fn new() -> Result<Vec<Self>> {
        // Create iterator over all files in data/ directory
        let directory = fs::read_dir("data/")?;
        // Create a vector of Countries
        let mut output: Vec<Self> = Vec::new();

        // Loop over all files in directory
        for file in  directory {
            // Imports the XML file as a String
            let src: String = fs::read_to_string(file?.path()).wrap_err("Failed to read XML file")?;
            // Convert String to &str and use serde_xml_rs to deserialize into the Struct Country
            let data: Self = serde_xml_rs::from_str(src.as_str()).wrap_err("Failed to deserialize XML data")?;
            // Push Country to the output vector
            output.push(data);
        }
        // Return data as the type Country
        Ok(output)
    }
}
