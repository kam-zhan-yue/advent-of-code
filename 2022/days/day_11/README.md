## The Solution

### The Chinese Remainder Theorem

The Chinese Remainder Theorem states that if one knows the remainders of the Euclidean division of an integer n by several integers, then one can determine uniquely the division of n by the product of these integers, under the condition that the divisors are pairwise coprime (no two divisors share a common factor other than 1).

For example, if one knows that the remainder of n divided by 3 is 2, the remainder of n divided by 5 is 3, and the remainder of n divided by 7 is 2, then with no other information, one can determine the remainder of n divided by 105 (the product of 3, 5, 7) without knowing the value of n. In this example, the remainder is 23. Moreoever, this remainder is the only possible positive value of n that is less than 105.

The whole trick is that we can multiply the divisors together and our equivalence relation doesn't change. Let's say that D is the product of all our divisors.

If `0 = b (mod n)`, then `0 * D = b * D (mod n * D)`

> This is missing a lot of key information about congruence relationships
