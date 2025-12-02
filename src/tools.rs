// A big number to function as a penalty for the impossible.
const BIG_NUM: usize = 0xffff;

type CoinSet = Vec<usize>;

#[derive(Debug,PartialEq,Eq,Clone)]
pub struct Spec {
    pub top: usize,
    pub frac: usize,
    pub num: usize,
}

pub fn gcd(a: usize, b: usize) -> usize {
    let (mut a, mut b) = (a, b);
    // tested somewhat faster than recursion
    while b != 0 { (a, b) = (b, a % b) }
    a
}

// Trivial cases:
//  *  The fractional denominator is not fully used (e.g., fraction of 4, but
//     all coins are halfs).  These cases are covered by smaller fracs.
//  *  A fractional denomination can only be used repeatedly to get up to an
//     integer or fraction with lower denominator.  This is strictly worse than
//     having that total as the coin.
// (More optimizations are possible here but would add code complexity.)
pub fn is_nontrivial(frac: usize, cs: &CoinSet) -> bool {
    if frac == 1 { return true } // optimization: no fractions
    // First criterion; optimize by skipping integral coins.
    // Optimization: in this scan check at least 2 fractional for 2nd criterion.
    cs.iter().filter(|&c| c % frac != 0).fold(
        (0, frac), |(ct, a), &b| (if ct < 2 { ct+1 } else { ct }, gcd(a, b)))
        == (2, 1) &&
    // Second criterion.
    cs.iter().all(|&c| {
        if c % frac == 0 { return true } // optimization: not a fractional coin
        let ogcd = cs.iter()
            // Optimize by skipping integral coins.
            .filter(|&&c2| c2 % frac != 0 && c != c2)
            .fold(frac, |a, &b| gcd(a, b));
        c % ogcd == 0
    })
}

pub fn enumerate(Spec { top, frac, num }: &Spec) -> impl Iterator<Item=CoinSet> {
    let num = *num;
    let frac = *frac;
    let limit = top*frac;
    let bad = num == 0 || *top == 0 || frac == 0 || num > limit;
    let mut cs: CoinSet = vec![limit; num];
    if !bad { cs[0] = 0 }
    std::iter::from_fn(move || {
        loop {
            if bad { return None } // need this in function alas
            let mut upi = num - 1;
            loop {
                let v = cs[upi];
                if v < limit + upi - num { break }
                if upi == 0 { return None }
                upi -= 1;
            }
            cs[upi] += 1;
            // Must be possible to make 1¢.
            if upi == 0 {
                // This quickly filters out impossible cases.
                // (Indeed, all of them for frac <= 4.)
                if cs[upi] > frac { return None }
                if cs[upi] > frac/2 { cs[upi] = frac }
            }
            for i in upi+1 .. num { cs[i] = cs[i-1] + 1 }
            // cloning then filtering is less efficient than looping
            if is_nontrivial(frac, &cs) { return Some(cs.clone()) }
        }
    })
}

// Number of coins to make every denomination from 1/frac to but not including top.
// (Skips fractional values less than smallest coin.)
pub fn mk_needs(Spec { top, frac, .. }: &Spec, coins: &CoinSet) -> Vec<usize> {
    let size = top*frac;
    let mut nd = vec![BIG_NUM; size];
    nd[0] = 0;
    for t in coins[0]..size {
        nd[t] = coins.iter().take_while(|&&c| c <= t).map(|&c| {
            1 + nd[t - c]
            }).min().unwrap();
    }
    nd
}

// Score is number of coins to get all integer totals.
pub fn score(Spec { top, frac, .. }: &Spec, nd: &CoinSet) -> usize {
    (1 .. *top).map(|i| nd[i*frac]).sum()
}

// Returns collection of best possible coinsets.
pub fn find_bests(spec: &Spec, it: impl Iterator<Item=CoinSet>)
        -> (Vec<CoinSet>, usize) {
    let mut best = BIG_NUM;
    let mut bests = Vec::new();
    for cs in it {
        let sc = score(spec, &mk_needs(spec, &cs));
        if sc > best { continue }
        if sc < best { bests.clear(); best = sc; }
        bests.push(cs);
    }
    (bests, best)
}

pub fn show_coin(frac: usize, c: usize) -> String {
    let mut s = String::new();
    if c >= frac || c == 0 {
        s.push_str(format!("{}¢", c/frac).as_str());
    }
    let r = c % frac;
    if r > 0 {
        s.push_str(format!("{}ᵼ", r).as_str());
    }
    s
}

pub fn show_coins(frac: usize, cs: &[usize]) -> String {
    cs.iter().map(|&c| show_coin(frac, c)).collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_nontrivial() {
        assert!(is_nontrivial(1, &vec![1, 2, 3]));
        // For prime numbers, iff there are >= two fractional.
        assert!(is_nontrivial(2, &vec![1, 2, 3]));
        assert!(!is_nontrivial(2, &vec![1, 2, 4]));
        assert!(!is_nontrivial(2, &vec![2, 5, 8]));
        assert!(is_nontrivial(3, &vec![3, 4, 9, 11]));
        assert!(is_nontrivial(3, &vec![3, 5, 9, 11]));
        assert!(!is_nontrivial(3, &vec![3, 5, 9, 12]));
        // For composite, can be okay if none have full fraction.
        assert!(is_nontrivial(6, &vec![2, 12, 18, 21, 27, 32]));
        // But if there is only one of a kind, no.
        assert!(!is_nontrivial(6, &vec![2, 12, 18, 21, 27]));
        assert!(!is_nontrivial(6, &vec![2, 12, 18, 21, 32]));
        // No good if only one fourth (only useful if doubled).
        assert!(!is_nontrivial(4, &vec![4, 9, 18, 22]));
        // But okay if a second one.
        assert!(is_nontrivial(4, &vec![4, 9, 18, 21]));
    }
    fn choose(n: usize, k: usize) -> usize {
        if k > n { 0 } else {
            (n-k+1 ..= n).product::<usize>() / (1 ..= k).product::<usize>()
        }
    }
    #[test]
    fn test_enumerate_count() {
        for top in 0 ..= 50 {
            for num in 0 .. 7 {
                // println!("{} {}", top, num);
                assert_eq!(
                    enumerate(&Spec { top, num, frac: 1 }).count(),
                    if top <= 1 { 0 } else { choose(top-2, num-1) }
                );
            }
        }
    }
    #[test]
    fn test_show_coins() {
        assert_eq!(show_coins(1, &[5, 12, 20]), "5¢ 12¢ 20¢");
        assert_eq!(show_coins(2, &[5, 12, 20]), "2¢1ᵼ 6¢ 10¢");
        assert_eq!(show_coins(2, &[10, 24, 40]), "5¢ 12¢ 20¢");
    }
}

