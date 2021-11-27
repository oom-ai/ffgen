pub mod oomstore;

use chrono::NaiveDateTime;
use fake::{faker::address::en::StateName, Fake};
use rand::{prelude::SliceRandom, Rng};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    pub entity: Entity,
    pub groups: Vec<Group>,
    pub labels: Vec<Group>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub name:        String,
    pub description: Option<String>,
    pub from:        i64,
    pub to:          i64,
    pub seq_gen:     SeqGen,
}

impl Entity {
    pub fn len(&self) -> usize {
        match self.seq_gen {
            SeqGen::Int => self.to.to_string().len(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub name:        String,
    pub description: Option<String>,
    pub features:    Vec<Feature>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feature {
    pub name:        String,
    pub description: Option<String>,
    pub rand_gen:    RandGen,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all(deserialize = "snake_case"))]
pub enum RandGen {
    Int { from: i64, to: i64 },
    Bool { prob: f64 },
    State,
    Enum { values: Vec<String> },
    Timestamp { from: NaiveDateTime, to: NaiveDateTime },
}

impl RandGen {
    pub fn gen<R: Rng + ?Sized>(&self, rng: &mut R) -> Box<dyn erased_serde::Serialize> {
        match &self {
            RandGen::Int { from, to } => Box::new(rng.gen_range(*from..=*to)),
            RandGen::Bool { prob } => Box::new(rng.gen_bool(*prob)),
            RandGen::State => Box::new(StateName().fake_with_rng::<String, _>(rng)),
            RandGen::Enum { values } => Box::new(values.choose(rng).cloned()),
            RandGen::Timestamp { from, to } => Box::new(rng.gen_range(from.timestamp()..=to.timestamp())),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all(deserialize = "snake_case"))]
pub enum SeqGen {
    Int,
}

impl SeqGen {
    pub fn gen(&self, value: i64) -> Box<dyn erased_serde::Serialize> {
        match self {
            Self::Int => Box::new(value),
        }
    }
}
