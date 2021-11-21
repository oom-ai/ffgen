use chrono::Utc;
use structopt::clap::{self, AppSettings};
use strum::{EnumString, EnumVariantNames, VariantNames};

pub use structopt::StructOpt;

use crate::feature_group::fraud_detection;

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

    /// Scenario fake data
    Generate(CategoryCommand),
}

#[derive(Debug, StructOpt)]
pub enum CategoryCommand {
    /// Feature group data
    Group {
        /// Target group
        #[structopt(subcommand)]
        group: GroupCommand,
    },

    /// Feature label
    Label {
        /// Target label name
        #[structopt(subcommand)]
        label: LabelCommand,
    },
}

macro_rules! group_command {
    ($($g:ident),* $(,)?) => {
        #[derive(Debug, StructOpt)]
        pub enum GroupCommand {
            $($g {
                /// Target group name
                #[structopt(possible_values = <concat_idents!($g, Group)>::VARIANTS , case_insensitive = true)]
                group: concat_idents!($g, Group),

                #[structopt(long, default_value = "1")]
                id_start: usize,

                #[structopt(long, default_value = "10")]
                limit: usize,
            },)*
        }
    };
}

macro_rules! label_command {
    ($($g:ident),* $(,)?) => {
        #[derive(Debug, StructOpt)]
        pub enum LabelCommand {
            $($g {
                /// Target label name
                #[structopt(possible_values = <concat_idents!($g, Label)>::VARIANTS, default_value = <concat_idents!($g, Label)>::VARIANTS[0], case_insensitive = true)]
                label: concat_idents!($g, Label),

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
            },)*
        }
    };
}

group_command!(FraudDetection);

label_command!(FraudDetection);

#[derive(EnumString, EnumVariantNames, Debug)]
#[strum(serialize_all = "snake_case")]
pub enum FraudDetectionGroup {
    Account(fraud_detection::FakeAccount),
    TransactionStats(fraud_detection::FakeTransactionStats),
}

#[derive(EnumString, EnumVariantNames, Debug)]
#[strum(serialize_all = "snake_case")]
pub enum FraudDetectionLabel {
    Label(fraud_detection::FakeLabel),
}
