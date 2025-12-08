#ifndef ALGORITHM_H
#define ALGORITHM_H

#include <iostream>
#include <map>

using namespace std;

struct Point {
  long x;
  long y;
  long z;
};

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

int get_connections() {
  string line;
  getline(cin, line);
  return stol(line);
}

vector<Point> get_points() {
  vector<Point> points;
  string line;
  while(std::getline(std::cin, line)) {
    size_t split_1 = line.find(',');
    size_t split_2 = line.find(',', split_1 + 1);

    string x = line.substr(0, split_1);
    string y = line.substr(split_1 + 1, split_2 - split_1 - 1);
    string z = line.substr(split_2 + 1);

    Point point = {
      stol(x),
      stol(y),
      stol(z),
    };
    
    points.push_back(point);

  }
  return points;
}

long long length(Point a, Point b) {
  long long dx = (long long)a.x - b.x;
  long long dy = (long long)a.y - b.y;
  long long dz = (long long)a.z - b.z;
  return dx*dx + dy*dy + dz*dz;
}

void solve(vector<Point> points, int connections) {
  // generate distances
  multimap<long long, tuple<int, int>> distances;
  for (int i=0; i<points.size(); ++i) {
    for (int j=i+1; j<points.size(); ++j) {
      long distance = length(points[i], points[j]);
      distances.insert({ distance, { i, j} });
    }
  }

  Graph graph(points);
  int connected = 0;
  for (const auto& [distance, pair] : distances) {
    const auto& [p1, p2] = pair;
    connected += graph.connect(p1, p2);

    if (connected == connections) {
      cout << "Part One: " << graph.get_part_one() << endl;
    }

    // If we connected all of them, then break
    if (graph.is_complete()) {
      long long p1_x = points[p1].x;
      long long p2_x = points[p2].x;
      cout << "Part Two: " << p1_x * p2_x << endl;
      break;
    }
  }
}

int get_closest_point(vector<Point> points, int index) {
  long squared_distance = 0;
  int min_index = -1;
  for (int i=0; i<points.size(); ++i) {
    if (i == index) continue;
    long distance = length(points[index], points[i]);
    if (min_index < 0) {
      squared_distance = distance;
      min_index = i;
    } else if (distance < squared_distance) {
      squared_distance = distance;
      min_index = i;
    }
  }
  return min_index;
}

#endif
