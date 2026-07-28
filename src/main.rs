use clap::Parser;
use notify_rust::{Notification, Timeout};
use std::process;

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

    /// Icon name or path (system icon name like 'dialog-information' or absolute file path)
    #[arg(short, long)]
    icon: Option<String>,
}

fn main() {
    let args = Args::parse();

    let mut notification = Notification::new();
    notification
        .summary(&args.summary)
        .body(&args.body)
        .appname(&args.app_name);

    if let Some(ref icon) = args.icon {
        notification.icon(icon);
    }

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

    if let Err(e) = notification.show() {
        eprintln!("Error sending desktop notification: {e}");
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_parsing_defaults() {
        let args = Args::parse_from(&["notifycli", "-s", "Hello", "-b", "World"]);
        assert_eq!(args.summary, "Hello");
        assert_eq!(args.body, "World");
        assert_eq!(args.app_name, "notifycli");
        assert_eq!(args.timeout, 5000);
        assert_eq!(args.icon, None);
    }

    #[test]
    fn test_args_parsing_custom() {
        let args = Args::parse_from(&[
            "notifycli",
            "--summary",
            "Alert",
            "--body",
            "Something happened",
            "--app-name",
            "MyApp",
            "--timeout",
            "10000",
            "--icon",
            "dialog-warning",
        ]);
        assert_eq!(args.summary, "Alert");
        assert_eq!(args.body, "Something happened");
        assert_eq!(args.app_name, "MyApp");
        assert_eq!(args.timeout, 10000);
        assert_eq!(args.icon, Some("dialog-warning".to_string()));
    }
}
