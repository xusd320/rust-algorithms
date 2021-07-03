pub fn inserction_sort(nums: &mut [i64]) {
    for i in 1..nums.len() {
        let v = nums[i];
        let mut j = i - 1;
        let mut hit_boundary = false;
        while nums[j] > v {
            nums[j + 1] = nums[j];
            if j == 0 {
                hit_boundary = true;
                break;
            }
            j -= 1;
        }
        if hit_boundary {
            nums[0] = v;
        } else {
            nums[j + 1] = v;
        }
    }
}

#[test]
fn test_inserction_sort() {
    let mut nums = [5, 4, 1, 3, 2, 6, 8, 7];
    inserction_sort(&mut nums);
    assert_eq!(nums, [1, 2, 3, 4, 5, 6, 7, 8]);

    let mut nums = [-1, 4, 2, 7, 3, 3, 9, 8];
    inserction_sort(&mut nums);
    assert_eq!(nums, [-1, 2, 3, 3, 4, 7, 8, 9]);
}
