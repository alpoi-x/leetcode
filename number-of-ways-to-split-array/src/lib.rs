fn ways_to_split_array(nums: Vec<i32>) -> i32 {
    let i64_nums = nums.iter().map(|&num| num as i64).collect::<Vec<i64>>();
    let mut split_count = 0;
    let mut pre_split = i64_nums[0];
    let mut post_split = i64_nums[1..].iter().sum();
    for i in 1..i64_nums.len() {
        // a "split" at index `i` is valid if sum(nums[:i+1]) >= sum(nums[i+1:]) and 0 <= i < n - 1
        if pre_split >= post_split {
            split_count += 1;
        }
        pre_split += i64_nums[i];
        post_split -= i64_nums[i];
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

    #[test]
    fn it_works_4() {
        let nums = (0..999999).collect();
        let res = ways_to_split_array(nums);
        assert_eq!(res, 292892);
    }
}

