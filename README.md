This implements a brute-force solution to an optimization problem.
What denominations of coins can we mint so that any whole number
of cents up to (but not including) a dollar can be gotten with, on
average, the least number of coins?  We can equivalently look at the
number of coins needed to produce all of 1¢ up to 99¢.

This program accepts a number of denominations, and also can configure
the “limit” (number of cents in a “dollar”) and can even try
fractional denominations.  Arguments can be a single number or a range.

Example:

```
$ coins 1-6
Up to 99¢
 Minting 1
   1¢ => need 4950
 Minting 2
   1¢ 10¢ => need 900
   1¢ 11¢ => need 900
 Minting 3
   1¢ 12¢ 19¢ => need 515
 Minting 4
   1¢ 5¢ 18¢ 25¢ => need 389
   1¢ 5¢ 18¢ 29¢ => need 389
 Minting 5
   1¢ 5¢ 16¢ 23¢ 33¢ => need 329
 Minting 6
   1¢ 4¢ 6¢ 21¢ 30¢ 37¢ => need 292
   1¢ 5¢ 8¢ 20¢ 31¢ 33¢ => need 292
```

We see for example than an optimal set of four denominations can
include a penny, nickel, quarter, and 18¢ coin.  With 389 coins total,
we can make every number of cents up to 99.

Example with different fractional values:

```
$ coins -f 1-5 -l 64 4
Up to 63¢
 Minting 4
   1¢ 4¢ 13¢ 20¢ => need 210
  2ᵼ = 1¢
   1¢ 2¢1ᵼ 8¢1ᵼ 20¢ => need 263
  3ᵼ = 1¢
   1¢ 2¢2ᵼ 11¢ 13¢1ᵼ => need 291
   1¢ 5¢ 11¢2ᵼ 15¢1ᵼ => need 291
   1¢ 6¢ 11¢2ᵼ 14¢1ᵼ => need 291
  4ᵼ = 1¢
   1¢ 7¢ 10¢3ᵼ 13¢1ᵼ => need 306
  5ᵼ = 1¢
   1¢ 8¢ 10¢1ᵼ 10¢4ᵼ => need 318 
```

Here ᵼ is a symbol for the relevant sub-cent value.

## History

I wrote a program to work on this problem written in OCaml in 2003.
This is a complete rewrite in Rust, with a few more optimizations,
a more friendly interface, and support for fractional values.

The current version may lack polish since I only wrote it to experiment
with the problem.

