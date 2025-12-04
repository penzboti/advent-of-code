#include <iostream>
#include <string>
#include <fstream>
#include <vector>

using namespace std;

// #define FILE "input/demo1.txt"
#define FILE "input/input.txt"

int part1() {
  int n = 0;

  // maybe vector<vector<char>>
  vector<string> map;

  string line;
  ifstream read_line(FILE);
  while (getline(read_line, line)) {
    line = "." + line + ".";
    map.push_back(line);
  }

  string empty_line = "";
  for (int i = 0; i < map[0].length(); i++) empty_line += ".";
  map.push_back(empty_line);
  map.insert(map.begin(), empty_line);

  for (int i = 1; i < map.size()-1; i++) {
    for (int j = 1; j < map[0].length()-1; j++) {

      char c = map[i][j];
      if (c == '.') continue;

      char arr[8] = {
        // --; -0; -+
        // 0-; 00; 0+
        // +-; +0; ++
        map[i-1][j-1],
        map[i-1][j],
        map[i-1][j+1],
        map[i][j-1],
        map[i][j+1],
        map[i+1][j-1],
        map[i+1][j],
        map[i+1][j+1],
      };

      int count = 0;
      for (char ch : arr) {
        if (ch == '@') count += 1;
      }
      if (count < 4) n += 1;

      // printf("%d %d: %c -> %d\n",i,j,c,count);
    }
  }

  return n;
}

int part2() {
  int n = 0;

  // maybe vector<vector<char>>
  vector<string> map;

  string line;
  ifstream read_line(FILE);
  while (getline(read_line, line)) {
    line = "." + line + ".";
    map.push_back(line);
  }

  string empty_line = "";
  for (int i = 0; i < map[0].length(); i++) empty_line += ".";
  map.push_back(empty_line);
  map.insert(map.begin(), empty_line);

  auto newmap = map;

  bool removed = true;
  while (removed) {
    map = newmap;
    removed = false;

    for (int i = 1; i < map.size()-1; i++) {
      for (int j = 1; j < map[0].length()-1; j++) {

        char c = map[i][j];
        if (c == '.') continue;

        char arr[8] = {
          // --; -0; -+
          // 0-; 00; 0+
          // +-; +0; ++
          map[i-1][j-1],
          map[i-1][j],
          map[i-1][j+1],
          map[i][j-1],
          map[i][j+1],
          map[i+1][j-1],
          map[i+1][j],
          map[i+1][j+1],
        };

        int count = 0;
        for (char ch : arr) {
          if (ch == '@') count += 1;
        }

        if (count < 4) {
          newmap[i][j] = '.';
          n += 1;
          removed = true;
        }

        // printf("%d %d: %c -> %d\n",i,j,c,count);
      }
    }
  }

  return n;
}

int main() {
  int p1 = part1();
  printf("part 1: %d\n", p1);
  int p2 = part2(); // stagger these so if the second part crashes, the first one still prints
  printf("part 2: %d\n", p2);
}

