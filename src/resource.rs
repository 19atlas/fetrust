pub struct SystemInfos {
    pub os: String,
    pub os_release: String,
    pub username: String,
    pub hostname: String,
    pub kernel_name: String,
    pub kernel: String,
    pub shell: String,
    pub family: String,
    pub uptime: String,
    pub cpu_type: String,
    pub memory: String,
    pub theme_name: String,
    pub icon_theme: String,
    pub font_name: String,
    pub cursor_theme: String,
}

pub mod sys {
    use crate::{extra_fn::format_memory, resource::SystemInfos};
    use std::{
        fs::File,
        io::{BufRead, BufReader},
        process::Command,
    };
    pub fn init() -> SystemInfos {
        #[cfg(target_os = "linux")]
        let themes = get_themes();

        #[cfg(not(target_os = "linux"))]
        let themes = Themes {
            name: "N/A",
            icon: "N/A",
            font: "N/A",
            cursor: "N/A",
        };

        SystemInfos {
            os: get_os(),
            os_release: get_release(),
            username: get_username(),
            hostname: get_hostname(),
            kernel_name: get_kernel_name(),
            kernel: get_kernel(),
            shell: get_shell(),
            family: get_family(),
            uptime: get_uptime(),
            cpu_type: get_cput(),
            memory: get_memory(),
            theme_name: themes.name,
            icon_theme: themes.icon,
            font_name: themes.font,
            cursor_theme: themes.cursor,
        }
    }

    pub struct Themes {
        pub name: String,
        pub icon: String,
        pub font: String,
        pub cursor: String,
    }

    pub fn get_os() -> String {
        #[cfg(target_os = "windows")]
        return "Windows NT".to_string();
        #[cfg(target_os = "macos")]
        return "XNU/darwin".to_string();
        #[cfg(target_os = "android")]
        return "GNU/Linux".to_string();
        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
        return get_unix_distro("/etc/os-release");
        #[cfg(target_os = "dragonfly")]
        return "DragonflyBSD".to_string();
        #[cfg(target_os = "openbsd")]
        return "OpenBSD".to_string();
        #[cfg(target_os = "netbsd")]
        return "NetBSD".to_string();
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    pub fn get_release() -> String {
        let mut version = "unknown release".to_string();
        if let Ok(release_d) = Command::new("lsb_release").arg("-sr").output() {
            version = String::from_utf8(release_d.stdout)
                .expect("ver")
                .replace('\n', "");
        } // gereksiz \n leri siler //turkish moment from creyde.sh
        version
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    pub fn get_unix_distro(file: &str) -> String {
        use std::fs;
        let os_release = fs::read_to_string(file).unwrap();
        let os_release: Vec<&str> = os_release.split('\n').collect();
        #[cfg(target_os = "linux")]
        let mut linux_distro = "GNU/Linux".to_string();
        #[cfg(target_os = "freebsd")]
        let mut linux_distro = "BSD".to_string();
        for readed_line in &os_release {
            if readed_line.starts_with("PRETTY_NAME=\"") {
                linux_distro = readed_line.replace("PRETTY_NAME=", "").replace('\"', "");
                break;
            }
        }
        linux_distro
    }

    pub fn get_kernel() -> String {
        #[cfg(target_os = "windows")]
        return "NT".to_string();
        #[cfg(target_os = "macos")]
        return "XNU".to_string();
        #[cfg(target_os = "ios")]
        return "XNU".to_string();
        #[cfg(any(target_os = "linux", target_os = "android", target_os = "freebsd"))]
        return get_ukernel_info();
        #[cfg(any(target_os = "dragonfly", target_os = "openbsd", target_os = "netbsd"))]
        return "BSD".to_string();
    }

    pub fn get_ukernel_info() -> String {
        let krl_vr = std::process::Command::new("uname").arg("-r").output();
        let krl_vr = match krl_vr {
            Ok(x) => {
                let rev_kernel_ver: String =
                    String::from_utf8(x.stdout).unwrap().chars().rev().collect();
                let rev_kernel_ver = rev_kernel_ver
                    .split('\n')
                    .next_back()
                    .unwrap()
                    .chars()
                    .rev()
                    .collect();

                rev_kernel_ver
            }
            Err(_) => "Unknown".to_string(),
        };
        krl_vr
    }

    #[cfg(target_os = "windows")]
    pub fn get_memory() -> String {
        let output = Command::new("cmd").args(["/C", "systeminfo"]).output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);

                    let mut total: f64 = 0.0;
                    let mut free: f64 = 0.0;

                    for line in stdout.lines() {
                        if line.contains("Total Physical Memory") {
                            let parts: Vec<&str> = line.split(':').collect();
                            if let Some(value) = parts.get(1) {
                                let value = value.trim().replace(",", "").replace(" MB", "");
                                total = value.parse::<f64>().unwrap() * 1024.0 * 1024.0;
                            }
                        } else if line.contains("Available Physical Memory") {
                            let parts: Vec<&str> = line.split(':').collect();
                            if let Some(value) = parts.get(1) {
                                let value = value.trim().replace(",", "").replace(" MB", "");
                                free = value.parse::<f64>().unwrap() * 1024.0 * 1024.0;
                            }
                        }

                        if total > 0.0 && free > 0.0 {
                            break;
                        }
                    }

                    let used = total - free;
                    format_memory(total, used)
                } else {
                    format!(
                        "Failed to retrieve memory info: {}",
                        String::from_utf8_lossy(&output.stderr)
                    )
                }
            }
            Err(e) => format!("Failed to run command: {}", e),
        }
    }

    #[cfg(target_os = "linux")]
    pub fn get_memory() -> String {
        let file = File::open("/proc/meminfo").unwrap();
        let reader = BufReader::new(file);

        let mut total = 0.0;
        let mut free = 0.0;

        for line in reader.lines() {
            let line = line.unwrap();
            if line.starts_with("MemTotal") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                total = parts[1].parse::<f64>().unwrap_or(0.0);
            } else if line.starts_with("MemAvailable") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                free = parts[1].parse::<f64>().unwrap_or(0.0);
            }
            if total > 0.0 && free > 0.0 {
                break;
            }
        }

        let used = total - free;
        format_memory(total, used)
    }

    #[cfg(target_os = "macos")]
    pub fn get_memory() -> String {
        use std::process::Command;

        let total_output = Command::new("sysctl").arg("hw.memsize").output().unwrap();
        let total = String::from_utf8_lossy(&total_output.stdout)
            .split_whitespace()
            .last()
            .unwrap_or("0")
            .parse::<f64>()
            .unwrap_or(0.0)
            / 1024.0; // Convert Bytes to KiB

        let free_output = Command::new("vm_stat").output().unwrap();
        let free_pages: f64 = String::from_utf8_lossy(&free_output.stdout)
            .lines()
            .filter_map(|line| {
                if line.contains("Pages free:") {
                    line.split_whitespace()
                        .nth(2)
                        .and_then(|val| val.trim_end_matches('.').parse::<f64>().ok())
                } else {
                    None
                }
            })
            .next()
            .unwrap_or(0.0);

        let free = free_pages * 4096.0 / 1024.0; // Convert Pages to Bytes, then to KiB
        let used = total - free;
        format_memory(total, used)
    }

    #[cfg(any(target_os = "freebsd", target_os = "openbsd", target_os = "netbsd"))]
    pub fn get_memory() -> String {
        let total_output = Command::new("sysctl").arg("hw.physmem").output().unwrap();
        let total = String::from_utf8_lossy(&total_output.stdout)
            .split_whitespace()
            .last()
            .unwrap_or("0")
            .parse::<f64>()
            .unwrap_or(0.0)
            / 1024.0; // Convert Bytes to KiB

        let free_pages_output = Command::new("sysctl")
            .arg("vm.stats.vm.v_free_count")
            .output()
            .unwrap();
        let free_pages = String::from_utf8_lossy(&free_pages_output.stdout)
            .split_whitespace()
            .last()
            .unwrap_or("0")
            .parse::<f64>()
            .unwrap_or(0.0);

        let page_size_output = Command::new("sysctl").arg("hw.pagesize").output().unwrap();
        let page_size = String::from_utf8_lossy(&page_size_output.stdout)
            .split_whitespace()
            .last()
            .unwrap_or("0")
            .parse::<f64>()
            .unwrap_or(0.0);

        let free = free_pages * page_size / 1024.0; // Convert Pages to KiB
        let used = total - free;
        format_memory(total, used)
    }

    pub fn get_username() -> String {
        std::env::var(if cfg!(any(target_os = "linux", target_os = "freebsd")) {
            "USER"
        } else {
            "USERNAME"
        })
        .unwrap()
    }

    #[cfg(target_os = "windows")]
    pub fn get_hostname() -> String {
        Command::new("cmd")
            .args(["/C", "hostname"])
            .output()
            .expect("hostname")
    }

    #[cfg(not(target_os = "windows"))]
    pub fn get_hostname() -> String {
        let mut hostname_str = "unknown hostname".to_string();
        match std::fs::read_to_string("/etc/hostname") {
            Ok(file) => {
                hostname_str = file;
            }
            _ => {
                if let Ok(host) = std::env::var("HOST") {
                    hostname_str = host;
                }
                if hostname_str == "unknown hostname" {
                    hostname_str = std::str::from_utf8(
                        &std::process::Command::new("sh")
                            .arg("-c")
                            .arg("hostname")
                            .output()
                            .expect("[E] error on hostname command.")
                            .stdout,
                    )
                    .expect("[E] hostname contains non-utf8 characters.")
                    .to_string()
                    .replace('\n', "");
                }
            }
        }
        hostname_str
    }

    pub fn get_shell() -> String {
        use std::env::var;
        let shell_var = if cfg!(target_os = "linux") || cfg!(target_os = "freebsd") {
            "SHELL"
        } else {
            "COMSPEC"
        };

        match var(shell_var) {
            Ok(val) => {
                #[cfg(target_os = "freebsd")]
                let val = val
                    .split('/')
                    .collect::<Vec<&str>>()
                    .pop()
                    .unwrap()
                    .to_string();
                //or
                val
            }
            _ => "Unknown".to_string(),
        }
    }

    #[cfg(target_os = "windows")]
    pub fn get_uptime() -> String {
        Command::new("cmd")
            .args(["/C", "systeminfo | find 'Boot Time' "])
            .output()
            .expect("1")
    }

    #[cfg(not(target_os = "windows"))]
    fn get_uptime() -> String {
        use std::fs;

        #[allow(unused_imports)]
        use crate::extra_fn::{format_uptime, get_elapsed_seconds_since, parse_sysctl_boottime};

        #[cfg(target_os = "linux")]
        {
            if let Ok(contents) = fs::read_to_string("/proc/uptime") {
                let parts: Vec<&str> = contents.split_whitespace().collect();
                if let Some(uptime_str) = parts.first() {
                    if let Ok(uptime) = uptime_str.parse::<f64>() {
                        return format_uptime(uptime);
                    }
                }
            }
            "EUPTM".to_string()
        }

        #[cfg(any(
            target_os = "macos",
            target_os = "freebsd",
            target_os = "openbsd",
            target_os = "netbsd"
        ))]
        {
            if let Ok(output) = Command::new("sysctl")
                .arg("-n")
                .arg("kern.boottime")
                .output()
            {
                if let Ok(output_str) = String::from_utf8(output.stdout) {
                    if let Some(boot_time) = parse_sysctl_boottime(&output_str) {
                        let uptime = get_elapsed_seconds_since(boot_time);
                        return format_uptime(uptime);
                    }
                }
            }
            "EUPTM".to_string()
        }
    }

    pub fn get_kernel_name() -> String {
        let kernel_name: String = String::from(std::env::consts::OS);
        kernel_name
    }

    pub fn get_family() -> String {
        let family: String = String::from(std::env::consts::FAMILY);
        family
    }

    // this only return cpu arch
    /*#[cfg(not(target_os = "linux"))]
    pub fn get_cput() -> String {
        let cput: String = String::from(std::env::consts::ARCH);
        cput
    }*/

    #[cfg(target_os = "linux")]
    pub fn get_cput() -> String {
        let file = File::open("/proc/cpuinfo");
        if let Ok(file) = file {
            let reader = BufReader::new(file);

            let mut model_name = String::new();
            let mut clock_rate_mhz: f64 = 0.0;

            for line in reader.lines().map_while(Result::ok) {
                if line.starts_with("model name") {
                    model_name = line.split(':').nth(1).unwrap().trim().to_string();
                } else if line.starts_with("cpu MHz") {
                    clock_rate_mhz = line
                        .split(':')
                        .nth(1)
                        .unwrap()
                        .trim()
                        .parse::<f64>()
                        .unwrap_or(0.0);
                }

                if !model_name.is_empty() && clock_rate_mhz > 0.0 {
                    break;
                }
            }

            // Format the clock rate based on its magnitude
            let clock_rate = if clock_rate_mhz >= 1000.0 {
                format!("{:.3} GHz", clock_rate_mhz / 1000.0)
            } else {
                format!("{:.3} MHz", clock_rate_mhz)
            };

            return format!("{} @ {}", model_name, clock_rate);
        }

        // Return fallback if there's an error
        "ECPUI".to_string()
    }

    #[cfg(target_os = "windows")]
    fn get_cput() -> String {
        use std::process::Command;

        if let Ok(output) = Command::new("wmic")
            .args(["cpu", "get", "Name,MaxClockSpeed", "/format:list"])
            .output()
        {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let mut model_name = String::new();
            let mut clock_rate_mhz: f64 = 0.0;

            for line in output_str.lines() {
                if line.starts_with("Name=") {
                    model_name = line.split('=').nth(1).unwrap().to_string();
                } else if line.starts_with("MaxClockSpeed=") {
                    clock_rate_mhz = line
                        .split('=')
                        .nth(1)
                        .unwrap()
                        .parse::<f64>()
                        .unwrap_or(0.0);
                }
            }

            let clock_rate = if clock_rate_mhz >= 1000.0 {
                format!("{:.3} GHz", clock_rate_mhz / 1000.0)
            } else {
                format!("{:.3} MHz", clock_rate_mhz)
            };

            return format!("{} @ {}", model_name, clock_rate);
        }

        "ECPUI".to_string()
    }

    #[cfg(target_os = "macos")]
    fn get_cput() -> String {
        use std::process::Command;

        if let Ok(output) = Command::new("sysctl")
            .args(["-n", "machdep.cpu.brand_string"])
            .output()
        {
            let model_name = String::from_utf8_lossy(&output.stdout).trim().to_string();

            // macOS does not directly provide clock rate, so return only model name.
            return format!("{} @ Unknown Clock Rate", model_name);
        }

        "ECPUI".to_string()
    }

    #[cfg(target_os = "linux")]
    fn get_themes() -> Themes {
        use crate::ini_parser::ini_parser;

        let ini = ini_parser(&format!("{}/.config/gtk-3.0/settings.ini", env!("HOME"))).unwrap();

        let section = ini.get("Settings").unwrap();
        let theme_name = section.get("gtk-theme-name").unwrap();
        let icon_theme = section.get("gtk-icon-theme-name").unwrap();
        let font_name = section.get("gtk-font-name").unwrap();
        let cursor_theme = section.get("gtk-cursor-theme-name").unwrap();

        Themes {
            name: theme_name.to_string(),
            icon: icon_theme.to_string(),
            font: font_name.to_string(),
            cursor: cursor_theme.to_string(),
        }
    }
}
