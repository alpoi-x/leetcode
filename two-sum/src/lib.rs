fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if i == j {
                continue;
            }

            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    panic!("Expected exactly one solution");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![2,7,11,15];
        let target = 9;
        let res = two_sum(nums, target);
        assert_eq!(res, vec![0,1]);
    }

    #[test]
    fn it_works_2() {
        let nums = vec![3,2,4];
        let target = 6;
        let res = two_sum(nums, target);
        assert_eq!(res, vec![1,2]);
    }

    #[test]
    fn it_works_3() {
        let nums = vec![3,3];
        let target = 6;
        let res = two_sum(nums, target);
        assert_eq!(res, vec![0,1]);
    }
}
