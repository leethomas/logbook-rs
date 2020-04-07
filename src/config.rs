use dirs;
use serde::{Serialize, Deserialize};
use std::{env, io};
use text_io::read;
use std::io::{Write};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct LogbookConf {
    pub logbook_dir: std::path::PathBuf,
}

impl std::default::Default for LogbookConf {
    // This is never called explicitly, only by confy when
    // an existing config is not found on the machine. The
    // config location setup code is in here but it should
    // probably be extracted soon.
    fn default() -> Self {
        print!("Where will your logbook be? ");
        let _ = io::stdout().flush();
        let user_selected_dir: String = read!("{}\n");

        let dir: PathBuf = if user_selected_dir.is_empty() {
            let maybe_logbook_dir = dirs::home_dir()
                .or_else(|| env::current_dir().ok());
            let mut fallback_dir = match maybe_logbook_dir {
                None => panic!("Could not find directory to write logbook!"),
                Some(dir) => dir
            };

            println!("No logbook dir chosen, using {}", fallback_dir.to_str().unwrap());
            fallback_dir.push("Logbook");
            fallback_dir
        } else {
            let mut user_path = PathBuf::new();
            user_path.push(user_selected_dir);
            user_path
        };

        Self { logbook_dir: dir }
    }
}