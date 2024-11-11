#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    use std::cmp::Ordering;

    match first_list.len().cmp(&second_list.len()) {
        Ordering::Equal => {
            let all_equal = first_list
                .iter()
                .zip(second_list)
                .all(|(first, second)| first == second);
            if all_equal {
                return Comparison::Equal;
            }
        }
        Ordering::Greater => {
            let mut all_contained = second_list.is_empty();
            for i in 0..=first_list.len() - second_list.len() {
                if second_list == &first_list[i..i + second_list.len()] {
                    all_contained = true;
                    break;
                }
            }
            if all_contained {
                return Comparison::Superlist;
            }
        }

        Ordering::Less => {
            return match sublist(second_list, first_list) {
                Comparison::Superlist => Comparison::Sublist,
                Comparison::Unequal => Comparison::Unequal,
                result @ (Comparison::Sublist | Comparison::Equal) => unreachable!("{result:?}"),
            }
        }
    }

    Comparison::Unequal
}
