use crate::schema::{RandGen, Schema};
use anyhow::anyhow;
use chrono::NaiveDateTime;
use rand::Rng;
use std::{io, iter::once};

pub fn generate_group_data(
    rng: &mut (impl Rng + ?Sized),
    schema: &Schema,
    group: &str,
    id_range: Option<&(i64, i64)>,
    wtr: impl io::Write,
) -> anyhow::Result<()> {
    let features = &schema
        .groups
        .iter()
        .find(|g| g.name == group)
        .ok_or_else(|| {
            anyhow!(
                "group not found int schema. possible_values = {:?}",
                schema.groups.iter().map(|g| &g.name).collect::<Vec<_>>()
            )
        })?
        .features;

    let header = once(schema.entity.name.as_str())
        .chain(features.iter().map(|f| f.name.as_str()))
        .collect::<Vec<_>>();

    let mut wtr = csv::Writer::from_writer(wtr);
    wtr.serialize(header)?;

    let id_range = match id_range {
        Some(&(start, end)) => start..=end,
        None => schema.entity.from..=schema.entity.to,
    };

    id_range
        .map(|i| {
            once(schema.entity.seq_gen.gen(i))
                .chain(features.iter().map(|f| f.rand_gen.gen(rng)))
                .collect::<Vec<_>>()
        })
        .try_for_each(|record| wtr.serialize(record))?;
    Ok(())
}

pub fn generate_label_data(
    rng: &mut (impl Rng + ?Sized),
    schema: &Schema,
    label: &str,
    time_range: &(NaiveDateTime, NaiveDateTime),
    id_range: Option<&(i64, i64)>,
    limit: usize,
    wtr: impl io::Write,
) -> anyhow::Result<()> {
    let features = &schema
        .labels
        .iter()
        .find(|l| l.name == label)
        .ok_or_else(|| {
            anyhow!(
                "label not found int schema. possible_values = {:?}",
                schema.labels.iter().map(|g| &g.name).collect::<Vec<_>>()
            )
        })?
        .features;

    let header = once(schema.entity.name.as_str())
        .chain(once("timestamp"))
        .chain(features.iter().map(|f| f.name.as_str()))
        .collect::<Vec<_>>();

    let mut wtr = csv::Writer::from_writer(wtr);
    wtr.serialize(header)?;

    let (id_from, id_to) = match id_range {
        Some(&(start, end)) => (start, end),
        None => (schema.entity.from, schema.entity.to),
    };
    let id_gen = RandGen::Int { from: id_from, to: id_to };

    let &(tm_from, tm_to) = time_range;
    let ts_gen = RandGen::Timestamp { from: tm_from, to: tm_to };

    (0..limit)
        .map(|_| {
            once(id_gen.gen(rng))
                .chain(once(ts_gen.gen(rng)))
                .chain(features.iter().map(|f| f.rand_gen.gen(rng)))
                .collect::<Vec<_>>()
        })
        .try_for_each(|record| wtr.serialize(record))?;
    Ok(())
}
