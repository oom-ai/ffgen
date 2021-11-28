use crate::{RandOpt, Schema, SchemaOpt};
use rand::prelude::*;
use std::{fs::File, io::BufReader};

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
