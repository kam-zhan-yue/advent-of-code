## The Solution

### The Chinese Remainder Theorem

The Chinese Remainder Theorem states that if one knows the remainders of the Euclidean division of an integer n by several integers, then one can determine uniquely the division of n by the product of these integers, under the condition that the divisors are pairwise coprime (no two divisors share a common factor other than 1).

For example, if one knows that the remainder of n divided by 3 is 2, the remainder of n divided by 5 is 3, and the remainder of n divided by 7 is 2, then with no other information, one can determine the remainder of n divided by 105 (the product of 3, 5, 7) without knowing the value of n. In this example, the remainder is 23. Moreoever, this remainder is the only possible positive value of n that is less than 105.

The whole trick is that we can multiply the divisors together and our equivalence relation doesn't change. Let's say that D is the product of all our divisors.

### Exploring Modulos
Take some number `b` and see the remainder when it is divided by `n`. Let's call the remainder `a`. This is modulo: `b % n = a` or `a = b % n`.

Imagine that we had an equation `b % n = a` where `a > 0`. In order to get a value of `a` to be 0, we would just subtract `a` from both sides. In other words, we woudl get `b - a = k * n` where k is some whole number. This is known as a congruence relation, meaning that `a` and `b` are congruent on modulo `n`.

With this congruence relation, we can see that `a = a (mod n)`, which is `a % n = a` when `a < n`.
- See `32 % 31 = 1` and `31 % 1 = 1`

Ok I can't understand this completely.
If `0 = b (mod n)`, then `0 * D = b * D (mod n * D)`

