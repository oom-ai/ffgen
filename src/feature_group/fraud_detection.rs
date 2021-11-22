use super::*;
use fake::{
    faker::{address::en::StateName, boolean::en::Boolean},
    Fake,
};
use rand::Rng;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Account {
    user:              usize,
    state:             String,
    credit_score:      u16,
    account_age_days:  u16,
    has_2fa_installed: bool,
}

#[derive(Debug, Serialize)]
pub struct TransactionStats {
    user:                  usize,
    transaction_count_7d:  u16,
    transaction_count_30d: u16,
}

#[derive(Debug, Serialize)]
pub struct Label {
    user:      usize,
    timestamp: i64,
}

impl FakeFeatureGroup for Account {
    fn fake<R: Rng + ?Sized>(rng: &mut R, id: usize) -> Self {
        Account {
            user:              id,
            state:             StateName().fake_with_rng(rng),
            credit_score:      (500..750).fake_with_rng(rng),
            account_age_days:  (1..365).fake_with_rng(rng),
            has_2fa_installed: Boolean(50).fake_with_rng(rng),
        }
    }
}

impl FakeFeatureGroup for TransactionStats {
    fn fake<R: Rng + ?Sized>(rng: &mut R, id: usize) -> Self {
        let transaction_account_7d = (0..10).fake_with_rng(rng);
        let transaction_account_30d = (transaction_account_7d..50).fake_with_rng(rng);
        TransactionStats {
            user:                  id,
            transaction_count_7d:  transaction_account_7d,
            transaction_count_30d: transaction_account_30d,
        }
    }
}

impl FakeFeatureLabel for Label {
    fn fake<R>(
        rng: &mut R,
        (id_start, id_end): &(usize, usize),
        (tm_start, tm_end): &(NaiveDateTime, NaiveDateTime),
    ) -> Self
    where
        R: Rng + ?Sized,
    {
        Label {
            user:      (*id_start..*id_end).fake_with_rng(rng),
            timestamp: (tm_start.timestamp()..tm_end.timestamp()).fake_with_rng(rng),
        }
    }
}
