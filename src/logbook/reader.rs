use chrono::NaiveDate;
use regex::Regex;

// Year, Month, Date
type YMD = (i32, Option<u32>, Option<u32>);

// Implement me!
pub struct Reader;
impl Reader {
  // Implement me!
    pub fn run(options: Options) -> Result<(), String> {
        println!("Received args: {:?}", options);
        Ok(())
    }
}

#[derive(Debug)]
pub struct Options {
    after: Option<NaiveDate>,
    before: Option<NaiveDate>,
    on: Option<NaiveDate>,
}

impl Options {
    const DAY_PATTERN: &'static str = r"^\d{4}-\d{2}-\d{2}$";
    const YEAR_PATTERN: &'static str = r"^\d{4}";
    const MONTH_PATTERN: &'static str = r"^\d{4}-\d{2}";
    const REGEX_INIT_ERR: &'static str =
        "Error initializing date matchers. Please report this as it's a bug.";

    pub fn new(
        before_arg: Option<String>,
        after_arg: Option<String>,
        on_arg: Option<String>,
    ) -> Result<Self, String> {
        if on_arg.is_some() && (before_arg.is_some() || after_arg.is_some()) {
            Err("Cannot filter using before/after with 'on'".to_string())
        } else {
            let before = Self::validate_filter_date(
                &before_arg,
                DateCompareOperatorKind::Before
            )?;
            let after = Self::validate_filter_date(
                &after_arg,
                DateCompareOperatorKind::After
            )?;
            let on = Self::validate_filter_date(
                &on_arg,
                DateCompareOperatorKind::On
            )?;

            Ok(Self { after, before, on })
        }
    }

    fn validate_filter_date(
        date: &Option<String>,
        compare_kind: DateCompareOperatorKind
    ) -> Result<Option<chrono::NaiveDate>, String> {
        match date {
            Some(d) => Self::parsed_ymd(d).and_then(|ymd| {
                let (y, m, d) = ymd;
                let maybe_naive_date = match compare_kind {
                    DateCompareOperatorKind::After =>
                        NaiveDate::from_ymd_opt(y, m.unwrap_or(12), d.unwrap_or(31)),
                    DateCompareOperatorKind::Before =>
                        NaiveDate::from_ymd_opt(y, m.unwrap_or(1), d.unwrap_or(1)),
                    DateCompareOperatorKind::On =>
                        NaiveDate::from_ymd_opt(y, m.unwrap_or(1), d.unwrap_or(1)),
                };

                if maybe_naive_date.is_none() {
                    Err("Date provided is out of range".to_string())
                } else {
                    Ok(maybe_naive_date)
                }
            }),
            None => Ok(None),
        }
    }

    fn parsed_ymd(date: &String) -> Result<YMD, String> {
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
                1 => Ok((parts[0], None, None)),
                2 => Ok((parts[0], Some(parts[1] as u32), None)),
                3 => Ok((parts[0], Some(parts[1] as u32), Some(parts[2] as u32))),
                // Theoretically impossible at this point but....
                _ => Err("Too many or too little numbers in date pattern".to_string()),
            }
        } else {
            Err("Could not match date to known pattern".to_string())
        }
    }
}

enum DateCompareOperatorKind {
  On,
  After,
  Before
}