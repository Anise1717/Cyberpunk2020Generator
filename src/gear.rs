use crate::skills::Modifier;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Gear {
    name: String,
    description: String,
    modifier: Option<Modifier>,
    cost: isize,
}
