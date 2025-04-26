pub fn min_and_max(nb_1: i32, nb_2: i32, nb_3: i32) -> (i32, i32) {
    let mut nums = [nb_1, nb_2, nb_3];
    nums.sort();
    
    (nums[0], nums[2])
}