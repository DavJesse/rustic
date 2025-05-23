pub fn reverse_it(v: i32) -> String {
    if v < 0 {
        return format!("-{}{}",v.abs().to_string().chars().rev().collect::<String>(), v.abs());
    }
    format!("{}{}",v.to_string().chars().rev().collect::<String>(), v)
}