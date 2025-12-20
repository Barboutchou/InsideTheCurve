use std::collections::HashMap;
use serde::Deserialize;
use serde_json::{json, Value};

// API structure
#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub meta: Meta,
    pub data: Data,
}
#[derive(Debug, Deserialize)]
pub struct Meta {
    pub id: String,
    pub prepared: String,
    pub test: bool,
    #[serde(rename = "datasetId")]
    pub dataset_id: String,
    pub sender: Entity,
    pub receiver: Entity,
    #[serde(default)]
    pub links: Vec<Link>,
}
#[derive(Debug, Deserialize)]
pub struct Entity {
    pub id: String,
}
#[derive(Debug, Deserialize)]
pub struct Data {
    #[serde(rename = "dataSets")]
    pub data_sets: Vec<DataSet>,
    pub structure: Structure,
}
#[derive(Debug, Deserialize)]
pub struct DataSet {
    #[serde(default)]
    pub links: Vec<Link>,
    #[serde(rename = "reportingBegin")]
    pub reporting_begin: String,
    #[serde(rename = "reportingEnd")]
    pub reporting_end: String,
    pub action: String,
    pub series: HashMap<String, Series>,
}
#[derive(Debug, Deserialize)]
pub struct Series {
    pub attributes: Vec<Option<u32>>,
    pub observations: HashMap<String, Vec<String>>,
}
#[derive(Debug, Deserialize)]
pub struct Link {
    pub rel: String,
    #[serde(default)]
    pub urn: Option<String>,
    #[serde(default)]
    pub href: Option<String>,
    #[serde(default)]
    pub uri: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct Structure {
    #[serde(default)]
    pub links: Vec<Link>,
    pub name: String,
    #[serde(default)]
    pub names: HashMap<String, String>,
    pub description: String,
    #[serde(default)]
    pub descriptions: HashMap<String, String>,
    pub dimensions: Dimensions,
    pub attributes: Attributes,
}
#[derive(Debug, Deserialize)]
pub struct Dimensions {
    pub dataset: Vec<Component>,
    pub series: Vec<Component>,
    pub observation: Vec<Component>,
}
#[derive(Debug, Deserialize)]
pub struct Attributes {
    pub dataset: Vec<Component>,
    pub series: Vec<Component>,
    pub observation: Vec<Component>,
}
#[derive(Debug, Deserialize)]
pub struct ComponentValue {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub start: Option<String>,
    #[serde(default)]
    pub end: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct Component {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(rename = "keyPosition")]
    pub key_position: Option<u32>,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub relationship: Option<Relationship>,
    #[serde(default)]
    pub values: Vec<ComponentValue>,
}
#[derive(Debug, Deserialize)]
pub struct Relationship {
    pub dimensions: Vec<String>,
}

impl ApiResponse {
    fn get_dimension_value(&self, dimension_id: &str, index: usize) -> String {
        self.data.structure.dimensions.series
            .iter()
            .find(|d| d.id == dimension_id)
            .and_then(|d| d.values.get(index))
            .map(|v| v.id.clone())
            .unwrap_or_default()
    }

    fn get_dimension_name(&self, dimension_id: &str, index: usize) -> String {
        self.data.structure.dimensions.series
            .iter()
            .find(|d| d.id == dimension_id)
            .and_then(|d| d.values.get(index))
            .and_then(|v| v.name.clone())
            .unwrap_or_default()
    }
}
// Financials Structure

