use std::collections::{ HashMap, HashSet } ;

fn count_palindromic_subsequence(s: String) -> i32 {
    let mut indexes: HashMap<char, (usize, usize)> = HashMap::new();
    let chars = s.chars().collect::<Vec<_>>();

    for idx in 0..chars.len() {
        if let Some((_, y)) = indexes.get_mut(&chars[idx]) {
            *y = idx;
        } else {
            indexes.insert(chars[idx], (idx, idx));
        }
    }

    let mut count = 0;

    for (_, (x, y)) in indexes.iter() {
        // only appears once or appears twice side-by-side
        if x.eq(y) || x.eq(&(y + 1)){
            continue;
        }

        let mut centers: HashSet<char> = HashSet::new();
        for idx in (x + 1)..*y {
            centers.insert(chars[idx]);
        }
        count += centers.len();
    }

    return count as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = count_palindromic_subsequence(String::from("aabca"));
        assert_eq!(result, 3);
    }

    #[test]
    fn it_works_2() {
        let result = count_palindromic_subsequence(String::from("adc"));
        assert_eq!(result, 0);
    }

    #[test]
    fn it_works_3() {
        let result = count_palindromic_subsequence(String::from("bbcbaba"));
        assert_eq!(result, 4);
    }
}
