
// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// #[serde(untagged)]
// pub enum Parser {
//     Inner(transform::Parser),
//     WithSample {
//         parser: transform::Parser,
//         input: Option<Vec<serde_json::Value>>,
//     },
// }

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Parser {
    Inner(Value),
}
