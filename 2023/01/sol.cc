#include <bits/stdc++.h>

using namespace std;

map<string, int> words = {
  {"one", '1'},
  {"two", '2'},
  {"three", '3'},
  {"four", '4'},
  {"five", '5'},
  {"six", '6'},
  {"seven", '7'},
  {"eight", '8'},
  {"nine", '9'}
};

int main() {
  string line;
  vector<int> values;
  while (cin >> line) {
    char first = 0;
    char last = 0;
    for (int i = 0; i < line.size(); i++) {
      char c = line[i];
      cout << c;
      if (isdigit(c)) {
        if (first == 0) first = c;
        last = c;
        continue;
      }
      for (auto& [w, c] : words) {
        int len = w.size();
        if (i + len <= line.size()) {
          if (line.substr(i, len) == w) {
            if (first == 0) first = c;
            last = c;
            break;
          }
        }
      }
    }
    values.emplace_back((first - '0') * 10 + (last -'0'));
    cout << " " << values.back() << endl;
  }
  cout << accumulate(values.begin(), values.end(), 0L) << endl;

  return 0;
}
