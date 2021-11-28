mod cli;
mod core;
mod schema;

use clap::{IntoApp, Parser};
use cli::*;
use rand::prelude::*;
use schema::Schema;
use std::{
    fs::File,
    io::{self, BufReader, Write},
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
    let wtr = &mut io::stdout();
    match Opt::parse() {
        Opt::Group { group, id_range, rand: rand_opts, schema } => {
            let schema: Schema = schema.try_into()?;
            let mut rng: StdRng = rand_opts.into();
            core::generate_group_data(&mut rng, &schema, &group, id_range.as_ref(), wtr)?;
        }
        Opt::Label { label, time_range, limit, id_range, rand: rand_opts, schema } => {
            let schema: Schema = schema.try_into()?;
            let mut rng: StdRng = rand_opts.into();
            core::generate_label_data(&mut rng, &schema, &label, &time_range, id_range.as_ref(), limit, wtr)?;
        }
        Opt::Schema { category: SchemaCategory::Oomstore, schema } => {
            let schema: Schema = schema.try_into()?;
            let schema: schema::oomstore::Schema = schema.try_into()?;
            writeln!(wtr, "{}", serde_yaml::to_string(&schema)?)?;
        }
        Opt::List { category, schema } => {
            let schema: Schema = schema.try_into()?;
            match category {
                ListCategory::Label => schema.labels.iter().try_for_each(|l| writeln!(wtr, "{}", l.name))?,
                ListCategory::Group => schema.groups.iter().try_for_each(|g| writeln!(wtr, "{}", g.name))?,
            }
        }
        Opt::Completion { shell } => {
            let app = &mut Opt::into_app();
            clap_generate::generate(shell, app, app.get_name().to_string(), &mut io::stdout())
        }
    }
    Ok(())
}

impl From<RandOpt> for StdRng {
    fn from(opt: RandOpt) -> Self {
        let seed = opt.seed.unwrap_or_else(|| chrono::Utc::now().timestamp() as u64);
        StdRng::seed_from_u64(seed)
    }
}

impl TryFrom<SchemaOpt> for Schema {
    type Error = anyhow::Error;
    fn try_from(opt: SchemaOpt) -> Result<Self, Self::Error> {
        let file = File::open(opt.schema)?;
        Ok(serde_yaml::from_reader(BufReader::new(file))?)
    }
}
