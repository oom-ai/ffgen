#![feature(concat_idents)]

mod cli;
mod feature_group;

use clap::{IntoApp, Parser};
use clap_generate::Generator;
use cli::*;
use feature_group::prelude::*;
use rand::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let opt = Opt::parse();
    match opt {
        Opt::Completion { shell } => shell.generate(&Opt::into_app(), &mut std::io::stdout()),
        Opt::Generate(GenerateCmd { subcommand, seed }) => {
            let seed = seed.unwrap_or_else(|| chrono::Utc::now().timestamp() as u64);
            let rng = &mut StdRng::seed_from_u64(seed);
            let mut csv_writer = csv::Writer::from_writer(std::io::stdout());

            macro_rules! ffg {
                ($t:ty, $id_range:expr) => {
                    fake_feature_group(rng, $id_range).try_for_each(|x: $t| csv_writer.serialize(x))?
                };
            }
            macro_rules! ffl {
                ($t:ty, $id_range:expr, $tm_range:expr, $limit:expr) => {
                    fake_feature_label(rng, $id_range, $tm_range)
                        .take($limit)
                        .try_for_each(|x: $t| csv_writer.serialize(x))?
                };
            }

            match subcommand {
                CategoryCmd::Group { group } => match group {
                    GroupCmd::FraudDetection { group, id_range } => match group {
                        FraudDetectionGroup::Account => ffg!(fraud_detection::Account, &id_range),
                        FraudDetectionGroup::TransactionStats => ffg!(fraud_detection::TransactionStats, &id_range),
                    },
                },
                CategoryCmd::Label { label } => match label {
                    LabelCmd::FraudDetection {
                        label,
                        id_range,
                        time_range,
                        limit,
                    } => match label {
                        FraudDetectionLabel::Label => ffl!(fraud_detection::Label, &id_range, &time_range, limit),
                    },
                },
            }
        }
    }
    Ok(())
}
