# Day Five - C

Blast from the past, I haven't written C in ages. Can't wait to use after free.

Anyways, the crux of the problem is to merge ranges together. Unfortunately, I didn't know how to do this intuitively, so will have to read up on it.

The simple approach is to group all intervals by sotring them, then starting from the first interval and comparing it with all other intervals for overlaps. If the first interval overlaps with any other interval, then remove the other interval from the list and merge the other into the first interval.

```python
merge_ranges(ranges: list[tuple[int, int]]):
    sort(ranges)
    merged: list[tuple[int, int]] = []

    for i in range(len(ranges)):
        start = ranges[i][0]
        end = ranges[i][1]
        # Skip already merged
        if merged.len > 0 and merged[-1][1] >= end:
            continue
        # Merge if possible
        for j in range(i + 1, len(ranges)):
            if ranges[j][0] <= end:
                end = max(end, ranges[j][1])
        merged.append((start, end))
    return merged
```

The optimised approach is just to check with the last merged interval

```python
merge_ranges(ranges: list[tuple[int, int]]):
    sort(ranges)
    merged: list[tuple[int, int]] = []
    # Push the first one always
    merged.append(ranges[0])
    for i in range(1, len(ranges)):
        last = merged[-1]
        curr = ranges[i]

        # If the current interval overlaps with the last merged, then merge
        if curr[0] <= last[1]:
            last[1] = max(last[1], curr[1])
        else
            merged.append(curr)
    return merged
```
