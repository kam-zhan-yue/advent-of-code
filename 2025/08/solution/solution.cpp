#include <solution.h>

int get_connections() {
  cout << "Get Connections" << endl;
  string line;
  getline(cin, line);
  return stol(line);
}

vector<Point> get_points() {
  cout << "Get Points" << endl;
  vector<Point> points;
  string line;
  while(getline(cin, line)) {
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
