use Transformation::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Transformation {
    Noop,
    Insert(char),
    Delete,
    Replace(char),
}

impl Transformation {
    pub fn all() -> Vec<Transformation> {
        let letters: Vec<char> = (b'a'..=b'z').map(|byte| char::from(byte)).collect();
        let mut insertions: Vec<Transformation> = letters
            .clone()
            .into_iter()
            .map(|char| Insert(char))
            .collect();
        let mut replacements: Vec<Transformation> = letters //     .clone()
            .into_iter()
            .map(|char| Replace(char))
            .collect();
        let mut all = vec![Noop, Delete];
        all.append(&mut insertions);
        all.append(&mut replacements);
        all
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
        collection.push(substring);
        return;
    }

    for t in Transformation::all() {
        let mut new_substring = substring.clone();
        replace_in_vec(&mut new_substring, offset, t);
        r_add_transformation(new_substring, offset + 1, thresh, collection);
    }
}

pub fn generate_transformations(len: usize, thresh: u8) -> Vec<Vec<Transformation>> {
    let mut transformations: Vec<Vec<Transformation>> = Vec::new();

    let mut base: Vec<Transformation> = Vec::new();
    base.resize(len, Transformation::Noop);

    r_add_transformation(base, 0, thresh, &mut transformations);

    transformations
}

pub fn transform_word(word: &str, transformations: &Vec<Vec<Transformation>>) -> Vec<String> {
    let mut transformed_words: Vec<String> = vec![];
    for word_transformation in transformations {
        let mut transformed: Vec<char> = word.clone().chars().collect();
        for index in 0..transformed.len() {
            if let Some(letter_transformation) = word_transformation.get(index) {
                match letter_transformation {
                    Noop => continue,
                    Insert(value) => transformed.insert(index, *value),
                    Delete => {
                        if let Some(_) = transformed.get(index) {
                            transformed.remove(index);
                        }
                    }
                    Replace(value) => {
                        if let Some(_) = transformed.get(index) {
                            transformed.remove(index);
                            transformed.insert(index, *value)
                        }
                    }
                }
            }
        }
        transformed_words.push(transformed.into_iter().collect())
    }

    transformed_words.dedup();
    transformed_words
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_transformations_test() {
        let transformations = generate_transformations(8, 2);
        let words = transform_word("liberty", &transformations);
        println!("{:?}", &transformations.len());
        println!("{:?}\nLen: {}", words, words.len());
    }
}
