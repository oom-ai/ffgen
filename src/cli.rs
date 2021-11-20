use chrono::Utc;
use structopt::clap::{self, arg_enum, AppSettings};
pub use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    global_settings(&[AppSettings::ColoredHelp]),
    about = env!("CARGO_PKG_DESCRIPTION")
)
]
pub enum Opt {
    /// Generate shell completion file
    Completion {
        /// Target shell name
        #[structopt(possible_values = &clap::Shell::variants())]
        shell: clap::Shell,
    },

    /// Scenario
    Generate {
        /// Target scenario name
        #[structopt(possible_values = &Scenario::variants(), case_insensitive = true)]
        scenario: Scenario,

        /// Subcommand
        #[structopt(subcommand)]
        typecommand: TypeCommand,
    },
}

#[derive(Debug, StructOpt)]
pub enum TypeCommand {
    /// Feature group values
    Group {
        /// Group name
        #[structopt(name = "group")]
        group: String,

        #[structopt(long, default_value = "1")]
        id_start: usize,

        #[structopt(long, default_value = "10")]
        limit: usize,
    },

    /// Feature label values
    Label {
        #[structopt(long, default_value = "1")]
        id_start: usize,

        #[structopt(long, default_value = "10")]
        id_end: usize,

        #[structopt(long, default_value = "2021-01-01 00:00:00+00:00")]
        time_start: chrono::DateTime<Utc>,

        #[structopt(long, default_value = "2021-02-01 00:00:00+00:00")]
        time_end: chrono::DateTime<Utc>,

        #[structopt(long, default_value = "10")]
        limit: usize,
    },
}

arg_enum! {
    #[allow(non_camel_case_types)]
    pub enum Scenario {
        fraud_detection,
    }
}
