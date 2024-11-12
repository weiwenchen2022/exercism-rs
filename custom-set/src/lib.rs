#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T: Ord + Eq + Clone> {
    // We fake using T here, so the compiler does not complain that
    // "parameter `T` is never used". Delete when no longer needed.
    // phantom: std::marker::PhantomData<T>,
    elements: Vec<T>,
}

impl<T: Ord + Copy> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        let mut elements = input.to_vec();
        elements.sort();
        elements.dedup();
        Self { elements }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.elements.binary_search(element).is_ok()
    }

    pub fn add(&mut self, element: T) {
        if let Err(index) = self.elements.binary_search(&element) {
            self.elements.insert(index, element);
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        if self.elements.len() > other.elements.len() {
            return false;
        }

        self.elements
            .iter()
            .all(|element| other.elements.binary_search(element).is_ok())
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        if self
            .elements
            .iter()
            .any(|element| other.elements.binary_search(element).is_ok())
        {
            return false;
        }
        true
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        Self {
            elements: self
                .elements
                .iter()
                .filter(|&element| other.elements.binary_search(element).is_ok())
                .copied()
                .collect(),
        }
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        Self {
            elements: self
                .elements
                .iter()
                .filter(|&element| other.elements.binary_search(element).is_err())
                .copied()
                .collect(),
        }
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut elements = self.elements.clone();
        elements.extend(other.elements.iter().copied());
        elements.sort();
        elements.dedup();

        Self { elements }
    }
}
