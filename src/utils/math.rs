pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false; 
    } 
    for i in 2..(n.isqrt()+1) {
        if n % i == 0 { 
            return false; 
        } 
    } 
    true  
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime_small() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
    }


    #[test]
    fn test_is_prime_for_primes() {
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(13));
        assert!(is_prime(4889));
        assert!(is_prime(7873));
    }

    #[test]
    fn test_is_prime_for_non_primes() {
        assert!(!is_prime(4));
        assert!(!is_prime(9));
        assert!(!is_prime(28));
        assert!(!is_prime(4890));
    }
}
