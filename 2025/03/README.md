# Day 3 - Elixir

My first encounter with a functional language! Honestly, not as bad as it seems and elixir benefits a lot from having an interactive shell like python. In fact, I found the experience really enjoyable. However, my smooth brained solution ended up failing some edge cases and I had to make a solution in python and compare both outputs to find out which cases were failing. A common pitfall with functional languages is the difficulty to debug, I imagine.

## Brainstorming

### Part One

```
987654321111111
811111111111119
234234234234278
818181911112111
```

When going through each string, we can analyse it from right to left. We can do two things:
- Make a map of the largest digit at a certain position
- Keep track of the largest possible number at a certai position

Case 1: `987654321111111`
- Index 14: Current is 1, Highest Digit is None -> 1
- Index 8-13: Current is 1, Highest Digit is 1 -> 11
- Index 7: Current is 2, Highest Digit is 1 -> 21
- Index 6: Current is 3, Highest Digit is 2 -> 32
- Index 5: Current is 4, Highest Digit is 3 -> 43
And so on unil 98

Case 2: `811111111111119`
- Index 14: Current is 9, Highest Digit is None -> 9
- Index 1-13: Current is 1, Highest Digit is 9 -> 9
- Index 0: Current is 8, Highest Digit is 9 -> 89
Hence, return 89

### Part Two

```
987654321111111
811111111111119
234234234234278
818181911112111
```

I think I can generalise my solution above in order to support an arbitrary number of digits (n < len(input)). We would build a list alongside our input that holds the current largest digits.

Case 1: `817819` digits = 3
- Index 5: Current is 9, Digits is [] -> [9]
- Index 4: Current is 1, Digits is [9] -> [1, 9]
- Index 3: Current is 8, Digits is [1, 9] -> [8, 1, 9]
- Index 2: Current is 7, Digits is [8, 1, 9] -> Do nothing
- Index 1: Current is 1, Digits is [8, 1, 9] -> Do nothing
- Index 0: Current is 8, Digits is [8, 1, 9] -> [8, 8, 9]

> We compare the current value to the minimum value of the list. If it is larger, we remove that value, then prepend it.


