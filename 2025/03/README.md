### Brainstorming

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
