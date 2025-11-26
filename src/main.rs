use std::collections::HashMap;
use std::env::consts;

struct SystemInfo {
    user: String,
    hostname: String,
    os: String,
    kernel: String,
    uptime: String,
    shell: String,
    cpu: String,
    cpu_cores: String,
    cpu_speed: String,
    memory: String,
}

impl SystemInfo {
    fn new() -> Self {
        Self {
            user: Self::get_user(),
            hostname: Self::get_hostname(),
            os: Self::get_os_info(),
            kernel: Self::get_kernel(),
            uptime: Self::get_uptime(),
            shell: Self::get_shell(),
            cpu: Self::get_cpu(),
            cpu_cores: Self::get_cpu_cores(),
            cpu_speed: Self::get_cpu_speed(),
            memory: Self::get_memory(),
        }
    }

    fn get_user() -> String {
        std::env::var("USER").unwrap_or_else(|_| "unknown".to_string())
    }

    fn get_hostname() -> String {
        std::fs::read_to_string("/proc/sys/kernel/hostname")
            .unwrap_or_else(|_| "unknown".to_string())
            .trim()
            .to_string()
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
                        let full_name = name.trim().to_string();
                        if full_name.len() > 30 {
                            return format!("{}...", &full_name[..27]);
                        }
                        return full_name;
                    }
                }
            }
        }
        "unknown".to_string()
    }

    fn get_cpu_cores() -> String {
        if let Ok(content) = std::fs::read_to_string("/proc/cpuinfo") {
            let cores = content.lines()
                .filter(|line| line.starts_with("processor"))
                .count();
            format!("{}", cores)
        } else {
            "unknown".to_string()
        }
    }

    fn get_cpu_speed() -> String {
        if let Ok(content) = std::fs::read_to_string("/proc/cpuinfo") {
            for line in content.lines() {
                if line.starts_with("cpu MHz") {
                    if let Some(speed) = line.split(':').nth(1) {
                        let mhz = speed.trim().parse::<f64>().unwrap_or(0.0);
                        let ghz = mhz / 1000.0;
                        return format!("{:.2}GHz", ghz);
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

    let user_host = format!("{}{}@{}{}", color, info.user, info.hostname, reset);
    
    let labels = vec![
        ("OS", info.os),
        ("Host", info.hostname.clone()),
        ("Kernel", info.kernel),
        ("Uptime", info.uptime),
        ("Shell", info.shell),
        ("CPU", format!("{} ({}) @ {}", info.cpu, info.cpu_cores, info.cpu_speed)),
        ("Memory", info.memory),
    ];

    let text_start_position = 30;
    
    let total_width = text_start_position + 30;

    println!("{:>width$}", user_host, width = total_width);

    for i in 0..logo.len() {
        if i >= 1 && i <= 7 {
            let (label, value) = &labels[i - 1];
            let info_text = format!("{}{}:{} {}", color, label, reset, value);
            
            let current_logo_length = logo[i].len();
            let padding_needed = if text_start_position > current_logo_length {
                text_start_position - current_logo_length
            } else {
                1
            };
            
            print!("{}{}{}", color, logo[i], reset);
            println!("{:width$}{}", "", info_text, width = padding_needed);
        } else {
            println!("{}{}{}", color, logo[i], reset);
        }
    }
}