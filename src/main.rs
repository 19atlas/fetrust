use std::env::consts;

pub mod ekhizmet;
pub mod kynk;

fn main() {

    let infos = crate::kynk::sys::init();
    let kern: &str = consts::OS;
    let aile: &str = consts::FAMILY;
    let cputab: &str = consts::ARCH;

    println!(
        "    {}@{}\n--------------------\nOS ==> {}\nKernel ==> {} {}\nShell ==> {}\nAile ==> {}\nUptime ==> {}\nCPU ==> {} tabanli",
        infos.username, infos.hostname, infos.os_name, kern, infos.kernel, infos.shell, aile, infos.uptime, cputab
    );
}
