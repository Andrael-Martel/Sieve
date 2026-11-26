//AMDG::IMI::IHS::Ω
//Andreal_Martel::Nov_22_2026
// Copyright 2026 Andrael Martel <and.martel@proton.me>
// Distributed under MIT OR Apache 2.0 license
// SPDX-License-Identifier: MIT OR Apache-2.0
//
//
// 
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
//
//plan: while loop to end of range
//variables for working number (n) vec of all primes (p) and index of vec of all p (i)
//if p[i] * p[i] < n
// i +=1
// else for all k in p less than or equal to i
// n % k != 0
// push n ;
// n +=2;
// else n +=2
// if p[i] * p[i] < n
// i += 1
// else for all k in p less than or equal to i
// n % k != 0
// push n
// n += 4
// else n + 4
// end while
// println p.len
// print p
// println to flush print p and actually display it on as few lines as possible.
//

fn main() {
    let mut p: Vec<u32> = vec![2, 3]; // vec of discovered primes
    let mut n: u32 = 5; // number to be checked for primality 
    let mut i = 2; //index of vec p
    let mut is_prime = true; // will be true by default, but set false if any k divides n 
    let range = 100000;
    while n < range {
        while i < p.len() && p[i] * p[i] <= n {
            i += 1;
        }
        for &k in &p[2..i] {
            if n % k == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime == true {
            p.push(n);
            n += 2;
        } else {
            is_prime = true;
            n += 2;
        }
        if n >= range {
            break;
        } else {
            while i < p.len() && p[i] * p[i] <= n {
                i += 1;
            }
            for &k in &p[2..i] {
                if n % k == 0 {
                    is_prime = false;
                    break;
                }
            }
            if is_prime == true {
                p.push(n);
                n += 4;
            } else {
                is_prime = true;
                n += 4;
            }
        }
    }
    println!("{}", p.len());
    let mut no = 0;
    for _element in &p {
        print!("Prime # {:?}: {:?}, ", no + 1, p[no]);
        no += 1;
    }
    println!("End")
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
