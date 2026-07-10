use clap::{Parser, ValueEnum};

#[derive(Parser, Default)]
pub struct Cli {
    #[arg(long, value_enum, default_value_t = RunMode::Default)]
    pub mode: RunMode,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Default)]
pub enum RunMode {
    #[default]
    Default,
    SquareTest,
}

