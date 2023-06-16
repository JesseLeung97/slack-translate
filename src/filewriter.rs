use std::fs::{File, read_to_string};
use std::io::{BufWriter, Error, Write };
use aho_corasick::AhoCorasick;


pub fn populate_html(filename: &str) -> Result<(), Error> {
    let mut filename = filename.to_string();
    if filename.len() > 0 {
        filename.remove(0);
    }

    let file_path = format!("src/html/{}.html", &filename);
    let generated_file_path = format!("generated/{}.html", &filename);

    let file_contents = read_to_string(&file_path)?;

    let ac_input = AhoCorasick::new(vec!["{placeholder}"]).unwrap();
    let ac_replace_strings = vec!["I'm replaced!", "Number two!"];
    
    let mut iter = 0;
    let mut replace_result = String::new();
    ac_input.replace_all_with(&file_contents, &mut replace_result, |_, _, write_to| {
        write_to.push_str(ac_replace_strings[iter]);
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
