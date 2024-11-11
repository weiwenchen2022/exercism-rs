// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration(pub f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        // todo!("s, measured in seconds: {s}")
        Self(s as f64)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

const EARTH_YEAR_SECONDS: f64 = 31_557_600.0;

const MERCURY_ORBITAL_PERIOD: f64 = 0.2408467;
const VENUS_ORBITAL_PERIOD: f64 = 0.61519726;
const EARTH_ORBITAL_PERIOD: f64 = 1.0;
const MARS_ORBITAL_PERIOD: f64 = 1.8808158;
const JUPITER_ORBITAL_PERIOD: f64 = 11.862615;
const SATURN_ORBITAL_PERIOD: f64 = 29.447498;
const URANUS_ORBITAL_PERIOD: f64 = 84.016846;
const NEPTUNE_ORBITAL_PERIOD: f64 = 164.79132;

macro_rules! impl_planet_for {
    () => {};
    ($(($t:ty, $period:expr)),+ $(,)?) => {
        $(
            impl Planet for $t {
                fn years_during(d: &Duration) -> f64 {
                    d.0 / EARTH_YEAR_SECONDS / $period
                }
            }
        )*
    };
}

impl_planet_for!(
    (Mercury, MERCURY_ORBITAL_PERIOD),
    (Venus, VENUS_ORBITAL_PERIOD),
    (Earth, EARTH_ORBITAL_PERIOD),
    (Mars, MARS_ORBITAL_PERIOD),
    (Jupiter, JUPITER_ORBITAL_PERIOD),
    (Saturn, SATURN_ORBITAL_PERIOD),
    (Uranus, URANUS_ORBITAL_PERIOD),
    (Neptune, NEPTUNE_ORBITAL_PERIOD),
);
