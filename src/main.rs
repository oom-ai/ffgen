#![feature(concat_idents)]

mod cli;
mod feature_group;

use crate::feature_group::prelude::*;
use cli::{CategoryCommand, FraudDetectionGroup, GroupCommand, Opt, StructOpt};
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
                CategoryCommand::Group { group } => match group {
                    GroupCommand::FraudDetection { group, id_start, limit } => match group {
                        FraudDetectionGroup::Account(g) => fake_feature_group(&g, rng, id_start)
                            .take(limit)
                            .try_for_each(|x| csvw.serialize(x))?,
                        FraudDetectionGroup::TransactionStats(g) => fake_feature_group(&g, rng, id_start)
                            .take(limit)
                            .try_for_each(|x| csvw.serialize(x))?,
                    },
                },
                CategoryCommand::Label { label } => match label {
                    cli::LabelCommand::FraudDetection {
                        label,
                        id_start,
                        id_end,
                        time_start,
                        time_end,
                        limit,
                    } => match label {
                        cli::FraudDetectionLabel::Label(l) => {
                            fake_feature_label(&l, rng, &(id_start, id_end), &(time_start, time_end))
                                .take(limit)
                                .try_for_each(|x| csvw.serialize(x))?
                        }
                    },
                },
            }
        }
    }
    Ok(())
}
