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