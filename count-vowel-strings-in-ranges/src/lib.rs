fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    const VOWELS: &[char] = &['a', 'e', 'i', 'o', 'u'];
    let mut cumulative: Vec<i32> = Vec::with_capacity(words.len());
    let mut total = 0;
    for word in words {
        if word.starts_with(VOWELS) && word.ends_with(VOWELS) {
            total += 1;
        }
        cumulative.push(total);
    }

    return queries
        .iter()
        .map(|query| {
            if query[0] == 0 {
                return cumulative[query[1] as usize]
            }
            return cumulative[query[1] as usize] - cumulative[(query[0] - 1) as usize]
        })
        .collect()
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
