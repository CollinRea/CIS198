/// Find all prime numbers less than `n`.
/// For example, `sieve(7)` should return `[2, 3, 5]`
pub fn sieve(n: u32) -> Vec<u32> {
  // Vector of bools, index representing the number
  let mut bool_vec: Vec<bool> = vec![true;(n + 1) as usize];
  // set 0 and 1 to false since they are not primes
  bool_vec[0] = false;
  bool_vec[1] = false;
  let upper_limit = (n as f32).sqrt() as u32 + 1;
  for i in 2..upper_limit {
    if bool_vec[i as usize] {
      for j in 0..(n + 1) {
        let inner = (i * 2) + (i * j);
        if inner <= n {
          bool_vec[inner as usize] = false;
        } else {
          break;
        }
      }
    }
  }
  let (primes,_): (Vec<_>, Vec<_>)  = bool_vec.into_iter()
    .enumerate()
    .map(|(i,v)|(i as u32,v))
    .filter(|&(_i,b)| b )
    .unzip();
  primes
}
