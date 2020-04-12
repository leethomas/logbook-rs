use chrono::NaiveDate;
use regex::Regex;

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
}

impl Options {
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