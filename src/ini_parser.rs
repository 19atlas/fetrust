use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn ini_parser(file_path: &str) -> Result<HashMap<String, HashMap<String, String>>, std::io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut current_section = String::new();
    let mut ini_data = HashMap::new();
    let mut section_data = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            if !current_section.is_empty() {
                ini_data.insert(current_section.clone(), section_data);
            }

            current_section = line[1..line.len() - 1].to_string();
            section_data = HashMap::new();
        } else if let Some(eq_pos) = line.find('=') {
            let key = line[0..eq_pos].trim().to_string();
            let value = line[eq_pos + 1..].trim().to_string();
            section_data.insert(key, value);
        }
    }

    if !current_section.is_empty() {
        ini_data.insert(current_section, section_data);
    }

    Ok(ini_data)
}

pub fn ini_reader(file_path: &str, section: &str, field: &str) -> String {
    let ini_infos = ini_parser(file_path).unwrap();

    ini_infos
        .get(section)
        .unwrap()
        .get(field)
        .unwrap()
        .to_string()

    /*match ini_parser(ini_path) {
        Ok(ini_data) => {
            for (section, data) in ini_data {
                println!("[{}]", section);
                for (key, value) in data {
                    println!("{} = {}", key, value);
                }
            }
        }
        Err(e) => eprintln!("Error reading INI file: {}", e),
    }*/
}
