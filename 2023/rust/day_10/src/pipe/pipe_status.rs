use std::fmt::Display;

#[derive(Debug)]
pub enum PipeStatus {
    MainLoop,
}

impl Display for PipeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            PipeStatus::MainLoop => "M",
        };
        write!(f, "{}", s)
    }
}
