#include <iostream>
#include <string>
#include <fstream>
#include <vector>

using namespace std;

// #define FILE "input/demo1.txt"
#define FILE "input/input.txt"

int part1() {
  int n = 0;

  string line;
  ifstream read_line(FILE);

  while (getline(read_line, line)) {

    vector<int> nums;
    for (int i = 0; i < line.length(); i++) {
      char c = line[i];
      int num = c-48;
      nums.push_back(num);
    }

    int max = 0;
    for (int i = 0; i < line.length(); i++) {
      for (int j = i+1; j < line.length(); j++) {
        int num = nums[i]*10+nums[j];
        if (num > max) max = num;
      }
    }
    n += max;
  }

  return n;
}

long long part2() {
  long long n = 0;

  string line;
  ifstream read_line(FILE);

  while (getline(read_line, line)) {

    vector<int> nums;
    for (int i = 0; i < line.length(); i++) {
      char c = line[i];
      int num = c-48;
      nums.push_back(num);
    }

    // what i've realised after reading some things on reddit
    // you can make the largest number by finding the largest digit in a range
    // where you know the 12 digits will still fit
    long long max = 0;
    int len = line.length();
    int range_start = 0;

    for (int i = 0; i < 12; i++) {
      int range_end = len-(12-i-1);
      int max_digit_index = 0;
      int max_digit = 0;
      for (int j = range_start; j < range_end; j++) {
        int cur_digit = nums[j];
        if (cur_digit > max_digit) {
          max_digit_index = j;
          max_digit = nums[max_digit_index];
          // printf("digit at i%d j%d is %d\n", i, j, cur_digit);
        }
      }
      range_start = max_digit_index + 1;

      // powering INTEGERS is insane: https://stackoverflow.com/questions/1505675/power-of-an-integer-in-c
      // why is the default implementation in FLOATS ONLY????
      long long power = max_digit;
      for (int p = 1; p < (12-i); p++) power *= 10;
      max += power;
      // printf("max digit at %d: %d\n", i, max_digit);
    }
    // cout << "max:\t" << max << endl;
    n += max;
  }

  return n;
}

int main() {
  int p1 = part1();
  long long p2 = part2();
  printf("part 1: %d\n", p1);
  printf("part 2: %lld\n", p2);
  return 0;
}
