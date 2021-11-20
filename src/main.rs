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
        Opt::Generate { scenario, typecommand } => {
            let rng = &mut StdRng::seed_from_u64(0);
            let mut csvw = csv::Writer::from_writer(std::io::stdout());
            match scenario {
                Scenario::fraud_detection => {
                    use crate::feature_group::fraud_detection::*;
                    match typecommand {
                        TypeCommand::Group { group, id_start, limit } => match group.as_ref() {
                            "account" => fake_feature_group(&FakeAccount, rng, id_start)
                                .take(limit)
                                .try_for_each(|x: Account| csvw.serialize(x))?,
                            "transaction_stats" => fake_feature_group(&FakeTransactionStats, rng, id_start)
                                .take(limit)
                                .try_for_each(|x| csvw.serialize(x))?,
                            _ => return Err("group not found".into()),
                        },
                        TypeCommand::Label {
                            id_start,
                            id_end,
                            time_start,
                            time_end,
                            limit,
                        } => fake_feature_label(&FakeLabel, rng, &(id_start, id_end), &(time_start, time_end))
                            .take(limit)
                            .try_for_each(|x| csvw.serialize(x))?,
                    }
                }
            }
        }
    }
    Ok(())
}
