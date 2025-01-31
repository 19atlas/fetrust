use ansi_term::Colour::*;
use std::fs;
use std::path::Path;
use tinyrand::Rand;
use tinyrand_std::thread_rand;

pub fn get_banner(bannered: String) -> String {
    let config_file = format!("{}/.config/fetrust/{}", env!("HOME"), "font.flf");
    if !Path::new(&config_file).exists() {
        println!("Creating default font ({})", config_file);
        if fs::create_dir_all(format!("{}/.config/fetrust", env!("HOME"))).is_err() {
            println!(
                "Error: Something happened wrong (creating {}/.config/",
                env!("HOME")
            )
        }
        //http://www.figlet.org/fonts/smslant.flf
        if fs::write(&config_file, include_str!("default-figlet.font.flf")).is_err() {
            println!("Error: Something happened wrong (writing {})", config_file)
        }
    }
    let slant = figlet_rs::FIGfont::from_file(config_file.as_str()).unwrap();
    let bannered = slant.convert(&bannered.replace("Linux", ""));
    assert!(bannered.is_some());
    bannered.unwrap().to_string()
}

pub fn handle_spacing(cache_text: &mut String, printing: &str, max_length: usize, padding: usize) {
    if cache_text.len() < max_length {
        let spaces = " ".repeat(max_length - cache_text.len() - padding);
        cache_text.push_str(&spaces);
    }
    cache_text.push_str(printing);
}

pub fn apply_color(color: &str, text: &str) -> String {
    match color {
        "black" => Black.paint(text).to_string(),
        "red" => Red.paint(text).to_string(),
        "green" => Green.paint(text).to_string(),
        "yellow" => Yellow.paint(text).to_string(),
        "blue" => Blue.paint(text).to_string(),
        "purple" => Purple.paint(text).to_string(),
        "cyan" => Cyan.paint(text).to_string(),
        "white" => White.paint(text).to_string(),
        "rand" | "random" => {
            let (r, g, b) = random_color_codes();
            RGB(r, g, b).paint(text).to_string()
        }
        _ => {
            println!(
                "{}",
                Yellow.paint(format!(
                    "Warning: Color \"{}\" isn't defined, so it's default color.",
                    color
                ))
            );
            text.to_string()
        }
    }
}

pub fn random_color_codes() -> (u8, u8, u8) {
    let mut rand = thread_rand();
    (
        rand.next_u16() as u8,
        rand.next_u16() as u8,
        rand.next_u16() as u8,
    )
}

pub fn format_memory(total: f64, used: f64) -> String {
    if total / 1024.0 > 1024.0 {
        format!(
            "{}MiB / {}MiB",
            (used / 1024.0).round() as u64,
            (total / 1024.0).round() as u64
        )
    } else if total > 1024.0 {
        format!("{}KiB / {}KiB", used.round() as u64, total.round() as u64)
    } else {
        format!(
            "{}Bytes / {}Bytes",
            (used * 1024.0).round() as u64,
            (total * 1024.0).round() as u64
        )
    }
}
