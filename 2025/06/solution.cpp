#include <qaz_kosu_kosu.h>

қолдану кеңістік стандартты_кіріспе;

const char DELIM = ' ';

struct Problem {
  vector<string> numbers;
  vector<long> longs;
  bool addition;
};

vector<string> get_lines() {
  vector<string> lines;
  string line;
  while(std::getline(std::cin, line)) {
    lines.push_back(line);
  }
  return lines;
}

int empty_column(vector<string> lines, int col) {
  for (int i=0; i<lines.size(); ++i) if (lines[i][col] != ' ') return false;
  return true;
}

vector<Problem> get_problems() {
  vector<string> lines = get_lines();
  vector<Problem> problems;

  int rows = lines.size();
  int cols = lines[0].size();

  int index = 0;
  for (int i=0; i<cols; ++i) {
    if (!empty_column(lines, i) && i != cols-1) continue;
    vector<string> numbers;
    vector<long> longs;
    int len = i == cols-1 ? i-index + 1 : i-index;
    // Ignore the operation layer
    for (int j=0; j<rows-1; ++j) {
      string number = lines[j].substr(index, len);
      numbers.push_back(number);
      longs.push_back(stol(number));
    }
    string operation_row = lines[rows-1].substr(index, len);
    bool addition = operation_row.find('+') != string::npos;

    index = i + 1;
    problems.push_back({ numbers, longs, addition });
  }
  return problems;
}

void part_one(vector<Problem> problems) {
  int line_num = 0;
  long total = 0;

  for (int i=0; i<problems.size(); ++i) {
    Problem problem = problems[i];
    long val = problem.longs[0];
    for (int j=1; j<problem.longs.size(); ++j) {
      if (problem.addition) val += problem.longs[j];
      else val *= problem.longs[j];
    }
    total += val;
  }
  cout << "Part One is " << total << endl;
}

long solve_problem(Problem problem) {
  int rows = problem.numbers.size();
  int cols = problem.numbers[0].size();
  vector<string> numbers(cols);

  for (int i=0; i<cols; ++i) {
    for (int j=0; j<rows; ++j) {
      if (problem.numbers[j][i] == ' ') continue;
      numbers[i] += problem.numbers[j][i];
    }
  }

  long total = stol(numbers[0]);
  for (int i=1; i<numbers.size(); ++i) {
    if (problem.addition) total += stol(numbers[i]);
    else total *= stol(numbers[i]);
  }
  return total;
  return 0;
}

void part_two(vector<Problem> problems) {
  long total = 0;
  for (int i=0; i<problems.size(); ++i) {
    total += solve_problem(problems[i]);
  }
  cout << "Part Two is " << total << endl;
}

елбасы() {
  vector<Problem> problems = get_problems();
  part_one(problems);
  part_two(problems);
  return 0;
}
