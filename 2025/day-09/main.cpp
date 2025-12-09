#include <string>
#include <fstream>
#include <vector>
#include <sstream>
#include <cmath> // abs
#include <algorithm> // sort, (min, max)
#include <inttypes.h>

using namespace std;

// #define FILE "input/demo1.txt"
#define FILE "input/input.txt"

struct POS {
  int x,y;
};

// struct AREA {
//   POS a, b;
//   int64_t area;
// };

int64_t calculate(POS a, POS b) {
  int x = abs(b.x-a.x)+1;
  int64_t y = abs(b.y-a.y)+1;
  // int*int still produces int; even though its put into an int64_t
  // because compilers work this way
  // the two ways you can solve this are shown here
  // please remember
  int64_t area = (int64_t)x*y;
  // printf("between %d:%d and %d:%d, it was x%d and y%d -> %d\n",a.x,a.y,b.x,b.y,x,y,area);
  return area;
};

// side note: it was too low
// and it wasnt the problem of an integer overflow
// might be my area calculator code (even tho its mathematically correct)
// ---
// still too low (as of a second going trough): 2147436216
// https://ivanr3d.com/blog/adventofcode2025.html#9 -> literally the same code
// the max integer limit is very close, even though it shouldn't interfere: 2147483647
// ---
// 65 83343:85539 ; 13287:17949 -> 440255391 incorrect (mine)
// 65 83343:85539 ; 13287:17949 -> 4735222687 correct (ran others code) (https://ivanr3d.com/blog/adventofcode2025.html#9)
// ---
// it was integer overflow again
// because my compiler doesn't want to give me errors (or warnings) on this matter
// read the comments in calculate function
// this took me hours
int64_t part1() {
  int64_t n = 0;

  vector<POS> positions;

  string line;
  ifstream read_line(FILE);
  while (getline(read_line, line)) {
    string segment;
    stringstream split(line);
    POS p;
    int i = 0;
    while (getline(split,segment,',')) {
      i++;
      int num = stoi(segment);
      switch (i) {
        case 1:
          p.x = num;
          break;
        case 2:
          p.y = num;
          break;
      }
    }
    // printf("%d:%d\n",p.x,p.y);
    positions.push_back(p);
  }

  // vector<AREA> areas;

  int64_t largest = 0;
  for (int i = 0; i < positions.size(); i++) {
    POS a = positions[i];
    for (int j = i+1; j < positions.size(); j++) {
      POS b = positions[j];
      int64_t area = calculate(a,b);
      if (largest < area) largest = area;
      // printf("%d %d:%d ; %d:%d -> %d\n",i,a.x,a.y,b.x,b.y,area);
      // AREA obj = {
      //   a,b,area
      // };
      // areas.push_back(obj);
    }
  }

  // struct {
  //   bool operator () (AREA a, AREA b) const { return a.area > b.area; }
  // } customSort;

  // sort(areas.begin(),areas.end(), customSort);

  // for (AREA a : areas) {
  //   printf("%d:%d ; %d:%d -> %d\n",a.a.x,a.a.y,a.b.x,a.b.y,a.area);
  // }

  // n = areas[0].area;
  n = largest;

  return n;
}

int main() {
  int64_t p1 = part1();
  printf("part 1: %" PRId64 "\n",p1);
  return 0;
}
