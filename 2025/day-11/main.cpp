#include <string>
#include <fstream>
#include <vector>
#include <sstream>
#include <algorithm>
#include <queue>
#include <set>

using namespace std;

#define FILE "input/demo1.txt"

struct Edge {
  int from, to;
  bool operator==(const Edge& rhs) {
    return from == rhs.from && to == rhs.to;
  }
};

// not implementing an adjacency matrix tho
// https://medium.com/basecs/from-theory-to-practice-representing-graphs-cfd782c5be38
// https://stackoverflow.blog/2022/05/26/the-complete-beginners-guide-to-graph-theory/
class Graph {
  vector<string> nodes;
  vector<Edge> edges;
  set<int> visited; // good for preventing infinite loops; however it stops two examples of running trough
public:
  Graph() {
    nodes.push_back("out");
    nodes.push_back("you");
  }
  void link(string from, string to) {
    auto from_find = find(nodes.begin(),nodes.end(),from);
    if (from_find == nodes.end()) {
      nodes.push_back(from);
    }
    int from_pos = distance(nodes.begin(),find(nodes.begin(),nodes.end(),from));

    auto to_find = find(nodes.begin(),nodes.end(),to);
    if (to_find == nodes.end()) {
      nodes.push_back(to);
    }
    int to_pos = distance(nodes.begin(),find(nodes.begin(),nodes.end(),to));

    // note: all edges are unique
    Edge edge = {from_pos, to_pos};
    edges.push_back(edge);
    // printf("%d: %d->%d\n",edges.size(),edge.from,edge.to);
  }
  // https://www.hackerearth.com/practice/algorithms/graphs/breadth-first-search/tutorial/
  int bfs() {
    int n = 0;
    queue<int> q;

    q.push(1); // preset position of "you"

    for (; !q.empty(); q.pop()) {
      int i = q.front();

      if (i == 0) {n++; continue;}
      if (find(visited.begin(),visited.end(),i) != visited.end()) continue;
      visited.insert(i);

      for (Edge e : edges) {
        if (e.from == i) {
          int j = e.to;
          printf("%s-%s\n",nodes[i].c_str(),nodes[j].c_str());
          q.push(j);
        }
      }
    }
    return n;
  }
};

int part1() {
  int n = 0;

  Graph g;

  string line;
  ifstream read_line(FILE);
  while (getline(read_line,line)) {
    string parent;
    vector<string> children;

    bool first = true;
    string segment;
    stringstream split(line);
    while (getline(split, segment, ' ')) {
      if (first) {
        parent = segment.substr(0,3);
        first = false;
        continue;
      }
      g.link(parent,segment);
    }
  }

  n = g.bfs();

  return n;
}

int main() {
  int p1 = part1();
  printf("part 1: %d\n",p1);
  return 0;
}
