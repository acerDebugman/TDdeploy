
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct TableOptions {
    // TODO: support case insensitive identifier, including table name and column name.
    /// Whether identifier is case insensitive. Not work for now.
    ///
    /// Default is `false`, which means identifier is case sensitive.
    #[serde(skip, default)]
    pub identifier_case_insensitive: bool,
    /// Replace dot in table name with this string.
    ///
    /// For example, if `replace_dot_in_table_name` is set to `_`, then table name `custom.table` will be converted to `custom_table`.
    ///
    /// Without this, table name `custom.table` will cause error 0x2617: "The table name cannot contain '.'".
    ///
    /// Default is `_`.
    #[serde(default)]
    pub replace_dot_in_table_name: String,

    /// Written method for insert.
    /// Default is `auto`.
    ///
    /// - `auto`: auto detect written method.
    /// - `sql`: use sql insert.
    /// - `stmt`: use stmt insert.
    /// - `sml`: use sml insert.
    #[serde(default)]
    pub written_protocol: WrittenProtocol,

    /// Flat written method
    written_method: Option<WrittenMethod>,

    /// Concurrent limit
    written_concurrent: Option<usize>,

    workers_per_vgroup: Option<usize>,

    /// How to deal with null values.
    null_values: Option<NullValues>,

    pub minimum_timestamp: Option<DateTime<Utc>>,
    pub maximum_timestamp: Option<DateTime<Utc>>,

    /// How to process on abnormal.
    #[serde(default)]
    #[serde(flatten)]
    pub process_on_abnormal: ProcessOnAbnormal,
}

impl Default for TableOptions {
    fn default() -> Self {
        Self {
            identifier_case_insensitive: false,
            replace_dot_in_table_name: "_".to_string(),
            written_protocol: WrittenProtocol::default(),
            written_method: None,
            written_concurrent: None,
            workers_per_vgroup: None,
            null_values: None,
            minimum_timestamp: None,
            maximum_timestamp: None,
            process_on_abnormal: ProcessOnAbnormal::default(),
        }
    }
}
impl TableOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn written_method(&self) -> WrittenMethod {
        self.written_method.unwrap_or_else(|| {
            std::env::var("TAOSX_WRITTEN_METHOD")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(WrittenMethod::Concurrent)
        })
    }

    pub fn concurrent_limit(&self) -> usize {
        self.written_concurrent.unwrap_or_else(|| {
            std::env::var("TAOSX_WRITTEN_CONCURRENT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(
                    std::thread::available_parallelism()
                        .ok()
                        .map_or(4, |v| v.get()),
                )
        })
    }

    pub fn workers_per_vgroup(&self) -> usize {
        self.workers_per_vgroup.unwrap_or_else(|| {
            std::env::var("TAOSX_WORKERS_PER_VGROUP")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(4)
        })
    }

    pub fn null_values(&self) -> NullValues {
        self.null_values.unwrap_or_else(|| {
            std::env::var("TAOSX_NULL_VALUES")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or_default()
        })
    }

    pub fn canonical_table_name<'b>(&self, name: &'b str) -> Cow<'b, str> {
        let dot = name.contains('.');
        match (self.identifier_case_insensitive, dot) {
            (true, true) => Cow::Owned(
                name.to_lowercase()
                    .replace('.', &self.replace_dot_in_table_name),
            ),
            (true, false) => Cow::Owned(name.to_lowercase()),
            (false, true) => Cow::Owned(name.replace('.', &self.replace_dot_in_table_name)),
            (false, false) => Cow::Borrowed(name),
        }
    }
}
