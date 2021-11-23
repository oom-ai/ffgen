mod cli;
mod feature_group;

use clap::{IntoApp, Parser};
use cli::*;
use feature_group::prelude::*;
use rand::prelude::*;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::parse();
    let seed = opt.seed.unwrap_or_else(|| chrono::Utc::now().timestamp() as u64);
    let rng = &mut StdRng::seed_from_u64(seed);
    let mut csv_writer = csv::Writer::from_writer(io::stdout());

    match opt.subcommand {
        Subcommand::Group { group, id_range } => {
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
        Subcommand::Label {
            label,
            id_range,
            time_range,
            limit,
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
            ffl! {
                Label::FraudDetectionLabel => fraud_detection::Label,
            }
        }
        Subcommand::Completion { shell } => {
            let app = &mut Opt::into_app();
            clap_generate::generate(shell, app, app.get_name().to_string(), &mut io::stdout())
        }
    }
    Ok(())
}
