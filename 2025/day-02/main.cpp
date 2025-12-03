#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <sstream>
#include <inttypes.h>

using namespace std;

// #define FILE "input/demo1.txt"
#define FILE "input/input.txt"

// on linux, 'long' worked fine
// on windows 'long long' is needed
// this is for numbers over 2^32
// -> long on linux is 64bit, while on windows its only 32bit. this is the DEFINITION
// thats why i switched to specifying bit size.
// but int64_t is OPTIONAL (by the implementation)
// so if this fails, use int_fast32_t
// usually, changing from signed to unsigned doesn't fix any problem
int64_t part1() {
  int64_t n = 0;

  string file;
  ifstream read_file(FILE);
  while (getline(read_file, file, ',')) {
    stringstream range(file);
    string segment;

    getline(range, segment, '-');
    int64_t start_i = stoll(segment);
    getline(range, segment, '-');
    int64_t end_i = stoll(segment);

    for (int64_t i = start_i; i <= end_i; i++) {
      string num = to_string(i);
      int len = num.length();
      if (len % 2 == 1) continue;
      string str1 = num.substr(0, len/2);
      string str2 = num.substr(len/2);
      if (str1 == str2) {
        n += i;
      }
    }
  };

  return n;
}

int64_t part2() {
  int64_t n = 0;

  string file;
  ifstream read_file(FILE);
  while (getline(read_file, file, ',')) {
    stringstream range(file);
    string segment;

    getline(range, segment, '-');
    int64_t start_i = stoll(segment);
    getline(range, segment, '-');
    int64_t end_i = stoll(segment);

    for (int64_t i = start_i; i <= end_i; i++) {
      string num = to_string(i);
      int len = num.length();

      for (int j = 1; j <= len/2; j++) {
        if (len % j != 0) continue;

        vector<string> parts;
        for (int k = j; k <= len; k+=j) {
          string s = num.substr(k-j,j);
          parts.push_back(s);
        }

        bool b = true;
        for (string s : parts) {
          if (parts[0] != s) b = false;
        }

        if (b) {
          n += i;
          break;
        }
      }
    }
  };

  return n;
}

int main() {
  int64_t p1 = part1();
  int64_t p2 = part2();
  // included inttypes.h just for this
  // it should work on windows aswell
  // without warnings
  printf("part 1: %" PRId64 "\n", p1);
  printf("part 2: %" PRId64 "\n", p2);
  // test it in action:
  // printf("%s\n", PRId64);
  // uses %ld for linux, will prob use %lld for windows
  return 0;
}
