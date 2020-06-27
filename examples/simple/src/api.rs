use serde::{Deserialize, Serialize};

#[derive(Hash, Debug, Serialize, Deserialize)]
pub struct TestResponse {
    ok: bool,
}
