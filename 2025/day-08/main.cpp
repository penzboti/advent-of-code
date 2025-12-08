#include <string>
#include <fstream>
#include <vector>
#include <sstream>
#include <cmath> // sqrt
#include <algorithm> // sort and find(_if)

using namespace std;

#define FILE "input/demo1.txt"

struct POS {
  int x,y,z;
  static bool pos_eq(POS a, POS b) {
    return a.x == b.x && a.y == b.y && a.z == b.z;
  };
  bool operator==(const POS& other) const {
    return pos_eq(*this,other);
  };
};

struct DIST {
  POS a,b;
  int d;
};

int distance (POS a, POS b) {
  int x = b.x-a.x;
  int y = b.y-a.y;
  int z = b.z-a.z;
  float d = sqrt(x*x + y*y + z*z);
  return d;
}

int part1() {
  int n = 0;

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
      // printf("%d %d\n",i,num);
      switch (i) {
        case 1:
          p.x = num;
          break;
        case 2:
          p.y = num;
          break;
        case 3:
          p.z = num;
          break;
      }
    }
    positions.push_back(p);
  }

  vector<DIST> distances;

  for (int i = 0; i < positions.size(); i++) {
    POS a = positions[i];
    for (int j = i+1; j < positions.size(); j++) {
      POS b = positions[j];
      int dist = distance(a,b);
      DIST d = {
        a, b, dist
      };

      distances.push_back(d);
    }
  }

  struct {
    bool operator () (DIST a, DIST b) const { return a.d < b.d; }
  } customSort;

  sort(distances.begin(),distances.end(), customSort);

  for (DIST d : distances) {
    printf("d!: %d %d %d : %d %d %d -> %d\n",d.a.x,d.a.y,d.a.z,d.b.x,d.b.y,d.b.z,d.d);
  }

  vector<vector<POS>> circuits;

  // reddit says disjoint set (= union find) algo here (instead of this)
  int i = 0;
  for (DIST d : distances) {
    if (i == 9) break;
    bool found = false;
    for (auto v : circuits) {
      for (POS p : v) {
        if (POS::pos_eq(p,d.a) || POS::pos_eq(p,d.a)) {
          found = true;
          break;
        }
      }
      if (found) {
        if(find(v.begin(), v.end(), d.a) == v.end()) {
          v.push_back(d.a);
        }
        if(find(v.begin(), v.end(), d.b) == v.end()) {
          v.push_back(d.b);
        }
        i++;
      }
    }
    if (!found) {
      circuits.push_back({d.a,d.b});
    }
  }

  for (auto v : circuits) {
    printf("size:%ld\n", v.size());
    for (POS p : v) {
      printf("elem: %d %d %d\n", p.x,p.y,p.z);
    }
  }
  printf("%d\n",circuits.size());

  return n;
}

int main() {
  int p1 = part1();
  printf("part 1: %d\n",p1);
  return 0;
}
