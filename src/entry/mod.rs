use chrono::{DateTime, Utc, FixedOffset, Local};
use itertools::Itertools;

pub struct Date {
  time: DateTime<FixedOffset>,
}

impl Date {
    pub fn new(now: &DateTime<Utc>, gmt_offset_hours: &Option<f32>) -> Date {
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

#[cfg(test)]
mod test {
    use chrono::{NaiveDateTime, NaiveDate};
    use super::*;

    // Helper functions
    fn get_epoch_datetime() -> DateTime<Utc> {
        let naive_epoch = NaiveDateTime::from_timestamp(0, 0);
        DateTime::<Utc>::from_utc(naive_epoch, Utc)
    }

    fn get_epoch_entry_date() -> Date {
        Date::new(&get_epoch_datetime(), &Some(0.0))
    }

    // Tags

    #[test]
    fn tags_is_empty_is_true_when_there_are_no_tags() {
        let result = Tags::new(None).is_empty();
        assert_eq!(result, true);
    }

    #[test]
    fn tags_is_empty_is_true_when_there_is_an_empty_list() {
        let result = Tags::new(Some(vec![])).is_empty();
        assert_eq!(result, true);
    }

    #[test]
    fn tags_is_empty_is_false_when_there_are_tags() {
        let result = Tags::new(Some(vec![
            "pizza".to_string(),
            "planet".to_string(),
        ])).is_empty();

        assert_eq!(result, false);
    }

    #[test]
    fn tags_to_string_prepends_with_hashes() {
        let result = Tags::new(Some(vec![
            "pizza".to_string(),
            "planet".to_string(),
        ])).to_string();

        assert_eq!(result, "#pizza, #planet".to_string());
    }

    // Header

    #[test]
    fn header_to_string_formats_correctly() {
        let result = Header {
            entry_date: Date::new(&get_epoch_datetime(), &Some(0.0)),
        }.to_string();
        assert_eq!(result, "[12:00:00 AM +0000]")
    }

    // Date

    #[test]
    fn date_new_contructs_a_date_with_correct_offset() {
        let now = get_epoch_datetime();
        let neutral_gmt_offset = Some(0.0);
        let date = Date::new(&now, &neutral_gmt_offset);
        let utc_offset = FixedOffset::east(0);

        assert_eq!(date.time.offset(), &utc_offset);
    }

    #[test]
    fn date_new_constructs_a_date_with_local_offset() {
        let now = get_epoch_datetime();
        let time_with_lcoal_tz = now.with_timezone(&Local);
        let local_offset = time_with_lcoal_tz.offset();
        let date = Date::new(&now, &None);

        assert_eq!(date.time.offset(), local_offset);
    }

    #[test]
    fn date_to_filename_date_formats_correctly() {
        let naive_dt = NaiveDate::from_ymd(1970, 1, 2)
            .and_hms(0, 0, 0);
        let dt_utc = DateTime::<Utc>::from_utc(naive_dt, Utc);
        let date = Date::new(&dt_utc, &Some(0.0));
        let result = date.to_filename_date();

        assert_eq!(result, "1970-01-02");
    }

    #[test]
    fn date_to_logbook_timestamp_formats_correctly() {
        let naive_dt = NaiveDate::from_ymd(1970, 1, 2)
            .and_hms(0, 0, 0);
        let dt_utc = DateTime::<Utc>::from_utc(naive_dt, Utc);
        let date = Date::new(&dt_utc, &Some(0.0));
        let result = date.to_logbook_timestamp();

        assert_eq!(result, "12:00:00 AM +0000");
    }

    // Entry

    #[test]
    fn entry_to_string_formats_with_no_tags() {
        let date = get_epoch_entry_date();
        let header = Header { entry_date: date };
        let tags = Tags::new(None);
        let result = Entry {
            header: header,
            tags: tags,
            content: "I haven't had pizza in 30 days".to_string(),
        }.to_string();

        let expected = format!("{}\n", vec![
            "[12:00:00 AM +0000]",
            "I haven't had pizza in 30 days"
        ].join("\n"));

        assert_eq!(result, expected);
    }

    #[test]
    fn entry_to_string_formats_correctly_with_tags() {
        let date = get_epoch_entry_date();
        let header = Header { entry_date: date };
        let tags = Tags::new(Some(vec![
            "pizza".to_string(),
            "planet".to_string(),
        ]));
        let result = Entry {
            header: header,
            tags: tags,
            content: "I haven't had pizza in 30 days".to_string(),
        }.to_string();

        let expected = format!("{}\n", vec![
            "[12:00:00 AM +0000]",
            "#pizza, #planet",
            "I haven't had pizza in 30 days"
        ].join("\n"));

        assert_eq!(result, expected);
    }
}
