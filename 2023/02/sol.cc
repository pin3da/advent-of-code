#include <bits/stdc++.h>
#include <ranges>

#define debug(x) cerr << #x << " = " << x << endl; 
using namespace std;

struct Game {
  int id;
  vector<map<string, int>> samples;
};

vector<map<string, int>> parseSamples(string line) {
  vector<map<string, int>> samples;
  for (const auto sample : views::split(line, ';')) {
    map<string, int> cur;
    string line{string_view(sample)};
    stringstream ss(line);
    int val; 
    string token;
    ss >> val;
    while (ss >> token) {
      if (token.back() == ',') token.pop_back();
      cur[token] += val;
      ss >> val;
    }
    samples.push_back(cur);
  }
  return samples;
}

Game ParseGame(string line) {
  Game g;
  stringstream ss(line);
  string token;
  ss >> token;
  ss >> token;
  token.pop_back();
  g.id = stoi(token);
  getline(ss, token);
  g.samples = parseSamples(token);
  return g;
}

bool IsPossible(const Game& g) {
  map<string, int> max_val = {
    {"red", 12},
    {"green", 13},
    {"blue", 14},
  };

  for (auto s : g.samples) {
    for (auto [col, val] : s) {
      if (max_val[col] < val) {
        return false;
      }
    }
  }
  return true;
}

int ComputeMin(const Game& g) {
  map<string, int> minimal;
  for (auto s : g.samples) {
    for (auto [col, val] : s) {
      minimal[col] = max(minimal[col], val);
    }
  }
  int ans = 1;
  for (auto m : minimal | views::values) {
    ans *= m;
  }
  return ans;
}

int main() {
  vector<Game> games;
  string line;
  while (getline(cin, line) && line.size() > 0) {
    games.emplace_back(ParseGame(line));
  }

  auto valid = games 
    | views::filter(IsPossible) 
    | views::transform([] (auto it) { return it.id; });

  cout << "Part 1 = " << accumulate(valid.begin(), valid.end(), 0L) << endl;

  auto minimal = games
    | views::transform(ComputeMin);

  cout << "Part 2 = " << accumulate(minimal.begin(), minimal.end(), 0L) << endl;

  return 0;
}
