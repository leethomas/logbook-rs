use chrono::{DateTime, Utc, FixedOffset, Local};
use itertools::Itertools;

pub struct Date {
  time: DateTime<FixedOffset>,
}

impl Date {
    pub fn new(gmt_offset_hours: Option<f32>) -> Date {
        let now = Utc::now();
        let converted_time = match gmt_offset_hours {
            None => {
                let local_tz = now.with_timezone(&Local);
                let local_offset = local_tz.offset();
                now.with_timezone(local_offset)
            },
            Some(offset_hours) => {
                let seconds = (offset_hours * 3600.0) as i32;
                let tz = FixedOffset::east(seconds);
                now.with_timezone(&tz)
            }
        };

        Date { time: converted_time }
    }

    pub fn to_filename_date(&self) -> String {
        self.time.format("%Y-%m-%d").to_string()
    }

    pub fn to_logbook_timestamp(&self) -> String {
        self.time.format("%I:%M:%S %p %z").to_string()
    }
}

pub struct Header {
    pub entry_date: Date,
}

impl Header {
    pub fn to_string(&self) -> String {
        format!("[{}]", self.entry_date.to_logbook_timestamp())
    }
}

pub struct Tags {
    tags: Vec<String>,
}

impl Tags {
    pub fn new(maybe_tags: Option<Vec<String>>) -> Self {
        let matched_tags = match maybe_tags {
            None => vec![],
            Some(tags) => tags
        };

        Tags { tags: matched_tags }
    }

    pub fn to_string(&self) -> String {
        self.tags.iter()
            .map(|t| format!("#{}", t))
            .join(", ")
    }

    pub fn is_empty(&self) -> bool {
        self.tags.is_empty()
    }
}

pub struct Entry {
    pub header: Header,
    pub tags: Tags,
    pub content: String,
}

impl Entry {
    pub fn to_string(&self) -> String {
        let everything = if self.tags.is_empty() {
            vec![self.header.to_string(), self.content.clone()]
        } else {
            vec![self.header.to_string(), self.tags.to_string(), self.content.clone()]
        };

        format!("{}\n", everything.join("\n"))
    }
}



