use clap::Parser;
use notify_rust::{Hint, Notification, Timeout};
use std::{process, thread, time::Duration};

/// Lightweight, portable cross-platform desktop notification CLI tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Summary or title of the notification
    #[arg(short, long, default_value = "Notification")]
    summary: String,

    /// Main body content of the notification
    #[arg(short, long, default_value = "")]
    body: String,

    /// Application name displaying the notification
    #[arg(short = 'a', long, default_value = "notifycli")]
    app_name: String,

    /// Notification display timeout in milliseconds (0 for server default, or -1 for never expire)
    #[arg(short, long, default_value_t = 5000)]
    timeout: i32,

    /// Pin notification persistently (sets Resident hint and infinite timeout so it stays until dismissed)
    #[arg(long, default_value_t = false)]
    pin: bool,

    /// Icon name or path (system icon name like 'dialog-information' or absolute file path)
    #[arg(short, long)]
    icon: Option<String>,

    /// Progress bar percentage value (0 to 100) - automatically renders an ASCII progress bar in notification body
    #[arg(short, long, value_parser = clap::value_parser!(i32).range(0..=100))]
    progress: Option<i32>,

    /// Disable automatic ASCII progress bar appending when using -p
    #[arg(long, default_value_t = false)]
    no_bar: bool,

    /// Replace/update an existing notification by its ID (prints ID on creation if --print-id is set)
    #[arg(short = 'r', long)]
    replace_id: Option<u32>,

    /// Print notification ID to stdout after sending (useful for script chaining/updates)
    #[arg(long, default_value_t = false)]
    print_id: bool,

    /// Delay execution in milliseconds before sending (helps avoid KDE Plasma ExcessNotificationGeneration rate limits)
    #[arg(short = 'd', long, default_value_t = 0)]
    delay: u64,

    /// Categorize notification (e.g. 'email', 'transfer', 'device')
    #[arg(short = 'c', long)]
    category: Option<String>,

    /// Urgency level ('low', 'normal', 'critical')
    #[arg(short = 'u', long, default_value = "normal")]
    urgency: String,

    /// Desktop entry ID for desktop app association (e.g., 'org.kde.dolphin', 'firefox')
    #[arg(long)]
    desktop_entry: Option<String>,

    /// Origin name hint (KDE Plasma specific origin identifier)
    #[arg(long)]
    origin_name: Option<String>,

    /// Custom arbitrary hint (format: 'type:name:value', e.g. 'int:value:50' or 'string:desktop-entry:firefox')
    #[arg(long)]
    hint: Vec<String>,
}

fn generate_ascii_bar(percentage: i32) -> String {
    let total_blocks: usize = 20;
    let filled_blocks = ((percentage as f32 / 100.0) * total_blocks as f32).round() as usize;
    let empty_blocks = total_blocks.saturating_sub(filled_blocks);

    format!(
        "[{}{}] {}%",
        "█".repeat(filled_blocks),
        "░".repeat(empty_blocks),
        percentage
    )
}

fn parse_and_apply_custom_hint(notification: &mut Notification, hint_str: &str) {
    let parts: Vec<&str> = hint_str.splitn(3, ':').collect();
    if parts.len() == 3 {
        let (hint_type, name, val) = (parts[0], parts[1], parts[2]);
        match hint_type {
            "int" | "integer" | "i32" => {
                if let Ok(v) = val.parse::<i32>() {
                    notification.hint(Hint::CustomInt(name.to_string(), v));
                }
            }
            "string" | "str" => {
                notification.hint(Hint::Custom(name.to_string(), val.to_string()));
            }
            "bool" | "boolean" => {
                if let Ok(v) = val.parse::<bool>() {
                    notification.hint(Hint::Custom(name.to_string(), v.to_string()));
                }
            }
            _ => {
                notification.hint(Hint::Custom(name.to_string(), val.to_string()));
            }
        }
    } else {
        eprintln!("Warning: Invalid custom hint format '{hint_str}'. Expected 'type:name:value'");
    }
}

fn main() {
    let args = Args::parse();

    if args.delay > 0 {
        thread::sleep(Duration::from_millis(args.delay));
    }

    let mut body_text = args.body.clone();

    if let Some(progress) = args.progress {
        if !args.no_bar {
            let bar_str = generate_ascii_bar(progress);
            if body_text.is_empty() {
                body_text = bar_str;
            } else {
                body_text = format!("{}\n{}", body_text, bar_str);
            }
        }
    }

    let mut notification = Notification::new();
    notification
        .summary(&args.summary)
        .body(&body_text)
        .appname(&args.app_name);

    if let Some(id) = args.replace_id {
        notification.id(id);
    }

    if let Some(ref icon) = args.icon {
        notification.icon(icon);
    }

    if args.pin {
        notification.hint(Hint::Resident(true));
        notification.timeout(Timeout::Never);
    } else {
        match args.timeout {
            0 => {
                notification.timeout(Timeout::Default);
            }
            t if t < 0 => {
                notification.timeout(Timeout::Never);
            }
            ms => {
                notification.timeout(Timeout::Milliseconds(ms as u32));
            }
        }
    }

    if let Some(progress) = args.progress {
        notification.hint(Hint::CustomInt("value".to_string(), progress));
    }

    if let Some(ref category) = args.category {
        notification.hint(Hint::Category(category.clone()));
    }

    if let Some(ref desktop_entry) = args.desktop_entry {
        notification.hint(Hint::DesktopEntry(desktop_entry.clone()));
    }

    if let Some(ref origin) = args.origin_name {
        notification.hint(Hint::Custom("x-kde-origin-name".to_string(), origin.clone()));
    }

    for hint_arg in &args.hint {
        parse_and_apply_custom_hint(&mut notification, hint_arg);
    }

    match args.urgency.to_lowercase().as_str() {
        "low" => {
            notification.urgency(notify_rust::Urgency::Low);
        }
        "critical" => {
            notification.urgency(notify_rust::Urgency::Critical);
        }
        _ => {
            notification.urgency(notify_rust::Urgency::Normal);
        }
    }

    match notification.show() {
        Ok(handle) => {
            if args.print_id {
                println!("{}", handle.id());
            }
        }
        Err(e) => {
            eprintln!("Error sending desktop notification: {e}");
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_parsing_pin_flag() {
        let args = Args::parse_from(&["notifycli", "-s", "Pinned Task", "--pin"]);
        assert_eq!(args.pin, true);
    }
}
