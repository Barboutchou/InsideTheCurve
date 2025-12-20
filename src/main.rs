// DOCUMENT CREATED BY SAMUEL GODLEWSKI LOYER

mod api;

use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT };
use reqwest::Client;

use std::time::Duration;
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
/// year-month-day
pub enum TimeSelector {
    TimePeriod { start: String, end: String },
    Dynamic { periods: String  },
}
impl TimeSelector {
    pub fn time_matching(self, mut params: Vec<(String, String)>) -> Vec<(String, String)> {
        match self {
            TimeSelector::TimePeriod { start, end } => {
                params.push(("startPeriod".to_string(), start));
                params.push(("endPeriod".into(), end));
            }
            TimeSelector::Dynamic { periods } => {
                params.push(("lastNObservations".into(), periods));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InstrumentType {
    Gbon, // Government bonds
    Tbil, // Treasury bills
}
impl InstrumentType {
    pub fn as_str(&self) -> &'static str {
        match self {
            InstrumentType::Gbon => "GBON",
            InstrumentType::Tbil => "TBIL",
        }
    }
    pub const ALL: &'static [InstrumentType] =
        &[InstrumentType::Gbon, InstrumentType::Tbil];
}

pub struct InstrumentSelection(&'static [InstrumentType]);
impl InstrumentSelection {
    pub const ALL: Self = Self(InstrumentType::ALL);
    pub const GBON: Self = Self(&[InstrumentType::Gbon]);
    pub const TBIL: Self = Self(&[InstrumentType::Tbil]);

    pub fn iter(&self) -> impl Iterator<Item = &InstrumentType> {
        self.0.iter()
    }
}

impl From<&'static [InstrumentType]> for InstrumentSelection {
    fn from(slice: &'static [InstrumentType]) -> Self {
        Self(slice)
    }
}



// Request Structure
pub struct NorgesBankClientBuilder {
    base_url: String,
    timeout: Duration,
}
impl NorgesBankClientBuilder {
    pub fn build(self) -> Result<NorgesBankClient, reqwest::Error> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("MyRustClient/1.0"));

        let client = Client::builder()
            .default_headers(headers)
            .timeout(self.timeout)
            .build()?;

        Ok(NorgesBankClient {
            client,
            base_url: self.base_url, // MOVED here
        })
    }
}
pub struct NorgesBankClient {
    client: Client,
    base_url: String,
}
impl NorgesBankClient {
    pub fn new() -> Result<Self, reqwest::Error> {
        let client = NorgesBankClientBuilder {
            base_url: "https://data.norges-bank.no/api/data".to_string(),
            timeout: Duration::from_secs(10),
        }
            .build()?;
        Ok( client )
    }

    /// Fetches Norwegian government securities (NGS) - Prices and yields
    /// Returns JSON with all observations.
    pub async fn fetch_ngs_market_data<I>(
        &self,
        timeselector: TimeSelector,
        isins : I,
    ) -> Result<ApiResponse, reqwest::Error>
    where
        I: IntoIterator,
        I::Item: AsRef<str>
    {
        let isin_list: Vec<String> = isins.into_iter().map(|s : I::Item | s.as_ref().to_string()).collect();
        let isin_param = isin_list.join("+");
        let url = format!("{}/SEC/B.{}.", &self.base_url, isin_param);
        let mut params = vec![
            ("format".to_string(), "sdmx-json".to_string()),
            ("locale".to_string(), "en".to_string()),
        ];
        params   = timeselector.time_matching( params );
        let resp: ApiResponse = self.client
            .get(url)
            .query(&params)
            .send()
            .await?
            .json()
            .await?;

        Ok(resp)
    }

    /// Fetches (NGS) – Primary market
    /// Returns JSON with all observations
    pub async fn fetch_ngs_primary_market(
        &self,
        timeselector: TimeSelector,
        instrument_types: InstrumentSelection ,
    ) -> Result<ApiResponse, reqwest::Error>
    {
        let instrument_list: Vec<String> = instrument_types
            .iter()
            .map(|i| i.as_str().to_string())
            .collect();               // collect into Vec<String>
        let instrument_param = instrument_list.join("+");
        let url = format!("{}/GOVT_PRIMARY_MARKET/{}..B...", &self.base_url, instrument_param);
        let mut params = vec![
            ("format".to_string(), "sdmx-json".to_string()),
            ("locale".to_string(), "en".to_string()),
        ];
        params   = timeselector.time_matching( params );
        let resp: ApiResponse = self.client
            .get(url)
            .query(&params)
            .send()
            .await?
            .json()
            .await? ;

        Ok(resp)
    }
}

#[tokio::main]
async fn main() {
    let nb_client = NorgesBankClient::new().unwrap();
    //let ts = TimeSelector::TimePeriod {start : "2025-12-15".into() , end : "2025-12-17".into() };
    let ts = TimeSelector::Dynamic {
        periods: "10".into()
    };
    let isin = ["NO0010757925"];

    // let res = nb_client.fetch_ngs_market_data(ts, &isin).await.unwrap();

    let res = nb_client.fetch_ngs_primary_market(ts, InstrumentSelection::ALL  ).await.unwrap();

    println!("{:?}", res);
}


#[cfg(test)]
mod tests {
    use super::*;

    //assert_eq!(add(1.0, 3.0), 4.0);

}
