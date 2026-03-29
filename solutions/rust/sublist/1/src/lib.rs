#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    }
    if first_list.len() > second_list.len() && is_sublist(second_list, first_list) {
        return Comparison::Superlist;
    }
    if second_list.len() > first_list.len() && is_sublist(first_list, second_list) {
        return Comparison::Sublist;
    }
    Comparison::Unequal
}

fn is_sublist(contained_list: &[i32], containing_list: &[i32]) -> bool {
    for i in 0..=containing_list.len() - contained_list.len() {
        if &containing_list[i..i + contained_list.len()] == contained_list {
            return true;
        }
    }
    false
}
