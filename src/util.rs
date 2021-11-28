use crate::{cli::DataFormatOpt, schema::DataIter, RandOpt, Schema, SchemaOpt};
use anyhow::{Error, Result};
use rand::prelude::*;
use std::{
    fs::File,
    io::{self, BufReader},
};

impl From<RandOpt> for StdRng {
    fn from(opt: RandOpt) -> Self {
        let seed = opt.seed.unwrap_or_else(|| chrono::Utc::now().timestamp() as u64);
        StdRng::seed_from_u64(seed)
    }
}

impl TryFrom<SchemaOpt> for Schema {
    type Error = Error;
    fn try_from(opt: SchemaOpt) -> Result<Self, Self::Error> {
        let file = File::open(opt.schema)?;
        Ok(serde_yaml::from_reader(BufReader::new(file))?)
    }
}

impl DataFormatOpt {
    pub fn serialize(&self, header: Vec<&str>, mut data_iter: impl DataIter, wtr: impl io::Write) -> Result<()> {
        match self.format {
            crate::cli::DataFormat::Csv => {
                let mut wtr = csv::Writer::from_writer(wtr);
                wtr.write_record(header)?;
                data_iter.try_for_each(|r| wtr.serialize(&r))?;
            }
            crate::cli::DataFormat::Json => todo!(),
        }
        Ok(())
    }
}