use super::*;
use fake::{
    faker::{address::en::StateName, boolean::en::Boolean},
    Fake,
};
use rand::Rng;
use serde::Serialize;
use std::ops::Range;

#[derive(Debug, Serialize)]
pub struct UserAccount {
    user:              usize,
    state:             String,
    credit_score:      u16,
    account_age_days:  u16,
    has_2fa_installed: bool,
}

#[derive(Debug, Serialize)]
pub struct UserTransactionStats {
    user:                    usize,
    transaction_account_7d:  u16,
    transaction_account_30d: u16,
}

#[derive(Debug, Serialize)]
pub struct UserLabel {
    user:      usize,
    timestamp: i64,
}

impl FakeFeatureGroup for UserAccount {
    fn fake_with_id<R: Rng + ?Sized>(rng: &mut R, id: usize) -> Self {
        Self {
            user:              id,
            state:             StateName().fake_with_rng(rng),
            credit_score:      (500..750).fake_with_rng(rng),
            account_age_days:  (1..3650).fake_with_rng(rng),
            has_2fa_installed: Boolean(50).fake_with_rng(rng),
        }
    }
}

impl FakeFeatureGroup for UserTransactionStats {
    fn fake_with_id<R: Rng + ?Sized>(rng: &mut R, id: usize) -> Self {
        let transaction_account_7d = (0..10).fake_with_rng(rng);
        let transaction_account_30d = (transaction_account_7d..50).fake_with_rng(rng);
        Self {
            user: id,
            transaction_account_7d,
            transaction_account_30d,
        }
    }
}

impl FakeFeatureLabel for UserLabel {
    fn fake<R, Tz>(rng: &mut R, id_range: &Range<usize>, time_range: &Range<DateTime<Tz>>) -> Self
    where
        R: Rng + ?Sized,
        Tz: TimeZone,
    {
        Self {
            user:      id_range.fake_with_rng(rng),
            timestamp: (time_range.start.timestamp()..time_range.end.timestamp()).fake_with_rng(rng),
        }
    }
}
