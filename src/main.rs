#![feature(derive_default_enum)]

mod cli;
mod core;
mod schema;

use clap::{IntoApp, Parser};
use cli::*;
use rand::prelude::*;
use schema::oomstore;
use schema::Schema;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::parse();
    let wtr = csv::Writer::from_writer(io::stdout());
    let seed = opt.seed.unwrap_or_else(|| chrono::Utc::now().timestamp() as u64);
    let rng = &mut StdRng::seed_from_u64(seed);

    match opt.subcommand {
        Subcommand::Group { group } => {
            let schema = parse_schema(opt.file)?;
            core::fake_group(rng, &schema, &group, wtr)?;
        }
        Subcommand::Label {
            label,
            time_range,
            limit,
        } => {
            let schema = parse_schema(opt.file)?;
            core::fake_label(rng, &schema, &label, &time_range, limit, wtr)?;
        }
        Subcommand::Schema { schema_type } => {
            let schema = parse_schema(opt.file)?;
            match schema_type {
                SchemaType::OomStore => {
                    let oomstore_schema: oomstore::Entity = schema.into();
                    println!("{}", serde_yaml::to_string(&oomstore_schema)?);
                }
            }
        }
        Subcommand::Completion { shell } => {
            let app = &mut Opt::into_app();
            clap_generate::generate(shell, app, app.get_name().to_string(), &mut io::stdout())
        }
    }
    Ok(())
}

fn parse_schema(path: Option<PathBuf>) -> Result<Schema, Box<dyn Error>> {
    let reader: Box<dyn BufRead> = match path {
        Some(path) => Box::new(BufReader::new(File::open(path)?)),
        None => Box::new(BufReader::new(io::stdin())),
    };
    Ok(serde_yaml::from_reader(reader)?)
}
