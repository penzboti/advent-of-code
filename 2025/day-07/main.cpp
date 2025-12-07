#include <string>
#include <fstream>
#include <vector>
#include <queue> // this gives queue (1-sided), and deque (2-sided)
#include <inttypes.h>

using namespace std;

// #define FILE "input/demo1.txt"
#define FILE "input/input.txt"
// #define FILE "input/demo2test.txt"

struct POS {
  int x, y;
};

int part1() {
  int n = 0;

  vector<string> lines;
  queue<POS> q;

  string line;
  ifstream read_line(FILE);
  while (getline(read_line, line)) {
    lines.push_back(line);
  }

  int x = lines[0].find('S');
  q.push(POS {x, 0});

  for (; !q.empty(); q.pop()) {
    POS p = q.front();
    POS next = {p.x, p.y+1};
    if (next.x >= lines[0].length() || next.x < 0 || next.y >= lines.size()) continue;
    char c = lines[next.y][next.x];
    switch (c) {
      case '.':
        q.push(next);
        lines[next.y][next.x] = '|';
        break;
      case '^': {
        POS next_left = {next.x-1, next.y};
        POS next_right = {next.x+1, next.y};
        q.push(next_left);
        q.push(next_right);
        n += 1;
        break;
      }
      case '|':
        break;
    }
  }

  return n;
}

// tried with the first queue approach (won't explain, code is here)
// didn't work
// then wrote this (some ideas from reddit)
// didn't work
// then tried a depth-first search algo (copied from reddit straight up)
// was slow af
// then learned what memoization is (using a hashmap to store function returns so we only calculate once)
// tried it with pointers
// didn't bother to fix a long c++ error
// then i used an online problem solver (https://aoc-puzzle-solver.streamlit.app/)
// it spit out a long long long long long number (its just %lld (on windows on better days (never on linux)))
// i realised that might be the problem
// switched everything to int64_t
// got the good result
// fuck you c++
// it was an unannounced integer overflow
// rust would have fixed this (in debug mode; but thats what you would use anyway)
// https://www.reddit.com/r/learnprogramming/comments/9xiae8/c_how_to_detect_integer_overflow/ ????? i cant
// (should probably research compiler flags and compilers in general (g++ vs gcc))
// moral of the story, use 64 bit numbers on 2nd parts of days,
// the advent of code gods will not bring us back to 32-bit land this late into the event
int64_t part2() {
  int64_t n = 0;

  vector<string> lines;
  vector< vector<int64_t> > map;

  string line;
  ifstream read_line(FILE);
  while (getline(read_line, line)) {
    lines.push_back(line);
  }

  // doesnt work with ranged for loops
  // or at least not how i think
  map.resize(lines.size(), {0});
  for (int i = 0; i < map.size(); i++) {
    map[i].resize(lines[0].length(), 0);
  }

  int x = lines[0].find('S');
  map[0][x] = 1;
  lines[0][x] = '.';

  // queue approach
  // do know whats problem but the fix would have been quite memory inefficient (i think)
  // too big of a function i wrote to not include
  // side note: basically worked like a for loop bc it went trough line by line
  // (the y values didn't fluctuate when printed out in debug)
  // 
  // queue<POS> q;
  // q.push(POS {x, 0});
  // for (; !q.empty(); q.pop()) {
  //   POS p = q.front();
  //   int m_tl = map[p.y][p.x];
  //   POS next = {p.x, p.y+1};
  //   if (next.x >= lines[0].length() || next.x < 0 || next.y >= lines.size()) {
  //     n += m_tl;
  //     // printf("END with %d; x:%d y:%d\n",m_tl,next.x,next.y);
  //     continue;
  //   }
  //   map[next.y][next.x] += m_tl;
  //   char c = lines[next.y][next.x];
  //   // printf("'%c' y%d : x%d -> %d\n",c,next.y+1,next.x+1,m_tl);
  //   switch (c) {
  //     case '.':
  //       q.push(next);
  //       lines[next.y][next.x] = '|';
  //       break;
  //     case '^': {
  //       POS next_left = {next.x-1, next.y};
  //       POS next_right = {next.x+1, next.y};
  //       map[next_left.y][next_left.x] += m_tl;
  //       map[next_right.y][next_right.x] += m_tl;
  //       // PROBLEM HERE: this doesn't account for if its already in the queue
  //       q.push(next_left);
  //       q.push(next_right);
  //       break;
  //     }
  //     case '|':
  //       map[next.y][next.x] -= m_tl; // this is kind of bad
  //       break;
  //   }
  // }

  for (int y = 1; y < lines.size(); y++) {
    for (int x = 0; x < lines[0].length(); x++) {
      // prev values
      int64_t map_val = map[y-1][x];
      char c = lines[y-1][x];

      // printf("%d:%d %c; %d\n",y,x,c, map_val);

      switch (c) {
        case '.':
          map[y][x] += map_val;
          break;
        case '^':
          map[y][x-1] += map_val;
          map[y][x+1] += map_val;
          break;
      }
    }
  }

  for (int i = 0; i < map[0].size(); i++) {
    int64_t num = map[lines.size()-1][i];
    n += num;
  }

  return n;
}

int main() {
  int p1 = part1();
  printf("part 1: %d\n",p1);
  int64_t p2 = part2();
  printf("part 2: %" PRId64 "\n",p2);
  return 0;
}
