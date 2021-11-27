mod cli;
mod core;
mod schema;

use clap::{IntoApp, Parser};
use cli::*;
use rand::prelude::*;
use schema::{oomstore, Schema};
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

fn main() {
    if let Err(e) = try_main() {
        if let Some(ioerr) = e.root_cause().downcast_ref::<io::Error>() {
            if ioerr.kind() == io::ErrorKind::BrokenPipe {
                std::process::exit(0);
            }
        }
        eprintln!("{}: {}", env!("CARGO_PKG_NAME"), e);
        std::process::exit(1)
    }
}

fn try_main() -> anyhow::Result<()> {
    let opt = Opt::parse();
    let seed = opt.seed.unwrap_or_else(|| chrono::Utc::now().timestamp() as u64);
    let rng = &mut StdRng::seed_from_u64(seed);

    match opt.subcommand {
        Subcommand::Group { group } => {
            let schema = parse_schema(opt.file)?;
            core::generate_group_data(rng, &schema, &group, io::stdout())?;
        }
        Subcommand::Label { label, time_range, limit } => {
            let schema = parse_schema(opt.file)?;
            core::generate_label_data(rng, &schema, &label, &time_range, limit, io::stdout())?;
        }
        Subcommand::Schema { schema_type: SchemaType::OomStore } => {
            let schema = parse_schema(opt.file)?;
            let oomstore_schema: oomstore::Entity = schema.into();
            println!("{}", serde_yaml::to_string(&oomstore_schema)?);
        }
        Subcommand::Completion { shell } => {
            let app = &mut Opt::into_app();
            clap_generate::generate(shell, app, app.get_name().to_string(), &mut io::stdout())
        }
    }
    Ok(())
}

fn parse_schema(path: Option<PathBuf>) -> anyhow::Result<Schema> {
    let reader: Box<dyn BufRead> = match path {
        Some(path) => Box::new(BufReader::new(File::open(path)?)),
        None => Box::new(BufReader::new(io::stdin())),
    };
    Ok(serde_yaml::from_reader(reader)?)
}
