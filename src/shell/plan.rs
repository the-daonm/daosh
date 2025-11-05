#[derive(Debug, Clone)]
pub struct ExecPipeline {
    pub commands: Vec<ExecCommand>,
}

#[derive(Debug, Clone)]
pub struct ExecCommand {
    pub program: String,
    pub args: Vec<String>,
    // placeholder for later (>, <, >> ...)
    #[allow(dead_code)]
    pub redirs: Vec<()>,
}
