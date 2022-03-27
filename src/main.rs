use std::time::SystemTime;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StructWithCustomDate {
    // DateTime supports Serde out of the box, but uses RFC3339 format. Provide
    // some custom logic to make it use our desired format.
    #[serde(with = "my_date_format")]
    //pub timestamp: DateTime<Utc>,
    pub timestamp: SystemTime,

    // Any other fields in the struct.
    pub bidder: String,
}

mod my_date_format {
    use std::time::SystemTime;
    use chrono::{Utc, TimeZone};
    use serde::{self, Deserialize, Serializer, Deserializer};

    //const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";
    const FORMAT: &'static str = "%m/%d/%Y %r";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(
        date: &SystemTime,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{:?}", date);
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<SystemTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        match Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom) {
            Ok(dt) => {
               // let dt: DateTime<Utc> = 
                let systime: SystemTime = dt.into();
                Ok(systime)
            },
            Err(e) => Err(e)
        }
    }
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
