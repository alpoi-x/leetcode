use std::collections::HashSet;

fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut cache: HashSet<i32> = HashSet::new();
    return queries
        .iter()
        .map(|query| {
            let mut count = 0;
            for i in query[0]..=query[1] {
                if cache.contains(&i) {
                    count += 1;
                    continue;
                }
                if is_vowel_wrapped(words[i as usize].clone()) {
                    count += 1;
                    cache.insert(i);
                }
            }
            return count;
        })
        .collect()
}

fn is_vowel_wrapped(word: String) -> bool {
    const VOWELS: &[char] = &['a', 'e', 'i', 'o', 'u'];
    return word.starts_with(VOWELS) && word.ends_with(VOWELS)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let words = vec![
            "aba".to_string(),
            "bcb".to_string(),
            "ece".to_string(),
            "aa".to_string(),
            "e".to_string()
        ];
        let queries = vec![vec![0, 2], vec![1, 4], vec![1, 1]];
        let res = vowel_strings(words, queries);
        assert_eq!(res, vec![2, 3, 0]);
    }
}
