pub mod fraud_detection;

use chrono::{DateTime, TimeZone};
use fake::{Fake, Faker};
use rand::Rng;

pub mod prelude {
    pub use super::fake_feature_group;
    pub use super::fake_feature_label;
    pub use super::FakeFeatureGroup;
    pub use super::FakeFeatureLabel;
}

pub trait FakeFeatureGroup {
    fn fake_with_id<R: Rng + ?Sized>(rng: &mut R, id: usize) -> Self;

    fn fake<R: Rng + ?Sized>(rng: &mut R) -> Self
    where
        Self: Sized,
    {
        let id = Faker.fake_with_rng(rng);
        Self::fake_with_id(rng, id)
    }
}

pub trait FakeFeatureLabel {
    fn fake<R, Tz>(rng: &mut R, id_range: &(usize, usize), tm_range: &(DateTime<Tz>, DateTime<Tz>)) -> Self
    where
        R: Rng + ?Sized,
        Tz: TimeZone;
}

pub fn fake_feature_group<T, R: Rng + ?Sized>(rng: &mut R, id_start: usize) -> impl Iterator<Item = T> + '_
where
    T: FakeFeatureGroup,
{
    (id_start..).map(|id| T::fake_with_id(rng, id))
}

pub fn fake_feature_label<'a, Tz, T, R: Rng + ?Sized>(
    rng: &'a mut R,
    id_range: &'a (usize, usize),
    tm_range: &'a (DateTime<Tz>, DateTime<Tz>),
) -> impl Iterator<Item = T> + 'a
where
    T: FakeFeatureLabel,
    Tz: TimeZone,
{
    (0..).map(|_| T::fake(rng, id_range, tm_range))
}
