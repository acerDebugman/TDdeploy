use std::{borrow::Cow, path::{Path, PathBuf}};



pub const ENV_TAOSX_DATA_DIR: &str = "TAOSX_DATA_DIR";
pub const ENV_TAOSX_DATA_DIR_DEFAULT: &'static str = "/tmp/taosx";
#[inline]
pub fn get_data_dir() -> PathBuf {
    Path::new(
        std::env::var(ENV_TAOSX_DATA_DIR)
            .map(Cow::Owned)
            .unwrap_or(Cow::Borrowed(ENV_TAOSX_DATA_DIR_DEFAULT))
            .as_ref(),
    )
    .to_path_buf()
}