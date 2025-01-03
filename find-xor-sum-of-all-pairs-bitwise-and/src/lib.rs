fn get_xor_sum(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
    /*
        x = [x_0, ..., x_n]
        y = [y_0, ..., y_m]
        s = (x_0 & y_0) ^ (x_0 & y_1) ^ ... ^ (x_i & y_j) ^ ... (x_n & y_m)
          = (x_0 & (y_0 ^ ... ^ y_m)) ^ ... ^ (x_n & (y_0 ^ ... ^ y_m))
          = (x_0 ^ ... ^ x_n) & (y_0 ^ ... ^ y_m)
    */

    let mut x_sum = 0;
    let mut y_sum = 0;

    for i in &arr1 {
        x_sum ^= i
    }

    for j in &arr2 {
        y_sum ^= j
    }

    return x_sum & y_sum;
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
