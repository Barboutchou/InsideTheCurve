mod client;
mod models;
mod types;

pub use client::NorgesBankClient;
pub use models::{ApiResponse, DataSet, Series, Meta, Data};
pub use types::{TimeSelector, InstrumentSelection, InstrumentType};


// Re-export public items so users can do: use my_crate::api::NorgesBankClient;
