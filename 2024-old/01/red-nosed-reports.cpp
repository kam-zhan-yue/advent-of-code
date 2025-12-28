#include <iostream>
#include <vector>
#include <map>
#include <fstream>
#include <string>
#include <string_view>
#include <array>
#include <cstdlib>

using namespace std;

array<string_view, 2> split(string_view str, char delimiter) {
  array<string_view, 2> tokens;
  size_t start = 0;
  size_t end = str.find(delimiter);
  tokens[0] = (str.substr(start, end));
  tokens[1] = (str.substr(end, str.size()));
  return tokens;
}

int main() {
  // Initialise data structures
  vector<int> list_1;
  vector<int> list_2;
  map<int, int> count;

  // Read the input file and allocate to list vectors
  ifstream file("input.txt");
  string str;
  while(std::getline(file, str)) {
    array<string_view, 2> strings = split(str, ' ');
    list_1.push_back(atoi(strings[0].data()));
    list_2.push_back(atoi(strings[1].data()));
  }

  // Loop through list_2 and generate a map of all occurences
  for (int i=0; i<list_2.size(); ++i) {
    int key = list_2[i];
    if (count.count(key)) {
      count[key] = count[key] + 1;
    } else {
      count[key] = 1;
    }
  }

  int similarity = 0;
  // Loop through list_1 and check similarity score
  for (int i=0; i<list_1.size(); ++i) {
    similarity += list_1[i] * count[list_1[i]];
  }

  cout << "Similarity Score is: " << similarity << endl;
  return 0;
}
