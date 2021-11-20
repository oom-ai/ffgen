mod feature_group;

use crate::feature_group::fraud_detection::{UserAccount, UserLabel, UserTransactionStats};
use crate::feature_group::*;
use chrono::{Duration, TimeZone, Utc};
use rand::{prelude::StdRng, SeedableRng};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let rng = &mut StdRng::seed_from_u64(0);
    {
        let mut csvw = csv::Writer::from_writer(std::io::stdout());
        UserAccount::walk(rng, 1, |x| Ok(csvw.serialize(x)?))?;
    }
    {
        let mut csvw = csv::Writer::from_writer(std::io::stdout());
        UserTransactionStats::walk(rng, 1, |x| Ok(csvw.serialize(x)?))?;
    }
    {
        let mut csvw = csv::Writer::from_writer(std::io::stdout());
        let start = Utc.ymd(2020, 1, 1).and_hms(0, 0, 0);
        let end = start + Duration::days(30);
        UserLabel::walk(rng, 10, &(1..11), &(start..end), |x| Ok(csvw.serialize(x)?))?;
    }
    Ok(())
}
