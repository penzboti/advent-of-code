#include <string>
#include <fstream>
#include <vector>
#include <sstream>
#include <cmath> // sqrt (but instead) hypot (which is sqrt(pow(x) + pow(y) + pow(z)))
#include <algorithm> // sort and find(_if)

using namespace std;

// #define FILE "input/demo1.txt"
// #define LENGTH 10
#define FILE "input/input.txt"
#define LENGTH 1000

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
  POS a,b; // this might be not even needed
  int pos_a, pos_b;
  int d;
};

int distance (POS a, POS b) {
  float x = b.x-a.x;
  float y = b.y-a.y;
  float z = b.z-a.z;
  // float d = sqrt(x*x + y*y + z*z);
  float d = hypot(x,y,z);
  // printf("x: %d, y: %d, z: %d -> %f\n",x,y,z,d);
  return d;
}

// from https://www.geeksforgeeks.org/dsa/introduction-to-disjoint-set-data-structure-or-union-find-algorithm/
class UnionFind {
  vector<int> parent;
public:
  vector<int> sizes;

  // constructor
  UnionFind(int size) {
    parent.resize(size);
    sizes.resize(size,1);

    // Initialize the parent array with each 
    // element as its own representative
    for (int i = 0; i < size; i++) {
      parent[i] = i;
    }
  }

  // Find the representative (root) of the
  // set that includes element i
  int find(int i) {
    // If i itself is root or representative
    if (parent[i] == i) {
      return i;
    }
  
    // Else recursively find the representative 
    // of the parent
    return find(parent[i]);
  }

  // Unite (merge) the set that includes element 
  // i and the set that includes element j
  void unite(int i, int j) {
    // Representative of set containing i
    int irep = find(i);
  
    // Representative of set containing j
    int jrep = find(j);

    if (irep == jrep) return;
   
    // Make the representative of i's set
    // be the representative of j's set
    parent[irep] = jrep;

    // change set sizes
    sizes[jrep] += sizes[irep];
    sizes[irep] = 0;
  }

  // own functions from here
  bool together(int x, int y) {
    return find(x) == find(y);
  }
};

int part1() {
  int n = 1;

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
        a, b, i, j, dist
      };

      distances.push_back(d);
    }
  }

  struct {
    bool operator () (DIST a, DIST b) const { return a.d < b.d; }
  } customSort;

  sort(distances.begin(),distances.end(), customSort);

  // for (DIST d : distances) {
  //   printf("d!: %d %d %d : %d %d %d -> %d\n",d.a.x,d.a.y,d.a.z,d.b.x,d.b.y,d.b.z,d.d);
  // }

  UnionFind uf(positions.size());

  for (int i = 0; i < LENGTH; i++) {
    DIST d = distances[i];
    // printf("d: %d; i: %d; j: %d\n", d.d,d.pos_a,d.pos_b);
    uf.unite(d.pos_a,d.pos_b);
  }

  vector<int> sizes = uf.sizes;
  sort(sizes.begin(), sizes.end(), greater<int>());
  sizes.erase(find(sizes.begin(),sizes.end(),0),sizes.end());

  for (int i = 0; i < 3; i++) n *= sizes[i];
  return n;
}

int main() {
  int p1 = part1();
  printf("part 1: %d\n",p1);
  return 0;
}
