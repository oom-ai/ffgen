mod feature_group;

use crate::feature_group::{fraud_detection::*, prelude::*};
use chrono::{Duration, TimeZone, Utc};
use rand::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let rng = &mut StdRng::seed_from_u64(0);
    {
        let mut csvw = csv::Writer::from_writer(std::io::stdout());
        fake_feature_group(rng, 1000)
            .take(10)
            .try_for_each(|x: UserAccount| csvw.serialize(x))?;
    }
    {
        let mut csvw = csv::Writer::from_writer(std::io::stdout());
        fake_feature_group(rng, 1000)
            .take(10)
            .try_for_each(|x: UserTransactionStats| csvw.serialize(x))?;
    }
    {
        let mut csvw = csv::Writer::from_writer(std::io::stdout());
        let start = Utc.ymd(2020, 1, 1).and_hms(0, 0, 0);
        let end = start + Duration::days(30);
        fake_feature_label(rng, &(1, 10), &(start, end))
            .take(10)
            .try_for_each(|x: UserLabel| csvw.serialize(x))?;
    }
    Ok(())
}
