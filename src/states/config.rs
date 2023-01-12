use yewdux::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(PartialEq, Serialize, Deserialize, Store, Clone)]
#[store(storage = "session")]
pub struct ConfigStore {
    pub name: String,
}

impl Default for ConfigStore {
  fn default() -> Self {
      Self {
        name: "yew app sample".to_string(),
      }
  }
}
