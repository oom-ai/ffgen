use fake::{Fake, Faker};
use rand::Rng;

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
