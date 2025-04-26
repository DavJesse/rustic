pub fn next_prime(nbr: u64) -> u64 {
    let mut nbr = nbr;

    if is_prime(nbr) {
        return nbr;
    }
    
    while !is_prime(nbr) {
        nbr += 1;
    }
    nbr
}

fn is_prime(nbr: u64) -> bool {
    if (nbr != 2 && nbr % 2 == 0) || (nbr != 3 && nbr % 3 == 0) {
        return false;
    }

    for i in 5..=nbr.isqrt() {
        if nbr % i == 0 {
            return false;
        }
    }
    true
}