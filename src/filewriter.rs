use std::fs::{File, read_to_string};
use std::io::{BufWriter, Write};
use std::error::Error;
use std::str::FromStr;
use aho_corasick::AhoCorasick;
use serde::{Serialize, Deserialize};
use sqlite::ConnectionWithFullMutex;
use std::fmt::Display;
use crate::analytics::get_analytics;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Placeholder {
    Value,
    Loop
}

impl Display for Placeholder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Placeholder::Value => write!(f, "{{Value}}"),
            Placeholder::Loop => write!(f, "{{Loop}}")
        }
    }
}

impl FromStr for Placeholder {
    type Err = ();

    fn from_str(placeholder: &str) -> Result<Placeholder, Self::Err> {
        match placeholder {
            "{Value}" => Ok(Placeholder::Value),
            "{Loop}" => Ok(Placeholder::Loop),
            _ => Err(())
        }
    }
}

pub fn populate_html(filename: &str, database_connection: &ConnectionWithFullMutex) -> Result<(), Box<dyn Error>> {
    let mut filename = filename.to_string();
    if filename.len() > 0 {
        filename.remove(0);
    }

    let file_path = format!("src/html/{}.html", &filename);
    let generated_file_path = format!("generated/{}.html", &filename);

    let analytics = get_analytics(database_connection)?;

    let file_contents = read_to_string(&file_path)?;

    let ac_input = AhoCorasick::new(vec!["{placeholder}", "{loop}"]).unwrap();
    let ac_replace_strings = vec!["I'm replaced!", "Number two!"];
    
    let mut iter = 0;
    let mut replace_result = String::new();
    ac_input.replace_all_with(&file_contents, &mut replace_result, |_, found_string, write_to| {
        let replace_string = match Placeholder::from_str(found_string).unwrap() {
            Placeholder::Value => ac_replace_strings[iter],
            Placeholder::Loop => {
                ""
            }
        };

        write_to.push_str(replace_string);
        iter += 1;
        true
    });

    let generate_file = File::create(&generated_file_path)?;
    let mut filewriter = BufWriter::new(generate_file);
    let file_content_parts = replace_result.split("\n");

    for line in file_content_parts {
        writeln!(filewriter, "{}", line.clone())?;
    }

    Ok(())
}
