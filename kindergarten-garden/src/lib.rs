pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    // todo!("based on the {diagram}, determine the plants the {student} is responsible for");

    let diagram = diagram.split("\n").map(str::as_bytes).collect::<Vec<_>>();
    assert_eq!(2, diagram.len());

    let mut stutents = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ];
    stutents.sort();

    let index = stutents.binary_search(&student).unwrap();

    ([
        diagram[0][index * 2].try_into().unwrap(),
        diagram[0][index * 2 + 1].try_into().unwrap(),
        diagram[1][index * 2].try_into().unwrap(),
        diagram[1][index * 2 + 1].try_into().unwrap(),
    ] as [Plant; 4])
        .iter()
        .map(Into::into)
        .collect()
}

#[derive(Debug)]
enum Plant {
    Grass,
    Clover,
    Radish,
    Violet,
}

impl From<&Plant> for &'static str {
    fn from(plant: &Plant) -> Self {
        match plant {
            Plant::Grass => "grass",
            Plant::Clover => "clover",
            Plant::Radish => "radishes",
            Plant::Violet => "violets",
        }
    }
}

impl TryFrom<u8> for Plant {
    type Error = std::convert::Infallible;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b'G' => Self::Grass,
            b'C' => Self::Clover,
            b'R' => Self::Radish,
            b'V' => Self::Violet,
            _ => panic!("{} out of range", value),
        })
    }
}
