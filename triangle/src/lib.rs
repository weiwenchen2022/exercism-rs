pub struct Triangle<T> {
    pub sides: [T; 3],
}

impl<T> Triangle<T>
where
    T: PartialEq + Default + std::ops::Add<Output = T> + PartialOrd + Copy,
{
    pub fn build(sides: [T; 3]) -> Option<Self> {
        if sides.iter().any(|side| &T::default() == side) {
            return None;
        }

        if sides[0] + sides[1] < sides[2] {
            return None;
        }

        if sides[1] + sides[2] < sides[0] {
            return None;
        }

        if sides[0] + sides[2] < sides[1] {
            return None;
        }

        Some(Self { sides })
    }

    pub fn is_equilateral(&self) -> bool {
        let a = self.sides.first().unwrap();
        self.sides.iter().skip(1).all(|side| a == side)
    }

    pub fn is_scalene(&self) -> bool {
        for (i, a) in self.sides.iter().enumerate() {
            if self
                .sides
                .iter()
                .enumerate()
                .filter_map(|(j, side)| if i != j { Some(side) } else { None })
                .any(|b| a == b)
            {
                return false;
            }
        }
        true
    }

    pub fn is_isosceles(&self) -> bool {
        for (i, a) in self.sides.iter().enumerate() {
            if self
                .sides
                .iter()
                .enumerate()
                .filter_map(|(j, b)| if i != j { Some(b) } else { None })
                .any(|b| a == b)
            {
                return true;
            }
        }
        false
    }
}
