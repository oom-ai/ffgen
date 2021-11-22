pub mod fraud_detection;

use chrono::NaiveDateTime;
use rand::Rng;

pub mod prelude {
    pub use super::fake_feature_group;
    pub use super::fake_feature_label;
    pub use super::FakeFeatureGroup;
    pub use super::FakeFeatureLabel;

    pub use super::fraud_detection;
}

pub trait FakeFeatureLabel {
    fn fake<R: Rng + ?Sized>(rng: &mut R, id_range: &(usize, usize), tm_range: &(NaiveDateTime, NaiveDateTime))
        -> Self;
}
pub trait FakeFeatureGroup {
    fn fake<R: Rng + ?Sized>(rng: &mut R, id: usize) -> Self;
}

pub fn fake_feature_label<'a, T, R: Rng + ?Sized>(
    rng: &'a mut R,
    id_range: &'a (usize, usize),
    tm_range: &'a (NaiveDateTime, NaiveDateTime),
) -> impl Iterator<Item = T> + 'a
where
    T: FakeFeatureLabel,
{
    (0..).map(|_| T::fake(rng, id_range, tm_range))
}

pub fn fake_feature_group<'a, T, R: Rng + ?Sized>(
    rng: &'a mut R,
    &(id_start, id_end): &'a (usize, usize),
) -> impl Iterator<Item = T> + 'a
where
    T: FakeFeatureGroup,
{
    (id_start..=id_end).map(|id| T::fake(rng, id))
}
