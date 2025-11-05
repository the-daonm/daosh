use std::collections::HashMap;

pub struct ShellEnv {
    pub should_exit: bool,
    pub last_status: i32,
    vars: HashMap<String, String>,
}

impl ShellEnv {
    pub fn new() -> Self {
        let mut vars = HashMap::new();
        for (k, v) in std::env::vars() {
            vars.insert(k, v);
        }
        Self {
            should_exit: false,
            last_status: 0,
            vars,
        }
    }

    pub fn get_var(&self, key: &str) -> Option<String> {
        self.vars.get(key).cloned()
    }

    #[allow(dead_code)]
    pub fn set_var<S: Into<String>>(&mut self, key: S, val: S) {
        self.vars.insert(key.into(), val.into());
    }
}
