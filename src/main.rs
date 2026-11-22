//AMDG::IMI::IHS::Ω
//Andreal_Martel::Nov_22_2026
// below is the start of a first implimentation that I immediately scrapped
//fn sieve (search_space: u32)-> u32 {
    //for i in 1...=10000 {
     //   if i % 6 == 0 {
      //      if i - 1 == 
       // }
    // 
     //   
    //}
//
//}
//Opening notes outlining architecture and reasoning; skip if just interested in code:
//Realizing this does many unnecessary divisions, I decided to model it the way I check
//primality in my head: compare to a table of perfect squares to find a square root less than or
//equal to sqrt i, then search all primes primes less than or equal to that sqrt for divisibility.
//Doing this computationally (instead of a perfect square lookup) means looking for any prime in the
//sieve such that p * p >= i ; I will start from the trivial primes 2 and 3, and from 5 successively
//add 2 and 4 to check only numbers that == 1 or 5 mod 6 (it is trivial to prove all primes greater
//than 3 must satisfy this condition) The real question is how to store this list. I'm told vecs are
//the way to go for a storable list but we haven't covered those yet. Arrays (which we have covered)
//aren't growable, so I may have to build the logic minus the storage (which is critical for
//actually checking the divisibility of i against) so that it *would* work if that storage were
//there to check against. 
fn main() {
    let mut p: Vec<u32> = vec![2,3,] ;
    let mut n: u32 = 5 ;
    while n <= 10000:u32 {
        if n % p != 0 {
            p.push(n)
        }
    }
//below is junk 2, needs to be replaced with a simple range for space and a vec for known primes
//     let mut space: Vec<u32> = vec![2, 3, 5,] ;
    //for i in space[2..] {
        //if (i + 2) % space[2..] != 0 
            //space.push(i + 2)
        //if else (i + 4) % space[2..] != 0 
            //space.push(i + 4)
    //}
    //println!("Primes: {:?}", space);
}
