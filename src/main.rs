#![feature(concat_idents)]

mod cli;
mod feature_group;

use cli::*;
use feature_group::prelude::*;
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
                CategoryCmd::Group { group } => match group {
                    GroupCmd::FraudDetection {
                        group,
                        id_range: (id_start, id_end),
                    } => match group {
                        FraudDetectionGroup::Account(g) => fake_feature_group(&g, rng, id_start)
                            .take(id_end - id_start + 1)
                            .try_for_each(|x| csvw.serialize(x))?,
                        FraudDetectionGroup::TransactionStats(g) => fake_feature_group(&g, rng, id_start)
                            .take(id_end - id_start + 1)
                            .try_for_each(|x| csvw.serialize(x))?,
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
