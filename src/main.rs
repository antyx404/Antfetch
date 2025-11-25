use std::collections::HashMap;
use std::process::Command;
use std::env::consts;

struct SystemInfo {
    os: String,
    hostname: String,
    kernel: String,
    uptime: String,
    shell: String,
    cpu: String,
    memory: String,
}

impl SystemInfo {
    fn new() -> Self {
        Self {
            os: Self::get_os_info(),
            hostname: Self::get_hostname(),
            kernel: Self::get_kernel(),
            uptime: Self::get_uptime(),
            shell: Self::get_shell(),
            cpu: Self::get_cpu(),
            memory: Self::get_memory(),
        }
    }

    fn get_os_info() -> String {
        if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
            for line in content.lines() {
                if line.starts_with("PRETTY_NAME=") {
                    return line.trim_start_matches("PRETTY_NAME=")
                        .trim_matches('"')
                        .to_string();
                }
            }
        }
        format!("{} {}", consts::OS, consts::ARCH)
    }

    fn get_hostname() -> String {
        std::fs::read_to_string("/proc/sys/kernel/hostname")
            .unwrap_or_else(|_| "unknown".to_string())
            .trim()
            .to_string()
    }

    fn get_kernel() -> String {
        std::fs::read_to_string("/proc/sys/kernel/osrelease")
            .unwrap_or_else(|_| "unknown".to_string())
            .trim()
            .to_string()
    }

    fn get_uptime() -> String {
        if let Ok(content) = std::fs::read_to_string("/proc/uptime") {
            if let Some(uptime_seconds) = content.split_whitespace().next() {
                if let Ok(seconds) = uptime_seconds.parse::<f64>() {
                    let hours = (seconds / 3600.0) as u32;
                    let minutes = ((seconds % 3600.0) / 60.0) as u32;
                    return format!("{}h {}m", hours, minutes);
                }
            }
        }
        "unknown".to_string()
    }

    fn get_shell() -> String {
        std::env::var("SHELL").unwrap_or_else(|_| "unknown".to_string())
    }

    fn get_cpu() -> String {
        if let Ok(content) = std::fs::read_to_string("/proc/cpuinfo") {
            for line in content.lines() {
                if line.starts_with("model name") {
                    if let Some(name) = line.split(':').nth(1) {
                        return name.trim().to_string();
                    }
                }
            }
        }
        "unknown".to_string()
    }

    fn get_memory() -> String {
        if let Ok(content) = std::fs::read_to_string("/proc/meminfo") {
            let mut mem_info = HashMap::new();
            for line in content.lines() {
                if let Some((key, value)) = line.split_once(':') {
                    let num: u64 = value.trim()
                        .split_whitespace()
                        .next()
                        .unwrap_or("0")
                        .parse()
                        .unwrap_or(0);
                    mem_info.insert(key.trim(), num);
                }
            }

            if let (Some(&total), Some(&available)) = 
                (mem_info.get("MemTotal"), mem_info.get("MemAvailable")) {
                let used = total - available;
                let used_gb = used as f64 / 1024.0 / 1024.0;
                let total_gb = total as f64 / 1024.0 / 1024.0;
                return format!("{:.1}GB / {:.1}GB", used_gb, total_gb);
            }
        }
        "unknown".to_string()
    }
}

fn get_logo() -> Vec<&'static str> {
    vec![
        "           |     |",
        "            \\   /",
        "             \\_/",
        "        __   /^\\   __",
        "       '  `. \\_/ ,'  `",
        "            \\/ \\/",
        "       _,--./| |\\.--._",
        "    _,'   _.-\\_/-._  `._",
        "         |   / \\   |",
        "         |  /   \\  |",
        "        /   |   |   \\",
        "      -'    \\___/    `-",
    ]
}

fn get_color_code() -> &'static str {
    "\x1b[36m" // cyan
}

fn main() {
    let info = SystemInfo::new();
    let logo = get_logo();
    let color = get_color_code();
    let reset = "\x1b[0m";

    let labels = vec![
        ("OS", info.os),
        ("Host", info.hostname),
        ("Kernel", info.kernel),
        ("Uptime", info.uptime),
        ("Shell", info.shell),
        ("CPU", info.cpu),
        ("Memory", info.memory),
    ];

    let max_logo_lines = logo.len();
    let max_info_lines = labels.len();

    for i in 0..std::cmp::max(max_logo_lines, max_info_lines) {
        if i < logo.len() {
            print!("{}{}{}", color, logo[i], reset);
        } else {
            print!("{:width$}", "", width = logo[0].len());
        }

        if i < labels.len() {
            let (label, value) = &labels[i];
            println!("  {}{}:{} {}", color, label, reset, value);
        } else {
            println!();
        }
    }
}