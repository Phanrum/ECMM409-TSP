//! This module creates the structure [`Country`] and methods to import data from
//! an XML file and deserialize into a [`Country`] so that it can be used.

use core::slice;
use serde::Deserialize;
use serde_xml_rs;
use std::fs;
use color_eyre::{eyre::WrapErr, Result};

/// This struct defines the datatype of an Edge, which is the cost to get to a city as a float
#[derive(Debug, Deserialize)]
pub struct Edge {
    pub cost: f64,
    #[serde(rename = "$value")]
    pub destination_city: u32,
}

/// This struct defines the Vertex, which is a Vector containing all the edges of a specific city
#[derive(Debug, Deserialize)]
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

/// This struct defines the graph, which is a Vector of all the Vertexs
#[derive(Debug, Deserialize)]
pub struct Graph {
    pub vertex: Vec<Vertex>,
}

/// This struct defines the root data structure containing all the information from the XML file
/// Attributes are used to rename these fields during deserialization so they match those in the XML file
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

/// Implement methods on `Country`
impl Country {
    /// Function to create the root structure for each countries XML file
    /// Due to there only being two files I have hardcoded Brazil as True and Burma as False so that no errors with intergers or strings can occur
    pub fn new(country: bool) -> Result<Self> {
        match country {
            true => {
                // Imports the XML file as a String
                let src = fs::read_to_string("data/brazil58.xml").wrap_err("Failed to read XML file")?;
                // Convert String to &str and use serde_xml_rs to deserialize into my struct Country
                let data: Self = serde_xml_rs::from_str(src.as_str()).wrap_err("Failed to deserialize XML data")?;
                // Return data as the type Country
                Ok(data)
            }

            false => {
                // Imports the XML file as a String
                let src = fs::read_to_string("data/burma14.xml").wrap_err("Failed to read XML file")?;
                // Convert String to &str and use serde_xml_rs to deserialize into my struct Country
                let data: Self = serde_xml_rs::from_str(src.as_str()).wrap_err("Failed to deserialize XML data")?;
                // Return data as the type Country
                Ok(data)
            }
        }
    }
}
