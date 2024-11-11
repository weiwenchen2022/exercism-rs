pub fn raindrops(n: u32) -> String {
    use std::iter::{once, repeat};

    let plings = repeat("").take(2).chain(once("Pling")).cycle();
    let plangs = repeat("").take(4).chain(once("Plang")).cycle();
    let plongs = repeat("").take(6).chain(once("Plong")).cycle();
    let plings_plangs_plongs = plings.zip(plangs).zip(plongs);

    (1..=n)
        .zip(plings_plangs_plongs)
        .map(|tuple| match tuple {
            (i, (("", ""), "")) => i.to_string(),
            (_, ((pling, plang), plong)) => format!("{}{}{}", pling, plang, plong),
        })
        .last()
        .unwrap()
}
