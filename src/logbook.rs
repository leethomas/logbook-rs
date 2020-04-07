use chrono::Utc;
use confy;
use std::path::{Path, PathBuf};
use crate::config;
use crate::entry;
use std::fs::OpenOptions;
use std::io::prelude::*;


pub struct Logbook;

impl <'a> Logbook {
    pub fn run(options: Options) -> Result<(), String> {
        let config: config::LogbookConf = confy::load(crate::APP_NAME)
            .map_err(|e| format!("{:?}", e))?;
        let date = entry::Date::new(&Utc::now(), &options.utc_offset);
        let logfile_path = Self::current_logfile(&date, config.logbook_dir.as_path());
        let entry = Self::create_entry(date, options);
        let logfile = OpenOptions::new()
            .create(true)
            .append(true)
            .open(logfile_path)
            .map_err(|e| format!("{:?}", e))?;

        writeln!(&logfile, "{}", entry.to_string().as_str())
            .map_err(|e| format!("{:?}", e))
    }

    pub fn current_logfile(date: &entry::Date, logbook_dir: &Path) -> PathBuf {
        let entry_date = date.to_filename_date();
        let dir = logbook_dir.to_str().unwrap();
        let mut logfile_path = PathBuf::new();
        logfile_path.push(dir);
        logfile_path.push(format!("{}.txt", entry_date));

        logfile_path
    }

    pub fn create_entry(date: entry::Date, options: Options) -> entry::Entry {
        let header = entry::Header { entry_date: date };
        let tags = entry::Tags::new(options.tags);

        entry::Entry {
            header: header,
            tags: tags,
            content: options.message,
        }
    }
}

pub struct Options {
    pub message: String,
    pub tags: Option<Vec<String>>,
    pub utc_offset: Option<f32>,
}

impl Options {
    pub fn new(
        message: String,
        tags: Option<Vec<String>>,
        utc_offset: Option<f32>
    ) -> Result<Self, String> {
        let offset = Self::validate_offset(utc_offset)?;

        Ok(Self {
            message: message,
            tags: tags,
            utc_offset: offset,
        })
    }

    fn validate_offset(offset: Option<f32>) ->
        Result<Option<f32>, String> {
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
