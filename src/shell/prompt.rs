use crate::shell::env::ShellEnv;

pub fn make_prompt(env: &ShellEnv) -> String {
    let user = std::env::var("USER").unwrap_or_else(|_| "user".into());

    let host = std::env::var("HOSTNAME")
        .or_else(|_| std::fs::read_to_string("/etc/hostname").map(|s| s.trim().to_string()))
        .unwrap_or_else(|_| "host".into());

    let cwd_full = std::env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "?".into());

    let home = std::env::var("HOME").unwrap_or_default();
    let cwd_display = if !home.is_empty() && cwd_full.starts_with(&home) {
        cwd_full.replacen(&home, "~", 1)
    } else {
        cwd_full.clone()
    };

    let status_icon = if env.last_status == 0 {
        "\x1b[32m✓\x1b[0m"
    } else {
        "\x1b[31m✗\x1b[0m"
    };

    format!("{status_icon} {user}@{host} {cwd} $ ", cwd = cwd_display)
}
