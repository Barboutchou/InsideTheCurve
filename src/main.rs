// DOCUMENT CREATED BY SAMUEL GODLEWSKI LOYER

use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT };
use reqwest::Client;

use std::time::Duration;
use std::collections::HashMap;

use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Debug, Deserialize)]
pub struct SDMXResponse {
    pub meta: Meta,
    pub data: Data,
}

#[derive(Debug, Deserialize)]
pub struct Meta {
    pub id: String,
    //pub prepared: String,
    //pub test: bool,
    //pub datasetId: String,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    pub dataSets: Vec<DataSet>,
    pub structure : StructureSet
}
#[derive(Debug, Deserialize)]
pub struct StructureSet {
    pub description: String,
    //pub observation: Vec<ObservationDimension>,
}
#[derive(Debug, Deserialize)]
pub struct ObservationDimension {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub keyPosition: Option<u32>,
    pub role: Option<String>,
    pub values: Vec<ObservationValue>,
}
#[derive(Debug, Deserialize)]
pub struct ObservationValue {
    pub id: String,
    pub name: String,
    pub start: Option<String>,
    pub end: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DataSet {
    pub series: HashMap<String, Series>,
    //pub reportingBegin: String,
    //pub reportingEnd: String,
    //pub action: String,
    // links ignored for now
}

#[derive(Debug, Deserialize)]
pub struct Series {
    //pub attributes: Vec<Option<serde_json::Value>>,
    pub observations: HashMap<String, Observation>,
}

#[derive(Debug, Deserialize)]
pub struct Observation(pub Vec<String>); // ["99.5210"]

/// year-month-day
pub enum TimeSelector {
    TimePeriod { start: String, end: String },
    Dynamic { periods: String  },
}
pub struct NorgesBankClient {
    client: Client,
    base_url: String,
}
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
impl NorgesBankClient {
    pub fn new() -> Result<Self, reqwest::Error> {
        let client = NorgesBankClientBuilder {
            base_url: "https://data.norges-bank.no/api/data".to_string(),
            timeout: Duration::from_secs(10),
        }
            .build()?;
        Ok( client )
    }
    /// Fetches Norwegian government securities (NGS) from Norges Bank API.
    /// Returns JSON with all observations.
    pub async fn get_ngs_sdmx<I>(
        &self,
        timeselector: TimeSelector,
        isins: I,
    ) -> Result<SDMXResponse, reqwest::Error>
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
        match timeselector {
            TimeSelector::TimePeriod { start, end } => {
                params.push(("startPeriod".into(), start));
                params.push(("endPeriod".into(), end));
            }
            TimeSelector::Dynamic { periods } => {
                params.push(("lastNObservations".into(), periods));
            }
        }
        let resp: SDMXResponse = self.client
            .get(url)
            .query(&params)
            .send()
            .await?
            .json()
            .await?;

        Ok(resp)
    }
}

#[tokio::main]
async fn main() {
    let nb_client = NorgesBankClient::new().unwrap();
    let ts = TimeSelector::TimePeriod {start : "2025-12-15".into() , end : "2025-12-17".into() };
    let isin = "NO0010757925";

    let x = nb_client.get_ngs_sdmx(ts, &[isin]).await.unwrap();

    for dataset in x.data.dataSets {
        // First, get the series we care about
        let prices_seriees = dataset.series.get("0:0:0").unwrap();
        let prices_series_bis = dataset.series.get("0:0:3").unwrap();
        println!("ASK: {:?}", prices_seriees );
        println!("BID: {:?}", prices_series_bis);

    }

    println!("{:?}", x.data.structure.description );

}


