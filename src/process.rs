// Module for stream processing


extern crate regex;
extern crate pcre;

use std::fs;
use std::io;
use std::io::{BufRead, Write};


pub fn process(filename: &str, rules: &pcre::Pcre, same_line: bool) -> Result<bool, String> {
    if filename == "-" {
        info!("Filename is '-' so use stdin.");

        let stream = io::stdin();
        let mut reader = stream.lock();

        process_stream(&mut reader, rules, same_line)
    } else {
        info!("Filename is '{}' so open a file", filename);

        let file = try!(fs::File::open(filename).map_err(|e| e.to_string()));
        let mut reader = io::BufReader::new(file);

        process_stream(&mut reader, rules, same_line)
    }
}

fn process_stream<R: io::BufRead>(reader: &mut R, rules: &pcre::Pcre, same_line: bool) -> Result<bool, String> {
    let stdout_stream = io::stdout();
    let mut stdout = stdout_stream.lock();

    for line in reader.lines() {
        match line {
            Ok(content) => {
                let trimmed_content = content.trim();
                debug!("Line: {}", trimmed_content);

                let matches = collect_matches(&trimmed_content, rules);
                debug!("Matches: {:?}", matches);

                if matches.len() == 0 {
                    continue
                }
                if same_line {
                    println!("{}", matches.connect(" "));
                    let _ = stdout.flush();
                } else {
                    for matched in matches.iter() {
                        println!("{}", matched);
                        let _ = stdout.flush();
                    }
                }
            },
            Err(error) => return Err(error.to_string())
        }
    }

    Ok(true)
}

fn collect_matches(content: &str, rules: &pcre::Pcre) -> Vec<String> {
    rules
        .matches(content)
        .filter(|gr| gr.group_len(0) > 0)
        .map(|gr| gr.group(0).to_string())
        .collect::<Vec<String>>()
}
