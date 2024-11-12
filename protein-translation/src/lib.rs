pub fn translate(rna: &str) -> Option<Vec<&str>> {
    // todo!("Return a list of protein names that correspond to the '{rna}' RNA string or None if the RNA string is invalid");

    use once_cell::sync::Lazy;
    use std::collections::HashMap;

    static PROTEINS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
        use std::iter;

        [
            (&["AUG"][..], "Methionine"),
            (&["UUU", "UUC"][..], "Phenylalanine"),
            (&["UUA", "UUG"][..], "Leucine"),
            (&["UCU", "UCC", "UCA", "UCG"][..], "Serine"),
            (&["UAU", "UAC"][..], "Tyrosine"),
            (&["UGU", "UGC"][..], "Cysteine"),
            (&["UGG"][..], "Tryptophan"),
            (&["UAA", "UAG", "UGA"][..], "STOP"),
        ]
        .into_iter()
        .flat_map(|(codons, protein)| codons.iter().copied().zip(iter::repeat(protein)))
        .collect()
    });

    rna.as_bytes()
        .chunks(3)
        .map(|codon| {
            if codon.len() != 3 {
                None
            } else {
                let codon = std::str::from_utf8(codon).unwrap();
                PROTEINS.get(&codon).copied()
            }
        })
        .take_while(|protein| !matches!(protein, Some("STOP")))
        .collect()
}
