use crate::{
    cli::{DataFormatOpt, SchemaFormat, SchemaFormatOpt},
    recipe::{DataIter, Recipe},
    RandOpt,
    RecipeOpt,
};
use anyhow::{Error, Result};
use rand::prelude::*;
use serde::ser::{self, SerializeSeq, Serializer};
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufReader},
};

impl From<RandOpt> for StdRng {
    fn from(opt: RandOpt) -> Self {
        let seed = opt.seed.unwrap_or_else(|| chrono::Utc::now().timestamp_nanos() as u64);
        StdRng::seed_from_u64(seed)
    }
}

impl TryFrom<RecipeOpt> for Recipe {
    type Error = Error;
    fn try_from(opt: RecipeOpt) -> Result<Self, Self::Error> {
        let file = File::open(opt.recipe)?;
        Ok(serde_yaml::from_reader(BufReader::new(file))?)
    }
}

impl DataFormatOpt {
    pub fn serialize(&self, header: &[&str], mut data_iter: impl DataIter, wtr: impl io::Write) -> Result<()> {
        match self.format {
            crate::cli::DataFormat::Csv => {
                let mut wtr = csv::Writer::from_writer(wtr);
                wtr.write_record(header)?;
                data_iter.try_for_each(|r| wtr.serialize(&r))?;
            }
            crate::cli::DataFormat::Json => {
                let mut ser = serde_json::Serializer::new(wtr);
                let mut seq = ser.serialize_seq(None)?;
                for row in data_iter {
                    let map: HashMap<_, _> = header.iter().zip(row.iter()).collect();
                    seq.serialize_element(&map)?
                }
                seq.end()?;
            }
            crate::cli::DataFormat::Yaml => {
                let mut ser = serde_yaml::Serializer::new(wtr);
                let mut seq = ser.serialize_seq(None)?;
                for row in data_iter {
                    let map: HashMap<_, _> = header.iter().zip(row.iter()).collect();
                    seq.serialize_element(&map)?
                }
                seq.end()?;
            }
        }
        Ok(())
    }
}

impl SchemaFormatOpt {
    pub fn serialize<T>(&self, schema: &T, mut wtr: impl io::Write) -> Result<()>
    where
        T: ser::Serialize,
    {
        match self.format {
            SchemaFormat::Yaml => {
                writeln!(wtr, "{}", serde_yaml::to_string(schema)?)?;
            }
            SchemaFormat::Json => {
                writeln!(wtr, "{}", serde_json::to_string(schema)?)?;
            }
            SchemaFormat::Toml => {
                // Note: the following code will not work: https://github.com/alexcrichton/toml-rs/issues/142
                // writeln!(wtr, "{}", toml::to_string(schema)?)?;

                let value = toml::Value::try_from(schema)?;
                writeln!(wtr, "{}", toml::to_string(&value)?)?;
            }
        }
        Ok(())
    }
}
