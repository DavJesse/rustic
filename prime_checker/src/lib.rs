#[derive(PartialEq, Eq, Debug)]
pub enum PrimeErr {
    Even,
    Divider(u32),
}

pub fn prime_checker(nb: u32) -> Option<Result<u32, PrimeErr>> {
    if nb <= 1 {
        return None;
    }

    if is_prime(nb) {
        return Some(Ok(nb));
    }

    if nb%2 == 0 {
        return Some(Err(PrimeErr::Even));
    }

    Some(Err(PrimeErr::Divider(smallest_divider(nb))))
}

fn is_prime(nb: u32) -> bool {
    if (nb != 2 && nb % 2 == 0) || (nb != 3 && nb % 3 == 0) {
        return false;
    }

    for i in 5..=nb.isqrt() {
        if nb % i == 0 {
            return false;
        }
    }
    true
}

fn smallest_divider(nb: u32) -> u32 {
    let mut start = 3;
    while start <= nb {
        if nb % start == 0 {
            return start;
        }
        start = next_prime(start);
    }
    nb
}

fn next_prime(nb: u32) -> u32 {
    let mut nb = nb;
    while !is_prime(nb) {
        nb += 1;
    }
    nb
}