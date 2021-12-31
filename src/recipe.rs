use anyhow::{anyhow, Error, Result};
use chrono::{DateTime, Utc};
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub name:        String,
    pub description: Option<String>,

    #[serde(default = "default_entity_id_range")]
    pub id_range: RangeInclusive<i64>,

    #[serde(default = "default_entity_time_range")]
    pub time_range: RangeInclusive<DateTime<Utc>>,

    #[serde(default = "entity_id_type_default")]
    pub id_type: EntityIdType,
}

fn default_entity_id_range() -> RangeInclusive<i64> {
    1..=10
}
fn default_entity_time_range() -> RangeInclusive<DateTime<Utc>> {
    let end = chrono::Utc::now();
    let start = end - chrono::Duration::days(1);
    start..=end
}
fn entity_id_type_default() -> EntityIdType {
    EntityIdType::Int
}

impl Entity {
    pub fn len(&self) -> usize {
        match self.id_type {
            EntityIdType::Int => self.id_range.end().to_string().len(),
            EntityIdType::Md5 => 32,
        }
    }

    pub fn id(&self, value: i64) -> Box<dyn erased_serde::Serialize> {
        match self.id_type {
            EntityIdType::Int => Box::new(value),
            EntityIdType::Md5 => {
                use md5::Digest;
                Box::new(format!("{:x}", md5::Md5::digest(&value.to_be_bytes())))
            }
        }
    }

    pub fn rand_id<R: Rng + ?Sized>(&self, rng: &mut R) -> Box<dyn erased_serde::Serialize> {
        Box::new(self.id(rng.gen_range(self.id_range.clone())))
    }

    pub fn rand_ts<R: Rng + ?Sized>(&self, rng: &mut R) -> Box<dyn erased_serde::Serialize> {
        let start = self.time_range.start().timestamp_millis();
        let end = self.time_range.end().timestamp_millis();
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
    Int(RangeInclusive<i64>),
    Float {
        #[serde(flatten)]
        range: RangeInclusive<f64>,

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
    Timestamp(RangeInclusive<DateTime<Utc>>),
    DateTime(RangeInclusive<DateTime<Utc>>),
}

fn randgen_float_precision_default() -> usize {
    3
}

impl RandGen {
    pub fn gen<R: Rng + ?Sized>(&self, rng: &mut R) -> Box<dyn erased_serde::Serialize> {
        match &self {
            RandGen::Int(range) => Box::new(rng.gen_range(range.clone())),
            RandGen::Bool { prob } => Box::new(rng.gen_bool(*prob)),
            RandGen::State => Box::new(StateName().fake_with_rng::<String, _>(rng)),
            RandGen::Enum { values } => Box::new(values.choose(rng).cloned()),
            RandGen::Float { range, precision } => {
                let x = rng.gen_range(range.clone());
                let factor = 10_u64.pow(*precision as u32) as f64;
                Box::new((x * factor).round() / factor)
            }
            RandGen::Timestamp(range) =>
                Box::new(rng.gen_range(range.start().timestamp_millis()..=range.end().timestamp_millis())),
            RandGen::DateTime(range) => {
                let diff = *range.end() - *range.start();
                let x = rng.gen_range(0..diff.num_milliseconds());
                Box::new(*range.start() + chrono::Duration::milliseconds(x))
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
        group: Option<&str>,
    ) -> Result<(Vec<&str>, impl DataIter + 'a)> {
        let group = match group {
            Some(group) => self.groups.iter().find(|g| g.name == group),
            None => self.groups.get(0),
        };
        let features = &group.ok_or_else(|| error_group_not_found(&self.groups))?.features;
        let entity = &self.entity;

        let header = once(entity.name.as_str())
            .chain(features.iter().map(|f| f.name.as_str()))
            .collect::<Vec<_>>();

        let data_iter = entity.id_range.clone().map(|i| {
            once(entity.id(i))
                .chain(features.iter().map(|f| f.rand_gen.gen(rng)))
                .collect::<Vec<_>>()
        });
        Ok((header, data_iter))
    }

    pub fn generate_label_data<'a>(
        &'a self,
        rng: &'a mut (impl Rng + ?Sized),
        group: Option<&str>,
    ) -> Result<(Vec<&str>, impl DataIter + 'a)> {
        let groups = &self.groups;
        let features = match group {
            Some(group) =>
                &groups
                    .iter()
                    .find(|g| g.name == group)
                    .ok_or_else(|| error_group_not_found(groups))?
                    .features,
            None => <&[Feature]>::default(),
        };
        let entity = &self.entity;

        let header = once(entity.name.as_str())
            .chain(once("timestamp"))
            .chain(features.iter().map(|f| f.name.as_str()))
            .collect::<Vec<_>>();

        let data_iter = repeat(()).map(move |_| {
            once(entity.rand_id(rng))
                .chain(once(entity.rand_ts(rng)))
                .chain(features.iter().map(|f| f.rand_gen.gen(rng)))
                .collect::<Vec<_>>()
        });
        Ok((header, data_iter))
    }
}

fn error_group_not_found(groups: &[Group]) -> Error {
    anyhow!(
        "group not found in recipe. possible_values = {:?}",
        groups.iter().map(|g| &g.name).collect::<Vec<_>>()
    )
}
