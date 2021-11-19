mod fake_feature_group;
mod fraud_detection;

use crate::fake_feature_group::FakeFeatureGroup;
use crate::fraud_detection::UserAccount;

use rand::{prelude::StdRng, SeedableRng};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let rng = &mut StdRng::seed_from_u64(0);
    let mut csvw = csv::Writer::from_writer(std::io::stdout());
    UserAccount::walk(rng, 1, |x| Ok(csvw.serialize(x)?))?;

    Ok(())
}
