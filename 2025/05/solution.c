#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
  long left;
  long right;
} Interval;

typedef struct {
  Interval *intervals;
  size_t len;
} Ranges;

int comp(const void *ai, const void *bi) {
  const Interval *a = ai;
  const Interval *b = bi;
  if (a->left < b->left) return -1;
  if (a->left > b->left) return 1;
  if (a->right < b->right) return 1;
  if (a->right > b->right) return -1;
  return 0;
}

Ranges merge_ranges(Ranges ranges) {
  qsort(ranges.intervals, ranges.len, sizeof(Interval), comp);
  Interval *merged = malloc(sizeof(Interval));
  merged[0] = ranges.intervals[0];
  size_t len = 1;

  for (int i=1; i<ranges.len; ++i) {
    Interval interval = ranges.intervals[i];
    Interval last = merged[len-1];
    if (interval.left <= last.right) {
      long max = (interval.right > last.right) ? interval.right : last.right;
      merged[len-1].right = max;
    } else {
      Interval *ptr = realloc(merged, (len + 1) * sizeof(Interval));
      merged = ptr;
      merged[len++] = interval;
    }
  }
  Ranges list = { merged, len };
  return list;
}

int within_ranges(long val, Ranges ranges) {
  for (int i=0; i<ranges.len; ++i) {
    Interval interval = ranges.intervals[i];
    if (val >= interval.left && val <= interval.right)
      return 1;
  }
  return 0;
}
  
int main() {
  char *line = NULL;
  size_t len = 0;
  ssize_t read;
  char delim[2];
  delim[0] = '-';
  delim[1] = 0;

  Interval *intervals = NULL;
  size_t ranges_len = 0;

  // Read the Ranges
  while(getline(&line, &len, stdin) != -1) {
    if (line[0] == '\n') {
      break;
    }
    long left = strtol(strtok(line, delim), NULL, 10);
    long right = strtol(strtok(NULL, delim), NULL, 10);

    Interval interval = { left, right };
    Interval *ptr = realloc(intervals, (ranges_len + 1) * sizeof(Interval));
    intervals = ptr;
    intervals[ranges_len++] = interval;
  }

  Ranges ranges = { intervals, ranges_len };
  Ranges merged = merge_ranges(ranges);

  // For Part One
  int part_one = 0;
  while (getline(&line, &len, stdin) != -1) {
    long val = strtol(line, NULL, 10);
    part_one += within_ranges(val, merged);
  }

  // For Part Two
  long part_two = 0;
  for (int i=0; i<merged.len; ++i) {
    Interval interval = merged.intervals[i];
    part_two += interval.right - interval.left + 1;
  }

  printf("====Part One====\n");
  printf("%d\n", part_one);

  printf("====Part Two====\n");
  printf("%ld\n", part_two);

  free(line);
  free(intervals);
  free(merged.intervals);
  return 0;
}
