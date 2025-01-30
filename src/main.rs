mod resource;
mod extra_fn;

use another_json_minimal::*;
use ansi_term::Colour::*;
use extra_fn::get_banner;
use std::fs;
use std::path::Path;
use tinyrand::Rand;
use tinyrand_std::thread_rand;

pub fn random_color_codes() -> (u8, u8, u8) {
    let mut rand = thread_rand();
    (
        rand.next_u16() as u8,
        rand.next_u16() as u8,
        rand.next_u16() as u8,
    )
}

fn main() {
    let mut cache: Vec<String> = Vec::new();
    let mut max_length = 0;
    let infos = crate::resource::sys::init();
    let config_file = format!("{}/.config/fetrust/{}", env!("HOME"), "config.json");
    if !Path::new(&config_file).exists() {
        println!("Creating default config ({})", config_file);
        if fs::create_dir_all(format!("{}/.config/fetrust", env!("HOME"))).is_err() {
            println!(
                "Error: Something happened wrong (creating {}/.config)",
                env!("HOME")
            )
        }
        if fs::write(&config_file, b"{\n\t\"banner\":\t\t\t[[\"os\"], \"rand\"],\n\t\"user_a_host_name\":\t[[\"          \",\"username\",\"@\",\"hostname\",\"          \"], \"yellow\"],\n\t\"os\":\t\t\t\t[[\"os\t==> \",\"os\",\" \",\"release\"], null],\n\t\"kernel\":\t\t\t[[\"Kernel\t==> \",\"kernel_name\",\" \",\"kernel\"], \"green\"],\n\t\"shell\":\t\t\t[[\"Shell\t==> \",\"shell\"], \"purple\"],\n\t\"family\":\t\t\t[[\"Family\t==> \",\"family\"], \"cyan\"],\n\t\"uptime\":\t\t\t[[\"Uptime\t==> \",\"uptime\"], null],\n\t\"cpu_type\":\t\t\t[[\"CPUt\t==> \",\"cpu_type\"], null]\n}").is_err() {
			println!("Error: Something happened wrong (writing {})", config_file)}
    }
    let config_json = &fs::read(config_file).unwrap();
    let json = Json::parse(config_json);
    for info in [
        "banner",
        "user_a_host_name",
        "os",
        "kernel",
        "shell",
        "family",
        "uptime",
        "cpu_type",
    ] {
        if let Some(Json::OBJECT { name, value }) = json.as_ref().unwrap().get(info) {
            let mut printing = "".to_string();
            let mut printingc = "".to_string();
            if let Json::ARRAY(value) = value.unbox() {
                if let Json::ARRAY(value) = value[0].unbox() {
                    let mut printingl = vec!["".to_string(); value.len()];
                    let mut i = 0;
                    for string in value {
                        printingl[i] = string.print();
                        i += 1;
                    }
                    i = 0;

                    let bprintingl = printingl.clone();

                    for getter in bprintingl {
                        match getter.as_str() {
                            "os" => printingl[i] = infos.os.clone(),
                            "os_release" => printingl[i] = infos.os_release.clone(),
                            "username" => printingl[i] = infos.username.clone(),
                            "hostname" => printingl[i] = infos.hostname.clone(),
                            "kernel_name" => printingl[i] = infos.kernel_name.clone(),
                            "kernel" => printingl[i] = infos.kernel.clone(),
                            "shell" => printingl[i] = infos.shell.clone(),
                            "family" => printingl[i] = infos.family.clone(),
                            "uptime" => printingl[i] = infos.uptime.clone(),
                            "cpu_type" => printingl[i] = infos.cpu_type.clone(),
                            _ => {}
                        }
                        i += 1;
                    }
                    printingc = printingl.join("");
                }
                match value[1].unbox() {
                    Json::STRING(value) => {
                        match value.as_str() {
                            "black" => {
                                if name.as_str() == "banner" {
                                    printingc = get_banner(printingc.to_string());
                                }
                                printing = Black.paint(&printingc).to_string();
                            }
                            "red" => {
                                if name.as_str() == "banner" {
                                    printingc = get_banner(printingc.to_string());
                                }
                                printing = Red.paint(&printingc).to_string();
                            }
                            "green" => {
                                if name.as_str() == "banner" {
                                    printingc = get_banner(printingc.to_string());
                                }
                                printing = Green.paint(&printingc).to_string();
                            }
                            "yellow" => {
                                if name.as_str() == "banner" {
                                    printingc = get_banner(printingc.to_string());
                                }
                                printing = Yellow.paint(&printingc).to_string();
                            }
                            "blue" => {
                                if name.as_str() == "banner" {
                                    printingc = get_banner(printingc.to_string());
                                }
                                printing = Blue.paint(&printingc).to_string();
                            }
                            "purple" => {
                                if name.as_str() == "banner" {
                                    printingc = get_banner(printingc.to_string());
                                }
                                printing = Purple.paint(&printingc).to_string();
                            }
                            "cyan" => {
                                if name.as_str() == "banner" {
                                    printingc = get_banner(printingc.to_string());
                                }
                                printing = Cyan.paint(&printingc).to_string();
                            }
                            "white" => {
                                if name.as_str() == "banner" {
                                    printingc = get_banner(printingc.to_string());
                                }
                                printing = White.paint(&printingc).to_string();
                            }
                            "rand" | "random" => {
                                if name.as_str() == "banner" {
                                    printingc = get_banner(printingc.to_string());
                                }
                                let (r, g, b) = random_color_codes();
                                printing = RGB(r, g, b).paint(&printingc).to_string();
                            }
                            _ => {
                                printing = printingc;
                                println!("{}", Yellow.paint(format!("Warning: Color \"{}\" isn't defined, so it's default color.", value.as_str())));
                            }
                        }
                    }
                    Json::NULL => {
                        printing = printingc;
                    }
                    _ => {}
                }
                //idk should I delete this"
                match name.as_str() {
                    "banner" => {
                        for j in 0..8{
                            let mut temp_string = String::new();
                            if j > 1 && j < 7{
                                if let Some(banner_line) = printing.lines().nth(j-2){
                                    temp_string.push_str(banner_line);
                                    temp_string.push_str("       ");
                                    max_length = temp_string.len();
                                }
                            }
                            cache.push(temp_string);
                        }
                    }
                    "user_a_host_name" => {
                        if let Some(cache_text) = cache.get_mut(1){
                            if cache_text.len() < max_length{
                                cache_text.push_str(&(0..max_length-5).map(|_| String::from(" ")).collect::<Vec<String>>().join(""));
                            }
                            cache_text.push_str(&printing); // problem that cant 
                        }
                    }
                    "os" => {
                        if let Some(cache_text) = cache.get_mut(2){
                            cache_text.push_str(&printing);
                        }
                    }
                    "kernel" => {
                        if let Some(cache_text) = cache.get_mut(3){
                            cache_text.push_str(&printing);
                        }
                    }
                    "shell" => {
                        if let Some(cache_text) = cache.get_mut(4){
                            cache_text.push_str(&printing);
                        }
                    }
                    "family" => {
                        if let Some(cache_text) = cache.get_mut(5){
                            cache_text.push_str(&printing);
                        }
                    }
                    "uptime" => {
                        if let Some(cache_text) = cache.get_mut(6){
                            if cache_text.len() < max_length{
                                cache_text.push_str(&(0..max_length).map(|_| String::from(" ")).collect::<Vec<String>>().join(""));
                            }
                            cache_text.push_str(&printing);
                        }
                    }
                    "cpu_type" => {
                        if let Some(cache_text) = cache.get_mut(7){
                            cache_text.push_str(&printing);
                        }
                    }
                    _ => {}
                }
                //"
            }
        }
    }
    let printable_text = cache.join("\r\n");
    println!("{}", printable_text);
}
