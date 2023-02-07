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
		let version = String::from_utf8(release_d.stdout).expect("ver").replace("\n", ""); // gereksiz \n leri siler //turkish moment from creyde.sh
		version
	}
	pub fn get_linux_distro(file: &str) -> String {
		if std::path::Path::new(file).exists() {
			if let Ok(lines) = crate::reading::read::read_lines(file) {
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
	pub fn get_uptime() -> String {
		//`uptime -p` komutu
		let up_time = std::process::Command::new("uptime").arg("-p").output();
		let up_time = match up_time {
			Ok(x) => {
				let time = String::from_utf8(x.stdout)
					.unwrap()
					/*.replace("hours", "saat")
					.replace("hour", "saat")
					.replace("minutes", "dakkikadir")
					.replace("minute", "dakkikadir") //turkish moment from creyde.sh
					.replace("days", "gün")
					.replace("day", "gün")
					.replace("up ", "")*/;
				time
			}
			Err(_) => "veri alinamadi".to_string(),
		};
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
