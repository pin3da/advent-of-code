import 'dart:collection';
import 'dart:io';
import 'dart:math';

Future<List<String>> readLines(String filename) async {
  String contents = await File(filename).readAsString();
  return contents.split('\n');
}

class Elements {
  // ore, clay, obsidean, geodes.
  List<int> p = List.filled(4, 0);

  bool operator >=(Elements o) {
    for (int i = 0; i < this.p.length; i++) {
      if (this.p[i] < o.p[i]) return false;
    }
    return true;
  }

  Elements operator -(Elements o) {
    Elements ans = this.clone();
    for (int i = 0; i < this.p.length; i++) ans.p[i] -= o.p[i];
    return ans;
  }

  Elements operator +(Elements o) {
    Elements ans = this.clone();
    for (int i = 0; i < this.p.length; i++) ans.p[i] += o.p[i];
    return ans;
  }

  Elements clone() {
    Elements ans = Elements();
    for (int i = 0; i < this.p.length; i++) ans.p[i] = this.p[i];
    return ans;
  }

  String toString() {
    return p.toString();
  }
}

class Blueprint {
  List<Elements> robots = [];

  Blueprint(List<int> costs) {
    // ore robot
    robots.add(Elements());
    robots.last.p[0] = costs[0];
    // clay
    robots.add(Elements());
    robots.last.p[0] = costs[1];
    // Obsidean
    robots.add(Elements());
    robots.last.p[0] = costs[2];
    robots.last.p[1] = costs[3];
    // geode
    robots.add(Elements());
    robots.last.p[0] = costs[4];
    robots.last.p[2] = costs[5];
  }
}

Map<String, int> memo = HashMap();
Elements max_robots = Elements();
Elements max_materials = Elements();

class State {
  int time_left = 0;
  Elements robots = Elements();
  Elements materials = Elements();
  State(int time_left) {
    this.time_left = time_left;
    this.robots.p[0] = 1;
  }

  State clone(Elements robots, Elements materials) {
    State ans = State(this.time_left - 1);
    ans.robots = robots;
    ans.materials = materials;
    for (int i = 0; i < ans.materials.p.length - 1; i++) {
      ans.materials.p[i] = min(ans.materials.p[i], max_materials.p[i]);
    }
    return ans;
  }

  String toString() {
    return "$time_left, $robots, $materials";
  }
}

int quality(Blueprint blueprint, State state) {
  if (state.time_left == 0) return state.materials.p[3];

  if (memo.containsKey(state.toString())) {
    return memo[state.toString()]!;
  }

  int best = state.time_left * state.robots.p[3] + state.materials.p[3];
  Elements extra_materials = state.robots.clone();

  // print(state);

  best = max(
      best,
      quality(blueprint,
          state.clone(state.robots, state.materials + extra_materials)));

  for (int robot_type = 0; robot_type < blueprint.robots.length; robot_type++) {
    if (state.robots.p[robot_type] >= max_robots.p[robot_type]) continue;
    if (state.materials >= blueprint.robots[robot_type]) {
      Elements new_robots = state.robots.clone();
      new_robots.p[robot_type]++;
      best = max(
          best,
          quality(
              blueprint,
              state.clone(
                new_robots,
                state.materials -
                    blueprint.robots[robot_type] +
                    extra_materials,
              )));
    }
  }
  memo[state.toString()] = best;
  return best;
}

void main(List<String> args) async {
  List<List<int>> costs = (await readLines("data.in"))
      .map((line) => line.split(" ").map(int.parse).toList())
      .toList();
  List<Blueprint> bluepritns = costs.map((e) => Blueprint(e)).toList();

  // prunning
  max_robots.p = [5, 20, 20, 20];
  max_materials.p = max_robots.p.map((e) => e * 3).toList();

  // int part1 = 0;
  // for (int id = 1; id <= bluepritns.length; id++) {
  //   memo = HashMap();
  //   int q = quality(bluepritns[id - 1], State(24));
  //   print("id $id, q $q");
  //   part1 += (id * q);
  // }
  // print(part1);

  int part2 = 1;
  for (int id = 1; id <= 3; id++) {
    memo = HashMap();
    int q = quality(bluepritns[id - 1], State(32));
    print("id $id, q $q");
    part2 *= q;
  }
  print(part2);
}
