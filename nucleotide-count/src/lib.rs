use std::collections::HashMap;

const NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !NUCLEOTIDES.contains(&nucleotide) {
        return Err(nucleotide);
    }

    if let Some(ch) = dna
        .bytes()
        .find(|ch| !NUCLEOTIDES.contains(&(ch.to_ascii_uppercase() as char)))
    {
        return Err(ch as char);
    }

    Ok(dna.bytes().filter(|&ch| nucleotide == ch as char).count())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    NUCLEOTIDES
        .iter()
        .map(|&nucleotide| -> Result<(char, usize), char> {
            Ok((nucleotide, count(nucleotide, dna)?))
        })
        .collect()
}
