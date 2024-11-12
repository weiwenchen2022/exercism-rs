#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Self, usize> {
        if let Some(index) = dna
            .chars()
            .position(|nucleotide| !['A', 'C', 'G', 'T'].contains(&nucleotide.to_ascii_uppercase()))
        {
            return Err(index);
        }

        Ok(Self(dna.to_string()))
    }

    pub fn into_rna(self) -> Rna {
        Rna::new(
            &self
                .0
                .chars()
                .map(|nucleotide| match nucleotide {
                    'G' | 'g' => 'C',
                    'C' | 'c' => 'G',
                    'T' | 't' => 'A',
                    'A' | 'a' => 'U',
                    n => unreachable!("{n:?}"),
                })
                .collect::<String>(),
        )
        .unwrap()
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Self, usize> {
        if let Some(index) = rna
            .chars()
            .position(|nucleotide| !['A', 'C', 'G', 'U'].contains(&nucleotide.to_ascii_uppercase()))
        {
            return Err(index);
        }

        Ok(Self(rna.to_string()))
    }
}
