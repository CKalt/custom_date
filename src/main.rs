use std::time::SystemTime;
use serde::{Serialize, Deserialize};

mod systime_fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct StructWithCustomDate {
    // DateTime supports Serde out of the box, but uses RFC3339 format. Provide
    // some custom logic to make it use our desired format.
    #[serde(with = "systime_fmt")]
    //pub timestamp: DateTime<Utc>,
    pub timestamp: SystemTime,

    // Any other fields in the struct.
    pub bidder: String,
}

fn main() -> Result<(), csv::Error> {
    let csv = r#""timestamp","bidder"
"2/15/2022 10:01:27 PM","Skrillex"
"#;

    let mut reader = csv::Reader::from_reader(csv.as_bytes());

    for record in reader.deserialize() {
        let record: StructWithCustomDate = record?;
        println!(
            "timestamp={:?}, bidder={}",
            record.timestamp,
            record.bidder,
        );
    }

    Ok(())
}
