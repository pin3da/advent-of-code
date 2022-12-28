import 'dart:io';
import 'dart:collection';
import 'dart:math';

Future<List<String>> readLines(String filename) async {
  String contents = await File(filename).readAsString();
  return contents.split('\n');
}

class Node {
  String id;
  int rate;
  List<String> adj;
  Node(this.id, this.rate, this.adj);

  String toString() => "$id, $rate, $adj";
}

class Key {
  String node;
  Set<String> opened;
  int time;
  Key(this.node, this.opened, this.time);

  bool operator ==(Object o) {
    Key other = o as Key;
    return (node == other.node) &&
        (opened == other.opened) &&
        (time == other.time);
  }

  int get hashCode {
    return Object.hash(node, opened.toString(), time);
  }
}

void main(List<String> args) async {
  List<String> entries = await readLines('data.in');
  List<String> valves = [];
  Map<String, Node> graph = HashMap();
  for (String line in entries) {
    List<String> tokens = line.split(' ');
    String from = tokens[1];
    int rate = int.parse(tokens[4].replaceAll(';', '').replaceAll('rate=', ''));
    List<String> to =
        tokens.sublist(9).map((it) => it.replaceAll(',', '')).toList();
    graph[from] = Node(from, rate, to);
    if (rate > 0) valves.add(from);
  }
  Map<String, Map<String, int>> shortestPath = HashMap();

  for (var start in graph.keys) {
    Map<String, int> dist = HashMap();
    Queue<String> next = Queue();
    next.addLast(start);
    dist[start] = 0;
    while (next.isNotEmpty) {
      String cur = next.removeFirst();
      for (var to in graph[cur]!.adj) {
        if (dist.containsKey(to)) continue;
        dist[to] = dist[cur]! + 1;
        next.addLast(to);
      }
    }
    shortestPath[start] = dist;
  }

  int presure(Set<String> opened) {
    int ans = 0;
    for (var v in opened) {
      ans += graph[v]!.rate;
    }
    return ans;
  }

  Map<Key, int> memo = HashMap();

  int limit = 26;
  // valve at `node` is already opened.
  int dp(String node, Set<String> opened, Set<String> can_open, int time) {
    if (time > limit) return -123456789;
    if (time == limit) return presure(opened);

    Key key = Key(node, opened, time);
    if (memo.containsKey(key)) return memo[key]!;

    if (can_open == opened) {
      memo[key] = presure(opened) * (limit - time);
      return memo[key]!;
    }

    int best = dp(node, opened, can_open, time + 1) + presure(opened);
    for (var to in can_open) {
      if (opened.contains(to)) continue;
      int dist = shortestPath[node]![to]! + 1;
      int released = dist * presure(opened);

      opened.add(to);
      best = max(best, dp(to, opened, can_open, time + dist) + released);
      opened.remove(to);
    }

    memo[key] = best;
    return best;
  }

  int ans = 0;
  for (int mask = 0; mask < (1 << (valves.length)) / 2; mask++) {
    Set<String> first = HashSet();
    Set<String> second = HashSet();
    for (int j = 0; j < valves.length; j++) {
      if (((mask >> j) & 1) > 0)
        first.add(valves[j]);
      else
        second.add(valves[j]);
    }
    memo = HashMap();
    int dp_first = dp('AA', HashSet(), first, 1);
    memo = HashMap();
    int dp_second = dp('AA', HashSet(), second, 1);
    ans = max(ans, dp_first + dp_second);
  }

  print(ans);
}
