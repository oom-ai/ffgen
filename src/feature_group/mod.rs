pub mod fraud_detection;

use chrono::{DateTime, TimeZone};
use fake::{Fake, Faker};
use rand::Rng;
use std::ops::Range;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait FakeFeatureGroup {
    fn fake_with_id<R: Rng + ?Sized>(rng: &mut R, id: usize) -> Self;

    fn fake<R: Rng + ?Sized>(rng: &mut R) -> Self
    where
        Self: Sized,
    {
        let id = Faker.fake_with_rng(rng);
        Self::fake_with_id(rng, id)
    }

    fn walk<R, F>(rng: &mut R, size: usize, mut f: F) -> Result<()>
    where
        R: Rng + ?Sized,
        F: FnMut(Box<Self>) -> Result<()>,
        Self: Sized,
    {
        for i in 1..=size {
            f(Box::new(Self::fake_with_id(rng, i)))?;
        }
        Ok(())
    }
}

pub trait FakeFeatureLabel {
    fn fake<R, Tz>(rng: &mut R, id_range: &Range<usize>, time_range: &Range<DateTime<Tz>>) -> Self
    where
        R: Rng + ?Sized,
        Tz: TimeZone;

    fn walk<R, F, Tz>(
        rng: &mut R,
        size: usize,
        id_range: &Range<usize>,
        time_range: &Range<DateTime<Tz>>,
        mut f: F,
    ) -> Result<()>
    where
        R: Rng + ?Sized,
        F: FnMut(Box<Self>) -> Result<()>,
        Tz: TimeZone,
        Self: Sized,
    {
        for _ in 1..=size {
            f(Box::new(Self::fake(rng, id_range, time_range)))?;
        }
        Ok(())
    }
}
