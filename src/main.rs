use std::env::consts;
pub mod ekhizmet;
pub mod kynk;

fn main() {

    let infos = crate::kynk::sys::init();
    let kern: &str = consts::OS;
    let aile: &str = consts::FAMILY;
    let cputab: &str = consts::ARCH;

    println!(
        "       \x1b[1m{}@{}\n\x1b[4m                            \x1b[0m\nOS ==> {} {}\nKernel ==> \x1b[33m{} {}\x1b[37m\nShell ==> \x1b[32m{}\x1b[37m\nAile ==> \x1b[35m{}\x1b[37m\nUptime ==> \x1b[36m{}\x1b[37m\nCPU ==> {} tabanli",
        infos.username, infos.hostname, infos.os_name, infos.os_rel, kern,infos.kernel , infos.shell, aile, infos.uptime, cputab
    );
}
