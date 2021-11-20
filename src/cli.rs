use chrono::Utc;
use structopt::clap::{self, arg_enum, AppSettings};
pub use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    global_settings(&[AppSettings::ColoredHelp]),
    about = env!("CARGO_PKG_DESCRIPTION"))
]
pub struct Opt {
    /// Subcommand
    #[structopt(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(Debug, StructOpt)]
pub enum Subcommand {
    /// Generate shell completion file
    Completion(CompletionOpt),

    /// Scenario
    Scenario(ScenarioOpt),
}

#[derive(Debug, StructOpt)]
pub struct CompletionOpt {
    /// Target shell name
    #[structopt(possible_values = &clap::Shell::variants())]
    pub shell: clap::Shell,
}

#[derive(Debug, StructOpt)]
pub struct ScenarioOpt {
    /// Target scenario name
    #[structopt(possible_values = &ScenarioName::variants())]
    pub scenario: ScenarioName,

    /// Group name
    #[structopt(short, long)]
    pub group: String,

    #[structopt(long, default_value = "1")]
    pub id_start: usize,

    #[structopt(long, default_value = "10")]
    pub id_end: usize,

    #[structopt(long, default_value = "2021-01-01 00:00:00+00:00")]
    pub time_start: chrono::DateTime<Utc>,

    #[structopt(long, default_value = "2021-02-01 00:00:00+00:00")]
    pub time_end: chrono::DateTime<Utc>,

    #[structopt(long, default_value = "10")]
    pub limit: usize,
}

arg_enum! {
    #[derive(Debug)]
    pub enum ScenarioName {
        fraud_detection
    }
}

