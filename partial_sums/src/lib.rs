pub fn parts_sums(arr: &[u64]) -> Vec<u64>{
    let mut result = Vec::new();
    let mut sum = 0;
    result.push(0 as u64);

    for num in arr.iter() {
        sum += num;
        result.push(sum);
    }
    result.reverse();
    result
}