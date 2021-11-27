use std::path::PathBuf;

use chrono::{NaiveDate, NaiveDateTime};
use clap::{self, crate_authors, crate_description, crate_version, Parser};
use clap_generate::Shell;
use strum::{EnumString, EnumVariantNames, VariantNames};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

#[derive(Debug, Parser)]
#[clap(
    about = crate_description!(),
    version = crate_version!(),
    author = crate_authors!(),
)]
pub struct Opt {
    #[clap(subcommand)]
    pub subcommand: Subcommand,

    /// Seed for the random generator
    #[clap(long, global = true, display_order = 100)]
    pub seed: Option<u64>,

    /// Schema file
    #[clap(short, long, global = true)]
    pub file: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    /// Generate feature group data
    #[clap(display_order = 1)]
    Group {
        /// Target group name
        #[clap()]
        group: String,
    },

    /// Generate feature label data
    #[clap(display_order = 2)]
    Label {
        /// Target label name
        #[clap()]
        label: String,

        /// Label time range
        #[clap(long, short = 'T', default_value = "2021-01-01..2021-02-01", parse(try_from_str = parse_datetime_range), display_order = 2)]
        time_range: (NaiveDateTime, NaiveDateTime),

        /// Max entries to generate
        #[clap(long, default_value = "10", display_order = 3)]
        limit: usize,
    },

    /// Generate oomstore schema
    #[clap(display_order = 3)]
    Schema {
        #[clap(possible_values = SchemaType::VARIANTS, default_value = SchemaType::VARIANTS[0])]
        schema_type: SchemaType,
    },

    #[clap(display_order = 100)]
    /// Generate shell completion file
    Completion {
        /// Target shell name
        #[clap(arg_enum)]
        shell: Shell,
    },
}

#[derive(EnumString, EnumVariantNames, Debug)]
#[strum(serialize_all = "snake_case")]
pub enum SchemaType {
    OomStore,
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
