use anyhow::{anyhow, bail, Error, Result};
use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc};
use clap::{self, AppSettings, Args, Parser};
use clap_complete::Shell;
use std::path::PathBuf;
use strum::{EnumString, EnumVariantNames, VariantNames};

#[derive(Debug, Parser)]
#[clap(about, version)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
pub enum Opt {
    /// Generate feature group data
    Group {
        /// Target group name
        group: Option<String>,

        /// ID range (example: 1000..2000)
        #[clap(long, short = 'I', parse(try_from_str = parse_i64_range), value_name = "FROM..TO")]
        id_range: Option<(i64, i64)>,

        #[clap(flatten)]
        rand: RandOpt,

        #[clap(flatten)]
        recipe: RecipeOpt,

        #[clap(flatten)]
        format: DataFormatOpt,
    },

    /// Generate feature label data
    Label {
        /// Target group name
        group: Option<String>,

        /// ID range (example: 1000..2000)
        #[clap(long, short = 'I', parse(try_from_str = parse_i64_range), value_name = "FROM..TO")]
        id_range: Option<(i64, i64)>,

        /// Label time range (example: 2022-01-01..2022-01-31)
        #[clap(long, short = 'T', parse(try_from_str = parse_datetime_range), value_name = "FROM..TO")]
        time_range: Option<(DateTime<Utc>, DateTime<Utc>)>,

        /// Max entries to generate
        #[clap(short, long, default_value = "10", value_name = "N")]
        limit: usize,

        #[clap(flatten)]
        rand: RandOpt,

        #[clap(flatten)]
        recipe: RecipeOpt,

        #[clap(flatten)]
        format: DataFormatOpt,
    },

    /// Generate feature store schema
    Schema {
        /// Schema category
        #[clap(possible_values = SchemaCategory::VARIANTS, default_value = SchemaCategory::VARIANTS[0])]
        category: SchemaCategory,

        #[clap(flatten)]
        recipe: RecipeOpt,

        #[clap(flatten)]
        format: SchemaFormatOpt,
    },

    /// List available groups
    List {
        #[clap(flatten)]
        recipe: RecipeOpt,
    },

    /// Output shell completion code
    Completion {
        /// Target shell name
        #[clap(arg_enum)]
        shell: Shell,
    },
}

#[derive(Debug, Args)]
pub struct RandOpt {
    /// Seed for the random generator
    #[clap(short, long)]
    pub seed: Option<u64>,
}

#[derive(Debug, Args)]
pub struct RecipeOpt {
    /// Recipe file path
    #[clap(short, long)]
    pub recipe: PathBuf,
}

#[derive(Debug, Args)]
pub struct DataFormatOpt {
    /// Data format
    #[clap(short, long, possible_values = DataFormat::VARIANTS, default_value = DataFormat::VARIANTS[0])]
    pub format: DataFormat,
}

#[derive(EnumString, EnumVariantNames, Debug)]
#[strum(serialize_all = "snake_case")]
pub enum DataFormat {
    Csv,
    Json,
    Yaml,
}

#[derive(Debug, Args)]
pub struct SchemaFormatOpt {
    /// Schema format
    #[clap(long, possible_values = SchemaFormat::VARIANTS, default_value = SchemaFormat::VARIANTS[0])]
    pub format: SchemaFormat,
}

#[derive(EnumString, EnumVariantNames, Debug)]
#[strum(serialize_all = "snake_case")]
pub enum SchemaFormat {
    Yaml,
    Json,
    Toml,
}

#[derive(EnumString, EnumVariantNames, Debug)]
#[strum(serialize_all = "snake_case")]
pub enum SchemaCategory {
    Oomstore,
}

#[derive(EnumString, EnumVariantNames, Debug)]
#[strum(serialize_all = "snake_case")]
pub enum ListCategory {
    Label,
    Group,
}

fn parse_i64_range(s: &str) -> Result<(i64, i64)> {
    parse_range(s, "..", |s| Ok(s.parse()?))
}

fn parse_datetime_range(s: &str) -> Result<(DateTime<Utc>, DateTime<Utc>)> {
    parse_range(s, "..", parse_utc_datetime)
}

fn parse_range<T>(s: &str, delimiter: &str, parse: fn(&str) -> Result<T>) -> Result<(T, T)> {
    match s.find(delimiter) {
        Some(pos) => Ok((parse(&s[..pos])?, parse(&s[pos + 2..])?)),
        None => bail!("range delimiter '..' not found in '{}'", s),
    }
}

fn parse_utc_datetime(s: &str) -> Result<DateTime<Utc>> {
    s.parse()
        .or_else(|_| Ok(Utc.timestamp_millis(s.parse()?)))
        .or_else(|_: Error| Ok(Utc.from_utc_datetime(&parse_naive_datetime(s)?)))
}

fn parse_naive_datetime(s: &str) -> Result<NaiveDateTime> {
    s.parse()
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S"))
        .or_else(|_| NaiveDate::parse_from_str(s, "%Y-%m-%d").map(|d| d.and_hms(0, 0, 0)))
        .map_err(|e| anyhow!(e))
}
