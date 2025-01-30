use std::fs;
use std::path::Path;

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
        if fs::write(&config_file, include_str!("default-figlet.font.flf")).is_err(){
			println!("Error: Something happened wrong (writing {})", config_file)}
    }
    let slant = figlet_rs::FIGfont::from_file(config_file.as_str()).unwrap();
    let bannered = slant.convert(&bannered.replace("Linux", ""));
    assert!(bannered.is_some());
    bannered.unwrap().to_string()
}
