mod cli;
mod feature_group;

use crate::feature_group::{fraud_detection::*, prelude::*};
use cli::{CompletionOpt, Opt, ScenarioName, StructOpt, Subcommand};
use rand::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let opt: Opt = Opt::from_args();
    match opt.subcommand {
        Subcommand::Completion(CompletionOpt { shell }) => {
            Opt::clap().gen_completions_to(env!("CARGO_PKG_NAME"), shell, &mut std::io::stdout());
        }
        Subcommand::Scenario(sopt) => {
            let rng = &mut StdRng::seed_from_u64(0);
            let mut csvw = csv::Writer::from_writer(std::io::stdout());
            match sopt.scenario {
                ScenarioName::fraud_detection => match sopt.group.as_ref() {
                    "user_account" => fake_feature_group(rng, sopt.id_start)
                        .take(sopt.limit)
                        .try_for_each(|x: UserAccount| csvw.serialize(x))?,
                    "label" => {
                        fake_feature_label(rng, &(sopt.id_start, sopt.id_end), &(sopt.time_start, sopt.time_end))
                            .take(sopt.limit)
                            .try_for_each(|x: Label| csvw.serialize(x))?
                    }
                    _ => return Err("group not found".into()),
                },
            }
        }
    }
    Ok(())
}
