pub mod fraud_detection;

use chrono::NaiveDateTime;
use rand::Rng;

pub mod prelude {
    pub use super::fake_feature_group;
    pub use super::fake_feature_label;
    pub use super::fraud_detection;
    pub use super::FakeFeatureGroup;
    pub use super::FakeFeatureLabel;
}

pub trait FakeFeatureGroup {
    type Group;
    fn fake<R: Rng + ?Sized>(&self, rng: &mut R, id: usize) -> Self::Group;
}

pub trait FakeFeatureLabel {
    type Label;
    fn fake<R>(&self, rng: &mut R, id_range: &(usize, usize), tm_range: &(NaiveDateTime, NaiveDateTime)) -> Self::Label
    where
        R: Rng + ?Sized;
}

pub fn fake_feature_group<'a, T, R: Rng + ?Sized>(
    ffg: &'a T,
    rng: &'a mut R,
    id_start: usize,
) -> impl Iterator<Item = T::Group> + 'a
where
    T: FakeFeatureGroup,
{
    (id_start..).map(|id| ffg.fake(rng, id))
}

pub fn fake_feature_label<'a, T, R: Rng + ?Sized>(
    ffl: &'a T,
    rng: &'a mut R,
    id_range: &'a (usize, usize),
    tm_range: &'a (NaiveDateTime, NaiveDateTime),
) -> impl Iterator<Item = T::Label> + 'a
where
    T: FakeFeatureLabel,
{
    (0..).map(|_| ffl.fake(rng, id_range, tm_range))
}
