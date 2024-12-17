use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct IsConnected(pub bool);

impl From<bool> for IsConnected {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
