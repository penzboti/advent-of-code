#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <sstream>

using namespace std;

// #define FILE "input/demo1.txt"
#define FILE "input/input.txt"

// on linux, 'long' worked fine
// on windows 'long long' is needed
// this is for numbers over 2^32
// but only if unsigned (2^31 for signed type)
long long part1() {
  long long n = 0;

  string file;
  ifstream read_file(FILE);
  while (getline(read_file, file, ',')) {
    stringstream range(file);
    string segment;

    getline(range, segment, '-');
    long long start_i = stoll(segment);
    getline(range, segment, '-');
    long long end_i = stoll(segment);

    for (long long i = start_i; i <= end_i; i++) {
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

long long part2() {
  long long n = 0;

  string file;
  ifstream read_file(FILE);
  while (getline(read_file, file, ',')) {
    stringstream range(file);
    string segment;

    getline(range, segment, '-');
    long long start_i = stoll(segment);
    getline(range, segment, '-');
    long long end_i = stoll(segment);

    for (long long i = start_i; i <= end_i; i++) {
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
  long long p1 = part1();
  long long p2 = part2();
  cout << "part 1: " << p1 << endl;
  cout << "part 2: " << p2 << endl;
  return 0;
}
