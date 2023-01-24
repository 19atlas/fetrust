pub struct SystemInfos {
    pub os_name: String,
    pub os_rel: String,
    pub kernel: String,
    pub username: String,
    pub hostname: String,
    pub shell: String,
    pub uptime: String
}
pub mod sys {
    use crate::kynk::SystemInfos;
    pub fn init() -> SystemInfos {
        SystemInfos {
            os_name: get_os(),
            os_rel: get_release(),
            kernel: get_kernel(),
            username: get_username(),
            hostname: get_hostname(),
            shell: get_shell(),
            uptime: get_syszaman()
        }
    }

    pub fn get_os() -> String {
        #[cfg(target_os = "windows")]
        return "Windows NT".to_string();
        #[cfg(target_os = "macos")]
        return "XNU/darwin".to_string();
        #[cfg(target_os = "linux")]
        return get_linux_distro("/etc/os-release");
        #[cfg(target_os = "android")]
        return get_linux_distro("/etc/os-release");
        #[cfg(target_os = "freebsd")]
        return "FreeBSD".to_string();
        #[cfg(target_os = "dragonfly")]
        return "DragonflyBSD".to_string();
        #[cfg(target_os = "openbsd")]
        return "OpenBSD".to_string();
        #[cfg(target_os = "netbsd")]
        return "NetBSD".to_string();
    }
    pub fn get_release() -> String {
        let release_d = std::process::Command::new("lsb_release").arg("-sr").output().expect("release");
        let version = String::from_utf8(release_d.stdout).expect("ver").replace("\n", ""); // gereksiz \n leri siler
        version
    }
    pub fn get_linux_distro(file: &str) -> String {
        if std::path::Path::new(file).exists() {
            if let Ok(lines) = crate::ekhizmet::yardimlar::read_lines(file) {
                for line in lines {
                    if let Ok(ip) = line {
                        if ip.starts_with('P') {
                            if ip.contains("PRETTY_NAME=\"") {
                                return ip.replace("PRETTY_NAME=", "").replace("\"", "");
                            }
                        }
                    }
                }
            }
        }
        "GNU/Linux".to_string()
    }
    pub fn get_kernel() -> String {
        #[cfg(target_os = "windows")]
        return "NT".to_string();
        #[cfg(target_os = "macos")]
        return "XNU".to_string();
        #[cfg(target_os = "ios")]
        return "XNU".to_string();
        #[cfg(target_os = "android")] return get_kernel_version();
        #[cfg(target_os = "freebsd")]
        return "BSD".to_string();
        #[cfg(target_os = "dragonfly")]
        return "BSD".to_string();
        #[cfg(target_os = "openbsd")]
        return "BSD".to_string();
        #[cfg(target_os = "netbsd")]
        return "BSD".to_string();
        #[cfg(target = "unix")]
        return "Unix".to_string();
        #[cfg(target_os = "linux")] return get_kernel_version();
    
    }

    pub fn get_kernel_version() -> String {
        let krl_vr = std::process::Command::new("uname").arg("-r").output();
        let krl_vr = match krl_vr {
            Ok(x) => {
                let rev_kernel_ver: String =
                    String::from_utf8(x.stdout).unwrap().chars().rev().collect();
                let rev_kernel_ver = rev_kernel_ver
                    .split("\n")
                    .last()
                    .unwrap()
                    .chars()
                    .rev()
                    .collect();
                    
                rev_kernel_ver
            }
            Err(_) => "Bilinmeyen".to_string(),
        };
        krl_vr
    }

    pub fn get_username() -> String {
        std::env::var(if cfg!(target_os = "linux") {
            "USER"
        } else {
            "USERNAME"
        })
        .unwrap()
    }
    pub fn get_hostname() -> String {
        let hostname_var = if cfg!(target_os = "linux") {
            "HOSTNAME"
        } else {
            "COMPUTERNAME"
        };
        std::env::var(hostname_var).unwrap_or("hostname".to_string())
    }
    pub fn get_shell() -> String {
        use std::env::var;
        let shell_var = if cfg!(target_os = "linux") {
            "SHELL"
        } else {
            "COMSPEC"
        };
        match var(shell_var) {
            Ok(val) => val,
            Err(_) => "Unknown".to_string(),
        }
    }
    pub fn get_syszaman() -> String {
        //`uptime -p` komutu
        let up_time = std::process::Command::new("uptime").arg("-p").output();
        let up_time = match up_time {
            Ok(x) => {
                let time = String::from_utf8(x.stdout)
                    .unwrap()
                    .replace("hours", "saat")
                    .replace("hour", "saat")
                    .replace("minutes", "dakkikadir")
                    .replace("minute", "dakkikadir")
                    .replace("days", "gün")
                    .replace("day", "gün")
                    .replace("up ", "");
                time
            }
            Err(_) => "veri alinamadi".to_string(),
        };
        let up_time = up_time.replace("\n", ""); // gereksiz \n leri siler
    
        up_time
    }
}