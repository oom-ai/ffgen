use chrono::Utc;
use structopt::clap::{self, AppSettings};
use strum::{EnumString, EnumVariantNames, VariantNames};

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
    Generate(TypeCommand),
}

#[derive(Debug, StructOpt)]
pub enum TypeCommand {
    /// Feature group values
    Group {
        /// Target scenario name
        #[structopt(possible_values = &Scenario::VARIANTS, case_insensitive = true)]
        scenario: Scenario,

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
        /// Target scenario name
        #[structopt(possible_values = &Scenario::VARIANTS, case_insensitive = true)]
        scenario: Scenario,

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

#[derive(EnumString, EnumVariantNames, Debug)]
#[strum(serialize_all = "snake_case")]
pub enum Scenario {
    FraudDetection,
}
