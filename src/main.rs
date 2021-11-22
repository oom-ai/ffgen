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
        Opt::Completion { shell } => shell.generate(&mut Opt::into_app(), &mut std::io::stdout()),
        Opt::Generate(GenerateCmd { subcommand, seed }) => {
            let seed = seed.unwrap_or_else(|| chrono::Utc::now().timestamp() as u64);
            let rng = &mut StdRng::seed_from_u64(seed);
            let mut csvw = csv::Writer::from_writer(std::io::stdout());

            match subcommand {
                CategoryCmd::Group { group } => match group {
                    GroupCmd::FraudDetection { group, id_range } => match group {
                        FraudDetectionGroup::Account(g) => {
                            fake_feature_group(&g, rng, &id_range).try_for_each(|x| csvw.serialize(x))?
                        }
                        FraudDetectionGroup::TransactionStats(g) => {
                            fake_feature_group(&g, rng, &id_range).try_for_each(|x| csvw.serialize(x))?
                        }
                    },
                },
                CategoryCmd::Label { label } => match label {
                    LabelCmd::FraudDetection {
                        label,
                        id_range,
                        time_range,
                        limit,
                    } => match label {
                        FraudDetectionLabel::Label(l) => fake_feature_label(&l, rng, &id_range, &time_range)
                            .take(limit)
                            .try_for_each(|x| csvw.serialize(x))?,
                    },
                },
            }
        }
    }
    Ok(())
}
