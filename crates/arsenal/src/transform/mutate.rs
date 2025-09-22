use serde::{Deserialize, Serialize};

use super::{filter::Filter, map::Map, parse::ParserImpl, TransformExt};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Mutate {
    Extract(ParserImpl),
    Filter(Filter),
    Map(Map),
}

impl TransformExt for Mutate {
    #[instrument(skip_all)]
    fn transform_record_batch(
        &self,
        records: &arrow::record_batch::RecordBatch,
    ) -> Result<arrow::record_batch::RecordBatch, super::Error> {
        match self {
            Mutate::Extract(parser) => parser.transform_record_batch(records),
            Mutate::Filter(filter) => filter.transform_record_batch(records),
            Mutate::Map(map) => map.transform_record_batch(records),
        }
    }
}