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
	pub cpu_type: String
}

pub mod sys {
	use crate::resource::SystemInfos;
	pub fn init() -> SystemInfos {
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
			cpu_type: get_cput()
		}
	}

	pub fn get_os() -> String {
		#[cfg(target_os = "windows")]
		return "Windows NT".to_string();
		#[cfg(target_os = "macos")]
		return "XNU/darwin".to_string();
		#[cfg(any(target_os = "linux", target_os = "android", target_os = "freebsd"))]
		return get_unix_distro("/etc/os-release");
		#[cfg(target_os = "dragonfly")]
		return "DragonflyBSD".to_string();
		#[cfg(target_os = "openbsd")]
		return "OpenBSD".to_string();
		#[cfg(target_os = "netbsd")]
		return "NetBSD".to_string();
	}
	
	pub fn get_release() -> String {
        let mut version = "unknown release".to_string();
        match std::process::Command::new("lsb_release").arg("-sr").output() {
		    Ok(release_d) => {version = String::from_utf8(release_d.stdout).expect("ver").replace("\n", "");} // gereksiz \n leri siler //turkish moment from creyde.sh
            _ => {}
        }
        version
	}
	
	pub fn get_unix_distro(file: &str) -> String {
		use std::fs;
		let os_release = fs::read_to_string(file).unwrap();
		let os_release: Vec<&str> = os_release.split("\n").collect();
		#[cfg(any(target_os = "linux", target_os = "android"))]
		let mut linux_distro = "GNU/Linux".to_string();
        #[cfg(target_os = "freebsd")]
		let mut linux_distro = "BSD".to_string();
		for line in 0..os_release.len() {
			let readed_line = os_release[line].to_string();
			if readed_line.starts_with("PRETTY_NAME=\"") {
				linux_distro = readed_line.replace("PRETTY_NAME=", "").replace("\"", "");
				break
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
		return get_kernel_version();
		#[cfg(target_os = "dragonfly")]
		return "BSD".to_string();
		#[cfg(target_os = "openbsd")]
		return "BSD".to_string();
		#[cfg(target_os = "netbsd")]
		return "BSD".to_string();
		#[cfg(target = "unix")]
		return "Unix".to_string();
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
			Err(_) => "Unknown".to_string(),
		};
		krl_vr
	}

	pub fn get_username() -> String {
		std::env::var(if cfg!(any(target_os = "linux", target_os = "freebsd")) {
			"USER"
		} else {
			"USERNAME"
		})
		.unwrap()
	}
	pub fn get_hostname() -> String {
		/* get $HOSTNAME with old way
		let hostname_var = if cfg!(target_os = "linux") {
			"HOSTNAME"
		} else {
			"COMPUTERNAME"
		};
		std::env::var(hostname_var).unwrap_or("hostname".to_string())*/
		use std::fs;
		let mut hostname_str = "unknown hostname".to_string();
        match fs::read_to_string("/etc/hostname") {
            Ok(file) => {hostname_str = file;}
            _ => {
                if cfg!(target_os = "freebsd") {
                    hostname_str = std::env::var("HOST").unwrap();
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
                let val = val.split('/').collect::<Vec<&str>>().pop().unwrap().to_string();
			    //or
                val
            },
			_ => "Unknown".to_string(),
		}
	}
	
	pub fn get_uptime() -> String {
		/*let up_time = match up_time {
			Ok(x) => {
				let time = String::from_utf8(x.stdout)
					.unwrap()
					/*.replace("hour(s)", "saat")
					.replace("minute(s)", "dakkikadir") //turkish moment from creyde.sh
					.replace("day(s)", "gün")
					.replace("up ", "")*/;
				time
			}
			Err(_) => "Can't get data.".to_string(),
		};*/
	    let up_time = cuptime_parser::command_uptime_parser();
		let up_time = up_time.replace("\n", ""); // gereksiz \n leri siler //turkish moment from creyde.sh
	
		up_time
	}

	pub fn get_kernel_name() -> String {
		let kernel_name: String = String::from(std::env::consts::OS);
		return kernel_name;
	}

	pub fn get_family() -> String {
		let family: String = String::from(std::env::consts::FAMILY);
		return family;
	}

	pub fn get_cput() -> String {
		let cput: String = String::from(std::env::consts::ARCH);
		return cput;
	}
}
