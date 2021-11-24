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
    let seed = opt.seed.unwrap_or_else(|| chrono::Utc::now().timestamp() as u64);
    let rng = &mut StdRng::seed_from_u64(seed);

    match opt.subcommand {
        Subcommand::Group { group, id_range, list } => {
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
            match list {
                true => Group::VARIANTS.iter().for_each(|x| println!("{}", x)),
                false => ffg! {
                    Group::FraudDetectionAccount => fraud_detection::Account,
                    Group::FraudDetectionTransactionStats => fraud_detection::TransactionStats
                },
            }
        }
        Subcommand::Label {
            label,
            id_range,
            time_range,
            limit,
            list,
        } => {
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
            match list {
                true => Label::VARIANTS.iter().for_each(|x| println!("{}", x)),
                false => ffl! {
                    Label::FraudDetectionLabel => fraud_detection::Label,
                },
            }
        }
        Subcommand::Schema { scenario, list } => match list {
            true => Scenario::VARIANTS.iter().for_each(|x| println!("{}", x)),
            false => match scenario {
                Scenario::FraudDetection => println!("{}", fraud_detection::schema()),
            },
        },
        Subcommand::Completion { shell } => {
            let app = &mut Opt::into_app();
            clap_generate::generate(shell, app, app.get_name().to_string(), &mut io::stdout())
        }
    }
    Ok(())
}
