use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let magazine_words: HashMap<&str, u32> =
        magazine.iter().fold(HashMap::new(), |mut words, word| {
            *words.entry(word).or_insert(0) += 1;
            words
        });
    let note_words: HashMap<&str, u32> = note.iter().fold(HashMap::new(), |mut words, word| {
        *words.entry(word).or_insert(0) += 1;
        words
    });

    note_words
        .iter()
        .all(|(word, count)| magazine_words.get(word).unwrap_or(&0) >= count)
}
