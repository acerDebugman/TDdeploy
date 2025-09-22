use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Parser {
    #[serde(default)]
    global: Arc<super::TableOptions>,
    parse: Option<ParserImpl>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    mutate: Vec<Mutate>,
    s_model: Option<STableModel>,
    model: Modeler,
}

impl Parser {
    pub fn global(&self) -> &TableOptions {
        &self.global
    }

    pub fn modeler(&self) -> &Modeler {
        &self.model
    }

    pub fn set_maximum_timestamp(&mut self, ts: DateTime<Utc>) {
        Arc::make_mut(&mut self.global).maximum_timestamp = Some(ts);
    }

    pub fn set_minimum_timestamp(&mut self, ts: DateTime<Utc>) {
        Arc::make_mut(&mut self.global).minimum_timestamp = Some(ts);
    }
}

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("Read parser from path {input} error: {error}")]
    IoError {
        input: String,
        error: std::io::Error,
    },
    #[error("Deserialize parser from string {input} error: {error}")]
    DeserializeError {
        input: String,
        error: serde_json::Error,
    },
}
impl FromStr for Parser {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(s) = s.strip_prefix('@') {
            let s = std::fs::read_to_string(s).map_err(|error| ParserError::IoError {
                input: s.to_string(),
                error,
            })?;
            return serde_json::from_str(&s).map_err(|error| ParserError::DeserializeError {
                input: s.to_string(),
                error,
            });
        }
        serde_json::from_str(s).map_err(|error| ParserError::DeserializeError {
            input: s.to_string(),
            error,
        })
    }
}

impl Parser {
    pub fn new(
        parse: Option<ParserImpl>,
        mutate: Vec<Mutate>,
        s_model: Option<STableModel>,
        model: Modeler,
    ) -> Self {
        Self {
            global: Arc::new(TableOptions::default()),
            parse,
            mutate,
            s_model,
            model,
        }
    }
}


#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
pub struct ParserImpl(LinkedHashMap<String, FieldParser>);

impl ParserImpl {
    pub fn new(map: LinkedHashMap<String, FieldParser>) -> Self {
        Self(map)
    }

    // pub fn list_plugins() -> LinkedHashMap<String, FieldParser> {
    //     let mut map = LinkedHashMap::new();
    //     map.insert("regex".to_string(), FieldParser::Regex(Regex::default()));
    //     map.insert("cast".to_string(), FieldParser::Cast(Cast::default()));
    //     map.insert("alias".to_string(), FieldParser::Alias { alias: "".to_string() });
    //     map.insert("split".to_string(), FieldParser::Split(Split::default()));
    //     map.insert("udt".to_string(), FieldParser::Udt(Udt::default()));
    //     map.insert("join".to_string(), FieldParser::Join(Join::default()));
    //     map.insert("json".to_string(), FieldParser::Json(Json::default()));
    //     map
    // }
}
impl std::ops::Deref for ParserImpl {
    type Target = LinkedHashMap<String, FieldParser>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
