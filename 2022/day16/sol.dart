import 'dart:io';
import 'dart:convert';
import 'dart:collection';
import 'dart:math';
import '../pos.dart';

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
    return (node == other.node) && (opened == other.opened) && (time == other.time);
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
    List<String> to = tokens.sublist(9).map((it) => it.replaceAll(',', '')).toList();
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
  
  // valve at `node` is already opened. 
  int dp(String node, Set<String> opened, int time) { 
    if (time > 30) return -123456789;
    if (time == 30) return  presure(opened);

    Key key = Key(node, opened, time);
    if (memo.containsKey(key)) return memo[key]!;
    
    int best = dp(node, opened, time + 1) + presure(opened);
    for (var to in valves) {
      if (opened.contains(to)) continue;
      int dist = shortestPath[node]![to]! + 1;
      int released = dist * presure(opened);

      opened.add(to);
      best = max(best, dp(to, opened, time + dist) + released);
      opened.remove(to);
    }

    memo[key] = best;
    return best;
  }

  print(dp('AA', HashSet(), 1));
}
