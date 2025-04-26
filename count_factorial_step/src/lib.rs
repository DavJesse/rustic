pub fn count_factorial_steps(factorial: u64) -> u64 {
    let mut steps = 0;
    let mut div = 1;
    let mut factorial = factorial;
    if factorial == 0 || factorial == 1{
        return 0;
    }

    while factorial / div > 0 {
        if factorial % div != 0 {
            return 0;
        }
        factorial /= div;
        steps += 1;
        div += 1;
    }
    steps
}
