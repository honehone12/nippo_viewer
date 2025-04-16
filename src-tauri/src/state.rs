use chrono::{DateTime, FixedOffset};
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
    // pub(crate) updated_at: DateTime<FixedOffset>,
    // pub(crate) deleted_at: Option<DateTime<FixedOffset>>,

    pub(crate) name: String,
    // pub(crate) line_id: String,
    // pub(crate) car_number: String,

    // pub(crate) org_id: String
}

#[derive(Debug, Default)]
pub(crate) struct Users {
    pub(crate) org_id: String,
    pub(crate) users: Vec<User>
}

impl Users {
    pub(crate) fn has(&self, org_id: &str) -> bool {
        return &self.org_id == org_id && self.users.len() != 0;
    }
}

#[derive(Debug, Default)]
pub(crate) struct Query {
    pub(crate) user: String
}
