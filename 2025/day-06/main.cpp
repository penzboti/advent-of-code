// #include <iostream> // note: still needed for cout
#include <string>
#include <fstream>
#include <vector>
#include <inttypes.h>
#include <sstream>

using namespace std;

// #define FILE "input/demo1.txt"
#define FILE "input/input.txt"

int64_t part1() {
  int64_t n = 0;

  vector<string> lines;
  vector<char> operators;
  vector<int64_t> numbers;

  string line;
  ifstream read_line(FILE);
  while (getline(read_line, line)) {
    stringstream split(line);
    string segment;
    lines.push_back(line);
  }

  string segment;
  stringstream split(lines[lines.size()-1]);
  int i = 0;
  while (getline(split, segment, ' ')) {
    if (segment == "") continue;
    // printf("%s\n",segment.c_str());
    operators.push_back(segment[0]);
  }

  for (int i = 0; i < lines.size()-1; i++) {
    string segment;
    stringstream split(lines[i]);

    int j = 0;
    while (getline(split, segment, ' ')) {
      if (segment == "") continue;
      if (segment == "*" || segment == "+") break; // failsafe (shouldnt get here)

      // printf("'%s'\n", segment.c_str());
      int64_t num = stoll(segment);
      char op = operators[j];

      // printf("i:%d j:%d size:%d\n", i,j,numbers.size());
      switch (op) {
        case '*':
          if (numbers.size() <= j) numbers.push_back(1);
          numbers[j] *= num;
          break;
        case '+':
          if (numbers.size() <= j) numbers.push_back(0);
          numbers[j] += num;
          break;
      }
      j++;
    }
  }

  for (int64_t i : numbers) n += i;

  return n;
}

int64_t part2() {
  int64_t n = 0;

  vector<string> lines;
  vector<int64_t> numbers;

  string line;
  ifstream read_line(FILE);
  while (getline(read_line, line)) {
    stringstream split(line);
    string segment;
    lines.push_back(line);
  }

  // setting up next step
  vector<string> newlines;
  int max = lines[0].length();
  for (int i = 0; i < max; i++) newlines.push_back("");

  // flipping the text by lines
  // if you read the task, thats exactly how i flip it
  for (int i = 0; i < lines.size(); i++) {
    for (int j = 0; j < lines[0].length(); j++) {
      newlines[max-j-1] += lines[i][j];
    }
  }

  // for(string line : newlines) printf("'%s'\n", line.c_str());

  // note: i could have used the operators (doesn't really matter)
  for (string line : newlines) {
    int64_t num = 0;
    bool empty = true;
    for (char c : line) if (c != ' ') empty = false;
    if (empty) continue;

    for (int i = 0; i < line.length()-1; i++) {
      char c = line[i];
      if (c == ' ' && num == 0) num *= 10;
      else if (c == ' ') continue;
      else num = num*10 + (line[i] - '0');
    }
    numbers.push_back(num);
    num = 0;
    char op = line[line.length()-1];
    if (op != ' ') {
      for (int64_t number : numbers) {
        // printf("_%lld\n",number);
        switch (op) {
          case '+':
            num += number;
            break;
          case '*':
            if (num == 0) num = 1;
            num *= number;
            break;
        }
      }
      n += num;
      // printf("end: %lld\n", num);
      numbers = {};
    }
  }

  return n;
}

int main() {
  int64_t p1 = part1();
  printf("part 1: %" PRId64 "\n", p1);
  int64_t p2 = part2();
  printf("part 2: %" PRId64 "\n", p2);
  return 0;
}
