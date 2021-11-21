use chrono::{NaiveDate, NaiveDateTime};
use structopt::clap::{self, AppSettings};
use strum::{EnumString, EnumVariantNames, VariantNames};

pub use structopt::StructOpt;

use crate::feature_group::fraud_detection;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, StructOpt)]
#[structopt(
    global_settings(&[AppSettings::ColoredHelp]),
    about = env!("CARGO_PKG_DESCRIPTION"),
)]
pub enum Opt {
    /// Generate fake data
    #[structopt(display_order = 1, aliases = &["gen"])]
    Generate(GenerateCmd),

    /// Generate shell completion file
    #[structopt(display_order = 2)]
    Completion {
        /// Target shell name
        #[structopt(possible_values = &clap::Shell::variants())]
        shell: clap::Shell,
    },
}

#[derive(Debug, StructOpt)]
pub struct GenerateCmd {
    #[structopt(subcommand)]
    pub subcommand: CategoryCmd,

    // global not works properly: https://github.com/clap-rs/clap/issues/1570
    // #[structopt(long, global = true)]
    #[structopt(long, default_value = "0")]
    pub seed: u64,
}

#[derive(Debug, StructOpt)]
pub enum CategoryCmd {
    /// Feature group data
    #[structopt(display_order = 1)]
    Group {
        /// Target group
        #[structopt(subcommand)]
        group: GroupCmd,
    },

    /// Feature label data
    #[structopt(display_order = 2)]
    Label {
        /// Target label name
        #[structopt(subcommand)]
        label: LabelCmd,
    },
}

macro_rules! group_cmd {
    ($($g:ident),* $(,)?) => {
        #[derive(Debug, StructOpt)]
        pub enum GroupCmd {
            $($g {
                /// Target group name
                #[structopt(possible_values = <concat_idents!($g, Group)>::VARIANTS , case_insensitive = true)]
                group: concat_idents!($g, Group),

                /// ID range
                #[structopt(long, default_value = "1..10", parse(try_from_str = parse_usize_range))]
                id_range: (usize, usize),
            },)*
        }
    };
}

macro_rules! label_cmd {
    ($($g:ident),* $(,)?) => {
        #[derive(Debug, StructOpt)]
        pub enum LabelCmd {
            $($g {
                /// Target label name
                #[structopt(possible_values = <concat_idents!($g, Label)>::VARIANTS, default_value = <concat_idents!($g, Label)>::VARIANTS[0], case_insensitive = true)]
                label: concat_idents!($g, Label),

                #[structopt(long, default_value = "1..10", parse(try_from_str = parse_usize_range))]
                id_range: (usize, usize),

                #[structopt(long, default_value = "2021-01-01..2021-02-01", parse(try_from_str = parse_datetime_range))]
                time_range: (NaiveDateTime, NaiveDateTime),

                #[structopt(long, default_value = "10")]
                limit: usize,
            },)*
        }
    };
}

fn parse_usize_range(s: &str) -> Result<(usize, usize)> {
    parse_range(s, "..", |s| Ok(s.parse()?))
}

fn parse_datetime_range(s: &str) -> Result<(NaiveDateTime, NaiveDateTime)> {
    parse_range(s, "..", |s| Ok(parse_datetime(s)?))
}

fn parse_range<T: Ord>(s: &str, delimiter: &str, parse: fn(&str) -> Result<T>) -> Result<(T, T)> {
    match s.find(delimiter) {
        Some(pos) => Ok((parse(&s[..pos])?, parse(&s[pos + 2..])?)),
        None => Err(format!("range delimiter '..' not found in '{}'", s).into()),
    }
}

fn parse_datetime(s: &str) -> std::result::Result<NaiveDateTime, chrono::ParseError> {
    s.parse()
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S"))
        .or_else(|_| NaiveDate::parse_from_str(s, "%Y-%m-%d").map(|d| d.and_hms(0, 0, 0)))
}

group_cmd!(FraudDetection);

label_cmd!(FraudDetection);

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
