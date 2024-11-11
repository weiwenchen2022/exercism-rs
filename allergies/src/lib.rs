#[derive(Debug)]
pub struct Allergies(u8);

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

fn _f() {
    const _: () = assert!(7 == Allergen::Cats as u8);
}

impl TryFrom<u8> for Allergen {
    type Error = std::convert::Infallible;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Eggs,
            1 => Self::Peanuts,
            2 => Self::Shellfish,
            3 => Self::Strawberries,
            4 => Self::Tomatoes,
            5 => Self::Chocolate,
            6 => Self::Pollen,
            7 => Self::Cats,
            _ => panic!("{} out of range", value),
        })
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let score = score as u8;
        let mut r = 0u8;
        for i in 0..=(Allergen::Cats as u8) {
            r |= score & (0x1 << i);
        }

        Self(r)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        // todo!("Determine if the patient is allergic to the '{allergen:?}' allergen.");
        self.0 & (0x1 << *allergen as u8) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        (0..=Allergen::Cats as u8)
            .filter_map(|i| {
                if self.0 & (0x1 << i) != 0 {
                    Some(i.try_into().unwrap())
                } else {
                    None
                }
            })
            .collect()
    }
}
