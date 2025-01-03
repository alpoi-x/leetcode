fn ways_to_split_array(nums: Vec<i32>) -> i32 {
    let mut split_count = 0;
    for i in 0..nums.len() - 1 {
        if is_valid_split(&nums, i) {
            split_count += 1
        }
    }
    return split_count;
}

fn is_valid_split(nums: &Vec<i32>, i: usize) -> bool {
    // a "split" at index `i` is valid if sum(nums[:i+1]) >= sum(nums[i+1:]) and 0 <= i < n - 1

    let mut j = 0usize;
    let mut pre_split = 0;
    let mut post_split = 0;

    while j <= i {
        pre_split += nums[j];
        j += 1
    }

    while j < nums.len() {
        post_split += nums[j];
        j += 1
    }

    return pre_split >= post_split;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![10,4,-8,7];
        let res = ways_to_split_array(nums);
        assert_eq!(res, 2);
    }

    #[test]
    fn it_works_2() {
        let nums = vec![2,3,1,0];
        let res = ways_to_split_array(nums);
        assert_eq!(res, 2);
    }

    #[test]
    fn it_works_3() {
        let nums = vec![100000,-100000,100000,-100000];
        let res = ways_to_split_array(nums);
        assert_eq!(res, 3);
    }
}

