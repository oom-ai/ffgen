mod cli;
mod feature_group;

use clap::{IntoApp, Parser};
use cli::*;
use feature_group::prelude::*;
use rand::prelude::*;
use std::io;
use strum::VariantNames;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::parse();
    let mut csv_writer = csv::Writer::from_writer(io::stdout());

    match opt {
        Opt::Group { group, id_range, seed } => {
            let seed = seed.unwrap_or_else(|| chrono::Utc::now().timestamp() as u64);
            let rng = &mut StdRng::seed_from_u64(seed);
            macro_rules! ffg {
                ($($p:pat => $t:ty),+ $(,)?) => {
                    match group {
                        $(
                            $p => fake_feature_group(rng, &id_range)
                                    .try_for_each(|x: $t| csv_writer.serialize(x))?,
                        )*
                    }
                }
            }
            ffg! {
                Group::FraudDetectionAccount => fraud_detection::Account,
                Group::FraudDetectionTransactionStats => fraud_detection::TransactionStats
            }
        }
        Opt::Label {
            label,
            id_range,
            time_range,
            limit,
            seed,
        } => {
            let seed = seed.unwrap_or_else(|| chrono::Utc::now().timestamp() as u64);
            let rng = &mut StdRng::seed_from_u64(seed);
            macro_rules! ffl {
                ($($p:pat => $t:ty),+ $(,)?) => {
                    match label {
                        $(
                            $p => fake_feature_label(rng, &id_range, &time_range)
                                    .take(limit)
                                    .try_for_each(|x: $t| csv_writer.serialize(x))?
                        )*
                    }
                }
            }
            ffl! {
                Label::FraudDetectionLabel => fraud_detection::Label,
            }
        }
        Opt::List { category } => match category {
            Category::Group => Group::VARIANTS.iter().for_each(|x| println!("{}", x)),
            Category::Label => Label::VARIANTS.iter().for_each(|x| println!("{}", x)),
        },
        Opt::Completion { shell } => {
            let app = &mut Opt::into_app();
            clap_generate::generate(shell, app, app.get_name().to_string(), &mut io::stdout())
        }
    }
    Ok(())
}
