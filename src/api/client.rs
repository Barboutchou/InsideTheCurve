use std::time::Duration;
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};

use super::models::ApiResponse;
use super::types::{TimeSelector, InstrumentSelection};

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