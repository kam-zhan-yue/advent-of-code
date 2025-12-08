#ifndef SOLUTION_H
#define SOLUTION_H

#include <map>

using namespace std;

struct Point {
  long x;
  long y;
  long z;
};

int get_connections();
vector<Point> get_points();
long long length(Point a, Point b);
void solve(vector<Point> points, int connections);
int get_closest_point(vector<Point> points, int index);

class Graph {
private:
  vector<Point> points;
  map<int, int> hashmap;
  map<int, vector<int>> circuits;
  int circuit_num = 0;

public:
  Graph(vector<Point> p) {
    points = p;
  }

  int connect(int a, int b) {
    // If either are in the graph and in the same circuit, then do nothing
    if (hashmap.count(a) && hashmap.count(b) && hashmap.at(a) == hashmap.at(b)) {
      return 1;
    }

    // If neither are in the graph, then init them together
    if (!hashmap.count(a) && !hashmap.count(b)) {
      circuits.insert({ circuit_num, { a, b } });
      hashmap.insert({ a, circuit_num });
      hashmap.insert({ b, circuit_num });
      circuit_num++;
      return 1;
    }

    if (hashmap.count(a) && !hashmap.count(b)) {
      int circuit_id = hashmap[a];
      hashmap.insert({ b, circuit_id });
      circuits[circuit_id].push_back(b);
      return 1;
    }
    if (hashmap.count(b) && !hashmap.count(a)) {
      int circuit_id = hashmap[b];
      hashmap.insert({ a, circuit_id });
      circuits[circuit_id].push_back(a);
      return 1;
    }

    // Then they are in two different circuits and we need to merge them
    int ca = hashmap[a];
    int cb = hashmap[b];
    if (circuits[ca].size() < circuits[cb].size()) {
      swap(ca, cb);
    }
    for (int node : circuits[cb]) {
      circuits[ca].push_back(node);
      hashmap[node] = ca;
    }
    circuits.erase(cb);
    return 1;
  }

  long get_part_one() {
    long part_one = 1;
    vector<size_t> sizes;
    for (const auto& [key, vec] : circuits) {
      sizes.push_back(vec.size());
    }

    sort(sizes.begin(), sizes.end(), greater<>());

    for (int i=0; i<3; ++i) {
      part_one *= sizes[i];
    }
    return part_one;
  }

  bool is_complete() {
    vector<int> connected = circuits.begin()->second;
    bool completed = connected.size() == points.size();
    return completed;
  }
};

#endif
