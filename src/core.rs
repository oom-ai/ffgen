use crate::schema::{RandGen, Schema};
use chrono::NaiveDateTime;
use rand::Rng;
use std::error::Error;
use std::{io, iter::once};

pub fn fake_group<R: Rng + ?Sized, W: io::Write>(
    rng: &mut R,
    schema: &Schema,
    group: &str,
    mut writer: csv::Writer<W>,
) -> Result<(), Box<dyn Error>> {
    let features = &schema
        .groups
        .iter()
        .find(|g| g.name == group)
        .ok_or(format!(
            "group not found int schema. possible_values = {:?}",
            schema.groups.iter().map(|g| &g.name).collect::<Vec<_>>()
        ))?
        .features;

    let header = once(schema.entity.name.as_str())
        .chain(features.iter().map(|f| f.name.as_str()))
        .collect::<Vec<_>>();
    writer.serialize(header)?;

    (schema.entity.from..=schema.entity.to)
        .map(|i| {
            once(schema.entity.seq_gen.gen(i))
                .chain(features.iter().map(|f| f.rand_gen.gen(rng)))
                .collect::<Vec<_>>()
        })
        .try_for_each(|record| writer.serialize(record))?;
    Ok(())
}

pub fn fake_label<R: Rng + ?Sized, W: io::Write>(
    rng: &mut R,
    schema: &Schema,
    label: &str,
    &(from, to): &(NaiveDateTime, NaiveDateTime),
    limit: usize,
    mut writer: csv::Writer<W>,
) -> Result<(), Box<dyn Error>> {
    let features = &schema
        .labels
        .iter()
        .find(|l| l.name == label)
        .ok_or(format!(
            "label not found int schema. possible_values = {:?}",
            schema.labels.iter().map(|g| &g.name).collect::<Vec<_>>()
        ))?
        .features;

    let header = once(schema.entity.name.as_str())
        .chain(once("timestamp"))
        .chain(features.iter().map(|f| f.name.as_str()))
        .collect::<Vec<_>>();
    writer.serialize(header)?;

    let id_gen = RandGen::Int {
        from: schema.entity.from,
        to:   schema.entity.to,
    };
    let ts_gen = RandGen::Timestamp { from, to };

    (0..limit)
        .map(|_| {
            once(id_gen.gen(rng))
                .chain(once(ts_gen.gen(rng)))
                .chain(features.iter().map(|f| f.rand_gen.gen(rng)))
                .collect::<Vec<_>>()
        })
        .try_for_each(|record| writer.serialize(record))?;
    Ok(())
}
