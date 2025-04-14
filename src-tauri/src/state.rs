use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub(crate) struct AdminState {
    org_id: String,
    tkn: String
}
