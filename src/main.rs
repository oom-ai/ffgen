#![feature(associated_type_defaults)]
#![feature(type_alias_impl_trait)]

mod cli;
mod feature_group;

use crate::feature_group::prelude::*;
use cli::{Opt, Scenario, StructOpt, TypeCommand};
use rand::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let opt: Opt = Opt::from_args();
    match opt {
        Opt::Completion { shell } => {
            Opt::clap().gen_completions_to(env!("CARGO_PKG_NAME"), shell, &mut std::io::stdout());
        }
        Opt::Generate(cmd) => {
            let rng = &mut StdRng::seed_from_u64(0);
            let mut csvw = csv::Writer::from_writer(std::io::stdout());
            match cmd {
                TypeCommand::Group {
                    scenario,
                    name,
                    id_start,
                    limit,
                } => {
                    macro_rules! ffg {
                            ($($p:expr => $e:expr),* $(,)?) => {
                                match name.as_ref() {
                                    $($p => fake_feature_group(&$e, rng, id_start)
                                     .take(limit)
                                     .try_for_each(|x| csvw.serialize(x))?,)*
                                    g => return Err(format!("group '{}' not found", g).into()),
                                }
                            };
                        }
                    match scenario {
                        Scenario::FraudDetection => ffg! {
                            "account"           => fraud_detection::FakeAccount,
                            "transaction_stats" => fraud_detection::FakeTransactionStats,
                        },
                    }
                }
                TypeCommand::Label {
                    scenario,
                    id_start,
                    id_end,
                    time_start,
                    time_end,
                    limit,
                } => match scenario {
                    Scenario::FraudDetection => {
                        use crate::feature_group::fraud_detection::*;
                        fake_feature_label(&FakeLabel, rng, &(id_start, id_end), &(time_start, time_end))
                            .take(limit)
                            .try_for_each(|x| csvw.serialize(x))?
                    }
                },
            }
        }
    }
    Ok(())
}
