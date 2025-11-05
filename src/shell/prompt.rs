use crate::shell::env::ShellEnv;

pub fn make_prompt(_env: &ShellEnv) -> String {
    // later can use last_status or user/host
    let cwd = std::env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "?".into());
    format!("{cwd} $ ")
}
