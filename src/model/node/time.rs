use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Time {
    pub localtime: i64,   // Seconds since 1970-01-01 00:00:00 (local time)
    pub time: i64,        // Seconds since 1970-01-01 00:00:00 UTC.
    pub timezone: String, // Time zone
}
