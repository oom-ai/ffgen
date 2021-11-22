use chrono::{NaiveDate, NaiveDateTime};
use clap::{self, crate_authors, crate_description, crate_version, Parser, ValueHint};
use clap_generate::Shell;
use strum::{EnumString, EnumVariantNames, VariantNames};

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

    #[clap(long, global = true)]
    pub seed: Option<u64>,
}

#[derive(Debug, Parser)]
pub enum CategoryCmd {
    /// Feature group data
    #[clap(display_order = 1)]
    Group {
        /// Target group
        #[clap(name = "group", possible_values = Group::VARIANTS, value_hint = ValueHint::CommandString)]
        group:    Group,
        /// ID range
        #[clap(long, short = 'I', default_value = "1..10", parse(try_from_str = parse_usize_range))]
        id_range: (usize, usize),
    },

    /// Feature label data
    #[clap(display_order = 2)]
    Label {
        /// Target label name
        #[clap(name = "label", possible_values = Label::VARIANTS)]
        label: Label,

        #[clap(long, short = 'I', default_value = "1..10", parse(try_from_str = parse_usize_range))]
        id_range: (usize, usize),

        #[clap(long, short = 'T', default_value = "2021-01-01..2021-02-01", parse(try_from_str = parse_datetime_range))]
        time_range: (NaiveDateTime, NaiveDateTime),

        #[clap(long, short, default_value = "10")]
        limit: usize,
    },
}

fn parse_usize_range(s: &str) -> Result<(usize, usize)> {
    parse_range(s, "..", |s| Ok(s.parse()?))
}

fn parse_datetime_range(s: &str) -> Result<(NaiveDateTime, NaiveDateTime)> {
    parse_range(s, "..", parse_datetime)
}

fn parse_range<T: Ord>(s: &str, delimiter: &str, parse: fn(&str) -> Result<T>) -> Result<(T, T)> {
    match s.find(delimiter) {
        Some(pos) => Ok((parse(&s[..pos])?, parse(&s[pos + 2..])?)),
        None => Err(format!("range delimiter '..' not found in '{}'", s).into()),
    }
}

fn parse_datetime(s: &str) -> Result<NaiveDateTime> {
    s.parse()
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S"))
        .or_else(|_| NaiveDate::parse_from_str(s, "%Y-%m-%d").map(|d| d.and_hms(0, 0, 0)))
        .map_err(Box::new)
        .or_else(|_| Ok(NaiveDateTime::from_timestamp(s.parse::<i64>()?, 0)))
}

#[derive(EnumString, EnumVariantNames, Debug)]
#[strum(serialize_all = "snake_case")]
pub enum Group {
    FraudDetectionAccount,
    FraudDetectionTransactionStats,
}

#[derive(EnumString, EnumVariantNames, Debug)]
#[strum(serialize_all = "snake_case")]
pub enum Label {
    FraudDetectionLabel,
}
