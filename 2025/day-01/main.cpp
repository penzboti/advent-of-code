#include <iostream>
#include <fstream>
#include <string>

using namespace std;

// #define FILE "input/demo1.txt"
#define FILE "input/input.txt"

// enum concept
// enum DIRECTION {
//   LEFT,
//   RIGHT
// };

// structure concept
// struct DATA {
//   DIRECTION d;
//   int i;
//   string t;
// };

// vector concept
// vector<DATA> data;
// data.push_back(DATA {
//                  file[0] == 'L' ? LEFT : RIGHT,
//                  stoi(file.substr(1)),
//                  file,
//                });

int part1() {
  int n = 0;
  int cur = 50;

  string file;
  ifstream read_file(FILE);
  while (getline (read_file, file)) {
    char dir = file[0];
    int num = stoi(file.substr(1));

    switch (dir) {
      case 'L':
        cur -= num;
        break;
      case 'R':
        cur += num;
        break;
    }

    // naive thinking only 2 digit numbers
    // if (cur < 0) cur += 100;
    // if (cur > 99) cur -= 100;

    // as it turns out, we can just leave negative numbers,
    // since any we need just goes to 0 when (-600 % 100 = 0)
    cur = cur % 100;

    if (cur == 0) n += 1;

    // cout << file << "\t" << cur << endl;
  }
  read_file.close();

  return n;
}

int part2() {
  int n = 0;
  int cur = 50;

  string file;
  ifstream read_file(FILE);
  while (getline (read_file, file)) {
    char dir = file[0];
    int num = stoi(file.substr(1));

    for (int i = 0; i < num; i++) {
      switch (dir) {
        case 'L':
          cur -= 1;
          break;
        case 'R':
          cur += 1;
          break;
      }
      cur = cur % 100;
      if (cur == 0) n += 1;
    }
    if (cur < 0) cur += 100;

    // cout << file << "\t" << cur << endl;
  }
  read_file.close();

  return n;
}


int main() {
  int p1 = part1();
  int p2 = part2();
  // it should be noted that cout is faster;
  // cout << "part 1: " << p1 << endl;
  printf("part 1: %d\n", p1);
  printf("part 2: %d\n", p2);
  return 0;
}
