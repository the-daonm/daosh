use crate::shell::env::ShellEnv;

pub fn is_builtin(name: &str) -> bool {
    matches!(name, "cd" | "exit")
}

pub fn run_builtin(name: &str, args: &[String], env: &mut ShellEnv) -> i32 {
    match name {
        "cd" => {
            let target = args.get(0).map(String::as_str).unwrap_or("/");

            if let Err(e) = std::env::set_current_dir(target) {
                eprintln!("cd: {e}");
                return 1;
            }

            0
        }
        "exit" => {
            env.should_exit = true;
            0
        }
        _ => 1,
    }
}
