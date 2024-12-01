#include <bits/stdc++.h>

using namespace std;

using Graph = map<string, vector<string>>;

#define debug(x) cerr << #x << " = " << x << endl

void AddNode(Graph& g, string line) {
  stringstream ss(line);
  string name;
  ss >> name;
  string token;
  ss >> token;
  ss >> token;
  token = token.substr(1, token.size() - 2);
  g[name].push_back(token);
  ss >> token;
  g[name].push_back(token.substr(0, token.size() - 1));
}

int GetDir(char d) {
  if (d == 'L') return 0;
  return 1;
}

struct State {
  int pos;
  string node;
  auto operator<=>(const State &s) const = default;
};

struct Cycle {
  int tail;
  int mod;
  vector<int> endings;
};

int main(int argc, char* argv[]) {
  string instructions;
  cin >> instructions;
  string line;
  Graph graph;
  while (getline(cin, line)) {
    if (line.empty()) continue;
    AddNode(graph, line);
  }

  string current = "AAA";
  int ans = 0;
  if (argc == 1) { // no arguments, part 1.
    while (current != "ZZZ") {
      char d = instructions[ans % instructions.size()];
      current = graph[current][GetDir(d)];
      ans++;
    }
    cout << "Part 1 " << ans << endl;
  }

  vector<Cycle> loops;
  for (auto node : views::keys(graph)) {
    if (node.back() == 'A') {
      vector<int> endings;
      map<State, int> seen;
      ans = 0;
      current = node;        
      int state_id = 0;
      while (true) {
        State key{state_id, current};
        if (seen.contains(key)) {
          int mod = ans - seen[key];
          loops.emplace_back(seen[key], mod, endings);
          break;
        }
        seen[key] = ans;
        if (current.back() == 'Z') {
          endings.push_back(ans);
        } 
        char d = instructions[state_id];
        current = graph[current][GetDir(d)];
        ans++;
        state_id = (state_id + 1) % instructions.size();
      }
    }
  }

  int64_t lcm = loops[0].mod;
  for (auto v : loops) {
    lcm = std::lcm(lcm, v.mod);
  }
  
  cout << "Part 2: " << lcm << endl;
  
  return 0;
}
