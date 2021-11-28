use anyhow::{anyhow, Result};
use chrono::NaiveDateTime;
use fake::{faker::address::en::StateName, Fake};
use rand::{prelude::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use std::iter::{once, repeat};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
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

pub trait DataIter = Iterator<Item = Vec<Box<dyn erased_serde::Serialize>>>;

impl Recipe {
    pub fn generate_group_data<'a>(
        &'a self,
        rng: &'a mut (impl Rng + ?Sized),
        group: &str,
        id_range: Option<&(i64, i64)>,
    ) -> Result<(Vec<&str>, impl DataIter + 'a)> {
        let features = &self
            .groups
            .iter()
            .find(|g| g.name == group)
            .ok_or_else(|| {
                anyhow!(
                    "group not found int recipe. possible_values = {:?}",
                    self.groups.iter().map(|g| &g.name).collect::<Vec<_>>()
                )
            })?
            .features;

        let header = once(self.entity.name.as_str())
            .chain(features.iter().map(|f| f.name.as_str()))
            .collect::<Vec<_>>();

        let id_range = match id_range {
            Some(&(start, end)) => start..=end,
            None => self.entity.from..=self.entity.to,
        };

        let data_iter = id_range.map(|i| {
            once(self.entity.seq_gen.gen(i))
                .chain(features.iter().map(|f| f.rand_gen.gen(rng)))
                .collect::<Vec<_>>()
        });
        Ok((header, data_iter))
    }

    pub fn generate_label_data<'a>(
        &'a self,
        rng: &'a mut (impl Rng + ?Sized),
        label: &str,
        time_range: &(NaiveDateTime, NaiveDateTime),
        id_range: Option<&(i64, i64)>,
    ) -> Result<(Vec<&str>, impl DataIter + 'a)> {
        let features = &self
            .labels
            .iter()
            .find(|l| l.name == label)
            .ok_or_else(|| {
                anyhow!(
                    "label not found int self. possible_values = {:?}",
                    self.labels.iter().map(|g| &g.name).collect::<Vec<_>>()
                )
            })?
            .features;

        let header = once(self.entity.name.as_str())
            .chain(once("timestamp"))
            .chain(features.iter().map(|f| f.name.as_str()))
            .collect::<Vec<_>>();

        let (from, to) = match id_range {
            Some(&(start, end)) => (start, end),
            None => (self.entity.from, self.entity.to),
        };
        let id_gen = RandGen::Int { from, to };

        let &(from, to) = time_range;
        let ts_gen = RandGen::Timestamp { from, to };

        let data_iter = repeat(()).map(move |_| {
            once(id_gen.gen(rng))
                .chain(once(ts_gen.gen(rng)))
                .chain(features.iter().map(|f| f.rand_gen.gen(rng)))
                .collect::<Vec<_>>()
        });
        Ok((header, data_iter))
    }
}
