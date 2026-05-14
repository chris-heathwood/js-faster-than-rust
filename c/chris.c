#include <stdio.h>
#include <stdint.h>
#include <time.h>

static char input[8192];
static int input_len;

static int find_first_fourteen(const char *s, int len) {
  uint32_t state = 0;
  int i = 13;
  while (i < len) {
    int end = i - 13;
    int w = i;
    while (w >= end) {
      uint32_t marker = 1u << (s[w] & 31);
      if ((state & marker) == 0) {
        state |= marker;
      } else {
        i = w + 14;
        state = 0;
        break;
      }
      if (w == end) return i + 1;
      w--;
    }
  }
  return 0;
}

int main() {
  FILE *f = fopen("../input.txt", "r");
  input_len = fread(input, 1, sizeof(input), f);
  fclose(f);

  volatile int result = 0;

  struct timespec before, after;
  clock_gettime(CLOCK_MONOTONIC, &before);
  for (int t = 0; t < 100; t++) {
    result = find_first_fourteen(input, input_len);
  }
  clock_gettime(CLOCK_MONOTONIC, &after);

  long ns = (after.tv_sec - before.tv_sec) * 1000000000L + (after.tv_nsec - before.tv_nsec);

  printf("findFirstFourteen found %d and took %.2f nanoseconds\n", result, ns / 100.0);
  return 0;
}
