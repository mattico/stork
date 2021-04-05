use std::{convert::TryInto, slice::Iter};
use Transformation::*;
#[derive(Debug, Copy, Clone, PartialEq)]
enum Transformation {
    Noop,
    Insert,
    Delete,
    Replace,
}

impl Transformation {
    pub fn iterator() -> Iter<'static, Transformation> {
        static DIRECTIONS: [Transformation; 4] = [Noop, Insert, Delete, Replace];
        DIRECTIONS.iter()
    }
}

fn get_distance(substring: &Vec<Transformation>) -> u8 {
    substring
        .iter()
        .filter(|t| **t != Transformation::Noop)
        .collect::<Vec<&Transformation>>()
        .len() as u8
}

fn replace_in_vec(vector: &mut Vec<Transformation>, offset: usize, replacement: Transformation) {
    let option_with_reference = vector.get_mut(offset);
    if let Some(value) = option_with_reference {
        *value = replacement
    }
}

fn r_add_transformation(
    substring: Vec<Transformation>,
    offset: usize,
    thresh: u8,
    collection: &mut Vec<Vec<Transformation>>,
) {
    if get_distance(&substring) == thresh || &offset == &substring.len() {
        println!("24 {} - {:?}", get_distance(&substring), substring);
        collection.push(substring);
        return;
    }

    for t in Transformation::iterator() {
        let mut new_substring = substring.clone();
        replace_in_vec(&mut new_substring, offset, *t);
        r_add_transformation(new_substring, offset + 1, thresh, collection);
    }
}

fn generate_transformations(len: usize, thresh: u8) -> Vec<Vec<Transformation>> {
    let mut transformations: Vec<Vec<Transformation>> = Vec::new();

    let mut base: Vec<Transformation> = Vec::new();
    base.resize(len, Transformation::Noop);

    println!("41 {:?}", base);
    r_add_transformation(base, 0, thresh, &mut transformations);

    transformations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_transformations_test() {
        println!("{:?}", generate_transformations(8, 2).len());
    }
}
