// #include <iostream> // apparently not needed if string is avaliable
#include <string>
#include <fstream>
#include <vector>
#include <sstream>
// #include <cstdint> // not needed if inttypes are avaliable
#include <inttypes.h>

using namespace std;

// #define FILE "input/demo1.txt"
#define FILE "input/input.txt"
// #define FILE "input/demo2test.txt"

struct RANGE {
  int64_t start, end;
};

int part1() {
  int n = 0;

  vector<RANGE> ranges;

  string line;
  ifstream read_line(FILE);
  bool past_line = false;
  while(getline(read_line, line)) {
    if (line == "") { past_line = true; continue; }
    // printf("'%s'\n", line.c_str());

    if (!past_line) {
      stringstream split(line);
      string segment;
      RANGE range;

      getline(split, segment, '-');
      range.start = stoll(segment);
      getline(split, segment, '-');
      range.end = stoll(segment);
      ranges.push_back(range);
    } else {
      int64_t num = stoll(line);
      for (RANGE range : ranges) {
        if (num >= range.start && num <= range.end) {
          n ++;
          // printf("%d -> %d; %d\n",range.start,range.end,num);
          break;
        }
      }
    }
  }

  return n;
}

vector<RANGE> compare_ranges(vector<RANGE> ranges, RANGE range) {
  vector<RANGE> new_ranges;
  bool broken = false;
  for (RANGE compare : ranges) {
    // C is compare (one of ranges), R is range (the new one being tested)
    // if start or end is the same, move it by 1
    if (compare.start == range.start) range.start += 1;
    if (compare.start == range.end) range.end -= 1;
    if (compare.end == range.start) range.start += 1;
    if (compare.end == range.end) range.end -= 1;

    // 4 possibilites
    // 1. C ... R ... R ... C
    // tested with 1-4; 2-3
    if (compare.start < range.start && compare.end > range.end) {
      broken = true;
      break;
    }
    // 2. R ... C ... C ... R
    // tested with mirror of 1.
    // we split the R range into two (make 1 and change 1), and test the new range with recursion 
    if (range.start < compare.start && range.end > compare.end) {
      RANGE range2 = {
        compare.end+1,
        range.end,
      };
      auto vec = compare_ranges(ranges, range2);
      for (RANGE r : vec) new_ranges.push_back(r);
      range.end = compare.start -1;        
    }

    // 3. C ... R ... C ... R
    // tested with 1-3; 2-4
    if (range.start < compare.end && range.start > compare.start) {
      range.start = compare.end+1;
    }
    // 4. R ... C ... R ... C
    // tested with mirror of 3.
    if (range.end < compare.end && range.end > compare.start) {
      range.end = compare.start-1;
    }
  }
  if (!broken) new_ranges.push_back(range);
  return new_ranges;
}

int64_t part2() {
  int64_t n = 0;

  vector<RANGE> ranges;

  string line;
  ifstream read_line(FILE);
  while(getline(read_line, line)) {
    if (line == "") break;
    stringstream split(line);
    string segment;
    RANGE range;

    getline(split, segment, '-');
    range.start = stoll(segment);
    getline(split, segment, '-');
    range.end = stoll(segment);

    auto vec = compare_ranges(ranges, range);
    for (RANGE r : vec) ranges.push_back(r);
  }

  for (RANGE range : ranges) {
    // printf("%d -> %d; %d\n", range.start, range.end, range.end-range.start);
    n += range.end-range.start+1;
  }
  return n;
}

// INSANELY SLOW (just as i thought)
// #include <unordered_set> // basically a hashset
// int64_t part2_hs() {
//   unordered_set<int64_t> hs;

//   string line;
//   ifstream read_line(FILE);
//   while(getline(read_line, line)) {
//     printf("line begin: %s\n", line.c_str());
//     if (line == "") break;
//     stringstream split(line);
//     string segment;

//     getline(split, segment, '-');
//     int64_t start = stoll(segment);
//     getline(split, segment, '-');
//     int64_t end = stoll(segment);

//     for (int64_t i = start; i <= end; i++) hs.insert(i);
//     printf("line len: %lld\n", end-start+1);
//   }
//   return hs.size();
// }

int main() {
  int p1 = part1();
  printf("part 1: %d\n", p1);
  int64_t p2 = part2();
  printf("part 2: %" PRId64 "\n", p2);
  return 0;
}
