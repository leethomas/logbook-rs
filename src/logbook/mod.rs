use confy;

use crate::config;
pub mod reader;
pub mod writer;

pub struct Logbook;

impl <'a> Logbook {
    pub fn app_config() -> Result<config::LogbookConf, String> {
        confy::load(crate::APP_NAME).map_err(|e| format!("{:?}", e))
    }
}

pub enum LogbookOperationKind {
    Write,
    Read
}
