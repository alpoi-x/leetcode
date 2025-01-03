fn ways_to_split_array(nums: Vec<i32>) -> i32 {
    let mut split_count = 0;
    let mut pre_split = nums[0];
    let mut post_split = nums[1..].iter().sum();
    for i in 1..nums.len() {
        // a "split" at index `i` is valid if sum(nums[:i+1]) >= sum(nums[i+1:]) and 0 <= i < n - 1
        if pre_split >= post_split {
            split_count += 1;
        }
        pre_split += nums[i];
        post_split -= nums[i];
    }
    return split_count;
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

