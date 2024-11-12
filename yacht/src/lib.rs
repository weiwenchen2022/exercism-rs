#[derive(Debug)]
pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];

pub fn score(dice: Dice, category: Category) -> u8 {
    // todo!("determine the score for {dice:?} in the {category:?}");

    match category {
        Category::Ones => dice.iter().filter(|&&d| 1 == d).count() as u8,
        Category::Twos => dice.iter().filter(|&&d| 2 == d).count() as u8 * 2,
        Category::Threes => dice.iter().filter(|&&d| 3 == d).count() as u8 * 3,
        Category::Fours => dice.iter().filter(|&&d| 4 == d).count() as u8 * 4,
        Category::Fives => dice.iter().filter(|&&d| 5 == d).count() as u8 * 5,
        Category::Sixes => dice.iter().filter(|&&d| 6 == d).count() as u8 * 6,

        Category::FullHouse => {
            use std::collections::HashMap;

            let mut times = HashMap::new();
            dice.iter().for_each(|&d| *times.entry(d).or_default() += 1);

            if times.len() == 2 && times.values().max().unwrap() == &3 {
                times.iter().fold(0, |score, (d, c)| score + d * c)
            } else {
                0
            }
        }

        Category::FourOfAKind => {
            use std::collections::HashMap;

            let mut times = HashMap::new();
            dice.iter().for_each(|&d| *times.entry(d).or_default() += 1);

            if times.len() == 1 {
                (times.values().next().unwrap() - 1) * times.keys().next().unwrap()
            } else if times.len() == 2 {
                let p = times.iter().max_by_key(|&(_, &v)| v).unwrap();
                if &4 == p.1 {
                    p.0 * p.1
                } else {
                    0
                }
            } else {
                0
            }
        }

        Category::LittleStraight => {
            let mut dice = dice;
            dice.sort();

            if [1, 2, 3, 4, 5] == dice {
                30
            } else {
                0
            }
        }
        Category::BigStraight => {
            let mut dice = dice;
            dice.sort();

            if [2, 3, 4, 5, 6] == dice {
                30
            } else {
                0
            }
        }

        Category::Choice => dice.iter().sum(),

        Category::Yacht => {
            use std::collections::HashSet;

            let set = dice.iter().copied().collect::<HashSet<_>>();
            if set.len() == 1 {
                50
            } else {
                0
            }
        }
    }
}
