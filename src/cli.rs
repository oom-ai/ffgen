use chrono::{NaiveDate, NaiveDateTime};
use clap::{self, crate_authors, crate_description, crate_version, Parser};
use clap_generate::Shell;
use strum::{EnumString, EnumVariantNames, VariantNames};

use crate::feature_group::fraud_detection;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

#[derive(Debug, Parser)]
#[clap(
    about = crate_description!(),
    version = crate_version!(),
    author = crate_authors!(),
)]
pub enum Opt {
    /// Generate fake data
    #[clap(display_order = 1, aliases = &["gen"])]
    Generate(GenerateCmd),

    /// Generate shell completion file
    #[clap(display_order = 2)]
    Completion {
        /// Target shell name
        #[clap(arg_enum)]
        shell: Shell,
    },
}

#[derive(Debug, Parser)]
pub struct GenerateCmd {
    #[clap(subcommand)]
    pub subcommand: CategoryCmd,

    #[clap(long, default_value = "0", global = true)]
    pub seed: u64,
}

#[derive(Debug, Parser)]
pub enum CategoryCmd {
    /// Feature group data
    #[clap(display_order = 1)]
    Group {
        /// Target group
        #[clap(subcommand)]
        group: GroupCmd,
    },

    /// Feature label data
    #[clap(display_order = 2)]
    Label {
        /// Target label name
        #[clap(subcommand)]
        label: LabelCmd,
    },
}

macro_rules! group_cmd {
    ($($g:ident),* $(,)?) => {
        #[derive(Debug, Parser)]
        pub enum GroupCmd {
            $($g {
                /// Target group name
                #[clap(possible_values = <concat_idents!($g, Group)>::VARIANTS , case_insensitive = true)]
                group: concat_idents!($g, Group),

                /// ID range
                #[clap(long, default_value = "1..10", parse(try_from_str = parse_usize_range))]
                id_range: (usize, usize),
            },)*
        }
    };
}

macro_rules! label_cmd {
    ($($g:ident),* $(,)?) => {
        #[derive(Debug, Parser)]
        pub enum LabelCmd {
            $($g {
                /// Target label name
                #[clap(possible_values = <concat_idents!($g, Label)>::VARIANTS, default_value = <concat_idents!($g, Label)>::VARIANTS[0], case_insensitive = true)]
                label: concat_idents!($g, Label),

                #[clap(long, default_value = "1..10", parse(try_from_str = parse_usize_range))]
                id_range: (usize, usize),

                #[clap(long, default_value = "2021-01-01..2021-02-01", parse(try_from_str = parse_datetime_range))]
                time_range: (NaiveDateTime, NaiveDateTime),

                #[clap(long, default_value = "10")]
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
