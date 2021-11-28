#![feature(trait_alias)]

mod cli;
mod recipe;
mod schema;
mod util;

use anyhow::Result;
use clap::{IntoApp, Parser};
use cli::*;
use rand::prelude::*;
use recipe::*;
use std::io::{self, Write};

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

fn try_main() -> Result<()> {
    let wtr = &mut io::stdout();
    match Opt::parse() {
        Opt::Group { group, id_range, rand, recipe, format } => {
            let mut rng: StdRng = rand.into();
            let mut recipe: Recipe = recipe.try_into()?;
            if let Some((from, to)) = id_range {
                recipe.entity.from = from;
                recipe.entity.to = to;
            };

            let (header, data_iter) = recipe.generate_group_data(&mut rng, &group)?;
            format.serialize(&header, data_iter, wtr)?;
        }
        Opt::Label { label, time_range, limit, id_range, rand, recipe, format } => {
            let mut rng: StdRng = rand.into();
            let mut recipe: Recipe = recipe.try_into()?;
            if let Some((from, to)) = id_range {
                recipe.entity.from = from;
                recipe.entity.to = to;
            };

            let (header, data_iter) = recipe.generate_label_data(&mut rng, &label, &time_range)?;
            format.serialize(&header, data_iter.take(limit), wtr)?;
        }
        Opt::Schema { category: SchemaCategory::Oomstore, recipe, format } => {
            let recipe: Recipe = recipe.try_into()?;
            let schema: schema::oomstore::Schema = recipe.try_into()?;
            format.serialize(&schema, wtr)?;
        }
        Opt::List { category, recipe } => {
            let recipe: Recipe = recipe.try_into()?;
            match category {
                ListCategory::Label => recipe.labels.iter().try_for_each(|l| writeln!(wtr, "{}", l.name))?,
                ListCategory::Group => recipe.groups.iter().try_for_each(|g| writeln!(wtr, "{}", g.name))?,
            }
        }
        Opt::Completion { shell } => {
            let app = &mut Opt::into_app();
            clap_generate::generate(shell, app, app.get_name().to_string(), &mut io::stdout())
        }
    }
    Ok(())
}
