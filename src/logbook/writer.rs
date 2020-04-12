use chrono::Utc;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use std::io::prelude::*;

use crate::config;
use crate::entry;

pub struct Writer;

impl Writer {
    pub fn run(config: config::LogbookConf, options: Options)
        -> Result<(), String> {
        let date = entry::Date::new(&Utc::now(), &options.utc_offset);
        let logfile_path = Self::current_logfile(
            &date,
            config.logbook_dir.as_path()
        );
        let logfile = OpenOptions::new()
            .create(true)
            .append(true)
            .open(logfile_path)
            .map_err(|e| format!("{:?}", e))?;
        let message = options.message.ok_or("A message is required.")?;
        let entry = Self::create_entry(date, options.tags, message);

        writeln!(&logfile, "{}", entry.to_string().as_str())
            .map_err(|e| format!("{:?}", e))
    }

    fn create_entry(
        date: entry::Date,
        tags: Option<Vec<String>>,
        content: String
    ) -> entry::Entry {
        let header = entry::Header { entry_date: date };
        let entry_tags = entry::Tags::new(tags);

        entry::Entry {
            header: header,
            tags: entry_tags,
            content,
        }
    }

    fn current_logfile(date: &entry::Date, logbook_dir: &Path) -> PathBuf {
        let entry_date = date.to_filename_date();
        let dir = logbook_dir.to_str().unwrap();
        let mut logfile_path = PathBuf::new();
        logfile_path.push(dir);
        logfile_path.push(format!("{}.txt", entry_date));

        logfile_path
    }
}

pub struct Options {
    pub message: Option<String>,
    pub tags: Option<Vec<String>>,
    pub utc_offset: Option<f32>,
}

impl Options {
    pub fn new(
        message: Option<String>,
        tags: Option<Vec<String>>,
        offset: Option<f32>,
    ) -> Result<Self, String> {
        let utc_offset = Self::validate_offset(offset)?;

        Ok(Self { message, tags, utc_offset })
  }

    fn validate_offset(offset: Option<f32>) -> Result<Option<f32>, String> {
        match offset {
            Some(offset) => {
                if offset % 0.25 == 0.0 {
                    Ok(Some(offset))
                } else {
                    Err("UTC offset must be in increments of 0.25".to_string())
                }
            },
            None => Ok(None)
        }
    }
}