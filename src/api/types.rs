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
