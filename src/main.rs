// DOCUMENT CREATED BY SAMUEL GODLEWSKI LOYER

mod api;

use api::{NorgesBankClient, TimeSelector, InstrumentSelection};
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
