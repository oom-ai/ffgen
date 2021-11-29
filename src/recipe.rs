use anyhow::{anyhow, Result};
use chrono::NaiveDateTime;
use fake::{faker::address::en::StateName, Fake};
use rand::{prelude::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use std::{
    iter::{once, repeat},
    ops::RangeInclusive,
};

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

    #[serde(default = "default_entity_seq_range")]
    pub seq_range: RangeInclusive<i64>,

    #[serde(default = "default_entity_time_range")]
    pub time_range: RangeInclusive<NaiveDateTime>,

    #[serde(default = "entity_id_type_default")]
    pub id_type: EntityIdType,
}

fn default_entity_seq_range() -> RangeInclusive<i64> {
    1..=10
}
fn default_entity_time_range() -> RangeInclusive<NaiveDateTime> {
    let end = chrono::Local::now().naive_local();
    let start = end - chrono::Duration::days(1);
    start..=end
}
fn entity_id_type_default() -> EntityIdType {
    EntityIdType::Int
}

impl Entity {
    pub fn len(&self) -> usize {
        match self.id_type {
            EntityIdType::Int => self.seq_range.end().to_string().len(),
            EntityIdType::Md5 => 32,
        }
    }

    pub fn seq(&self, value: i64) -> Box<dyn erased_serde::Serialize> {
        match self.id_type {
            EntityIdType::Int => Box::new(value),
            EntityIdType::Md5 => {
                use md5::Digest;
                Box::new(format!("{:x}", md5::Md5::digest(&value.to_be_bytes())))
            }
        }
    }

    pub fn rand_id<R: Rng + ?Sized>(&self, rng: &mut R) -> Box<dyn erased_serde::Serialize> {
        Box::new(self.seq(rng.gen_range(self.seq_range.clone())))
    }

    pub fn rand_ts<R: Rng + ?Sized>(&self, rng: &mut R) -> Box<dyn erased_serde::Serialize> {
        let start = self.time_range.start().timestamp();
        let end = self.time_range.end().timestamp();
        Box::new(rng.gen_range(start..=end))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub name:        String,
    pub description: Option<String>,
    #[serde(default)]
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
    Int {
        from: i64,
        to:   i64,
    },
    Float {
        from: f64,
        to:   f64,

        #[serde(default = "randgen_float_precision_default")]
        precision: usize,
    },
    Bool {
        prob: f64,
    },
    State,
    Enum {
        values: Vec<String>,
    },
    Timestamp {
        from: NaiveDateTime,
        to:   NaiveDateTime,
    },
}

fn randgen_float_precision_default() -> usize {
    3
}

impl RandGen {
    pub fn gen<R: Rng + ?Sized>(&self, rng: &mut R) -> Box<dyn erased_serde::Serialize> {
        match &self {
            RandGen::Int { from, to } => Box::new(rng.gen_range(*from..=*to)),
            RandGen::Bool { prob } => Box::new(rng.gen_bool(*prob)),
            RandGen::State => Box::new(StateName().fake_with_rng::<String, _>(rng)),
            RandGen::Enum { values } => Box::new(values.choose(rng).cloned()),
            RandGen::Timestamp { from, to } => Box::new(rng.gen_range(from.timestamp()..=to.timestamp())),
            RandGen::Float { from, to, precision } => {
                let x = rng.gen_range(*from..=*to);
                let factor = 10_u64.pow(*precision as u32) as f64;
                Box::new((x * factor).round() / factor)
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub enum EntityIdType {
    Int,
    Md5,
}

pub trait DataIter = Iterator<Item = Vec<Box<dyn erased_serde::Serialize>>>;

impl Recipe {
    pub fn generate_group_data<'a>(
        &'a self,
        rng: &'a mut (impl Rng + ?Sized),
        group: &str,
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

        let data_iter = self.entity.seq_range.clone().map(|i| {
            once(self.entity.seq(i))
                .chain(features.iter().map(|f| f.rand_gen.gen(rng)))
                .collect::<Vec<_>>()
        });
        Ok((header, data_iter))
    }

    pub fn generate_label_data<'a>(
        &'a self,
        rng: &'a mut (impl Rng + ?Sized),
        label: &str,
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

        let data_iter = repeat(()).map(move |_| {
            once(self.entity.rand_id(rng))
                .chain(once(self.entity.rand_ts(rng)))
                .chain(features.iter().map(|f| f.rand_gen.gen(rng)))
                .collect::<Vec<_>>()
        });
        Ok((header, data_iter))
    }
}
