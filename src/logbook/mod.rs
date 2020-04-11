use chrono::{NaiveDate, Utc};
use confy;
use regex::Regex;
use itertools::Itertools;
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
    pub message: Option<String>,
    pub tags: Option<Vec<String>>,
    pub utc_offset: Option<f32>,
    pub readCmdOpts: ReadCmdOptions,
}

impl Options {
    pub fn new(
        message: Option<String>,
        tags: Option<Vec<String>>,
        utc_offset: Option<f32>,
        before_after_filters: (Option<String>, Option<String>),
    ) -> Result<Self, String> {
        let offset = Self::validate_offset(utc_offset)?;
        let (before, after) = before_after_filters;
        let readCmdOpts = ReadCmdOptions::new(before, after)?;

        Ok(Self {
            message,
            tags,
            utc_offset: offset,
            readCmdOpts,
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

struct ReadCmdOptions {
    after: Option<NaiveDate>,
    before: Option<NaiveDate>,
}

impl ReadCmdOptions {
    const DAY_PATTERN: &'static str = r"^\d{4}-\d{2}-\d{2}$";
    const YEAR_PATTERN: &'static str = r"^\d{4}";
    const MONTH_PATTERN: &'static str = r"^\d{4}-\d{2}";
    const REGEX_INIT_ERR: &'static str =
        "Error initializing date matchers. Please report this as it's a bug.";

    pub fn new(before_arg: Option<String>, after_arg: Option<String>) -> Result<Self, String> {
        let before = Self::validate_filter_date(&before_arg)?;
        let after = Self::validate_filter_date(&after_arg)?;

        Ok(Self { after, before })
    }

    fn validate_filter_date(date: &Option<String>) -> Result<Option<chrono::NaiveDate>, String> {
        match date {
            Some(d) => Self::parsed_date(d).map(Some),
            None => Ok(None)
        }
    }

    fn parsed_date(date: &String) -> Result<NaiveDate, String> {
        let day_pattern = Regex::new(Self::DAY_PATTERN)
            .map_err(|_| Self::REGEX_INIT_ERR.to_string())?;
        let year_pattern = Regex::new(Self::YEAR_PATTERN)
            .map_err(|_| Self::REGEX_INIT_ERR)?;
        let month_pattern = Regex::new(Self::MONTH_PATTERN)
            .map_err(|_| Self::REGEX_INIT_ERR)?;
        
        if day_pattern.is_match(date)
            || year_pattern.is_match(date) || month_pattern.is_match(date) {
            let parts: Vec<i32> = date.split("-")
                .collect::<Vec<&str>>()
                .iter()
                .map(|part| part.parse::<i32>().unwrap())
                .collect();

            match parts.len() {
                1 => Ok(NaiveDate::from_ymd(parts[0], 1, 1)),
                2 => Ok(NaiveDate::from_ymd(parts[0], parts[1] as u32, 1)),
                3 => Ok(NaiveDate::from_ymd(parts[0], parts[1] as u32, parts[2] as u32)),
                // Theoretically impossible at this point but....
                _ => Err("Date had more numbers than expected".to_string())
            }
        } else {
            Err("Could not match date to known pattern".to_string())
        }
    }
}
