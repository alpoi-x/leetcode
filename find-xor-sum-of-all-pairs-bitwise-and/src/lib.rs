fn get_xor_sum(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in &arr1 {
        for j in &arr2 {
            sum ^= i & j;
        }
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let arr1 = vec![1,2,3];
        let arr2 = vec![6,5];
        let res = get_xor_sum(arr1, arr2);
        assert_eq!(res, 0);
    }

    #[test]
    fn it_works_2() {
        let arr1 = vec![12];
        let arr2 = vec![4];
        let res = get_xor_sum(arr1, arr2);
        assert_eq!(res, 4);
    }
}
