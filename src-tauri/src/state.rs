use chrono::{DateTime, FixedOffset, TimeZone, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub(crate) struct Admin {
    pub(crate) org_id: String,
    pub(crate) tkn: String
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub(crate) struct User {
    pub(crate) id: String,
    pub(crate) created_at: DateTime<FixedOffset>,
    // pub(crate) update_at: DateTime<FixedOffset>,
    // pub(crate) deleted_at: Option<DateTime<FixedOffset>>,

    pub(crate) name: String,
    pub(crate) line_id: String,
    pub(crate) car_number: String,

    pub(crate) org_id: String
}
