#include <bits/stdc++.h>

using namespace std;

#define debug(x) cerr << #x << " = " << x << endl;

struct Pos {
  int x;
  int y;
  auto operator<=>(const Pos&) const = default;
};

struct Number{
  int val = 0;
  set<Pos> pos;
  auto operator<=>(const Number&) const = default;
};

bool IsSymbol(const vector<string>& data, int x, int y) {
  if (x < 0 || x >= data.size()) return false;
  if (y < 0 || y >= data[0].size()) return false;
  char c = data[x][y];
  if (c == '.' || isdigit(c)) return false;
  return true;
}

bool IsValid(const Number& n, const vector<string>& data) {
  for (auto p : n.pos) {
    for (int dx = -1; dx <= 1; dx++) {
      for (int dy = -1; dy <= 1; dy++) {
        if (IsSymbol(data, p.x + dx, p.y + dy)) {
          return true;
        }
      }
    }
  }
  return false;
}

vector<Number> ReadNumbers(const vector<string>& data) {
  vector<Number> all_numbers;
  for (int x = 0; x < data.size(); x++) {
    Number current;
    auto& line = data[x];
    for (int y = 0; y < line.size(); y++) {
      if (isdigit(line[y])) {
        current.val *= 10;
        current.val += line[y] - '0';
        current.pos.insert({x, y});
      } else {
        if (current.val != 0) {
          all_numbers.emplace_back(current);
        }
        current.val = 0;
        current.pos.clear();
      }
    }
    if (current.val != 0) {
      all_numbers.emplace_back(current);
    }
  }
  return all_numbers;
}

int64_t Part2(const vector<string>& data, const vector<Number>& parts) {
  map<Pos, const Number*> number_by_pos;
  for (auto& n : parts) {
    for (auto& p : n.pos) number_by_pos[p] = &n;
  }

  int64_t total = 0;
  for (int x = 0; x < data.size(); x++) {
    for (int y = 0; y < data[0].size(); y++) {
      if (data[x][y] != '*') continue;
      set<Number> touching;
      for (int dx = -1; dx <= 1; dx++) {
        for (int dy = -1; dy <= 1; dy++) {
          auto r = number_by_pos.find({x + dx, y + dy});
          if (r != number_by_pos.end()) {
            touching.insert(*r->second);
          }
        }
      }
      if (touching.size() == 2) {
        long cur = 1;
        for (auto& it : touching) cur *= it.val;
        total += cur; 
      }
    }
  }
  return total;
}

int main() {
  string line;
  vector<string> data;
  while (cin >> line) {
    data.push_back(line);
  }

  auto filtered = ReadNumbers(data)
    | views::filter([&data](auto &n) { return IsValid(n, data); });
  vector<Number> parts(filtered.begin(), filtered.end());

  auto valid = parts
    | views::transform([](auto &n) { return n.val; });

  cout << "Part 1 = " << accumulate(valid.begin(), valid.end(), 0L) << endl;
  cout << "Part 2 = " << Part2(data, parts) << endl;

  return 0;
}
