import 'dart:io';
import 'dart:convert';
import 'dart:collection';
import 'dart:math';
import '../pos.dart';

Future<List<String>> readLines(String filename) async {
  String contents = await File(filename).readAsString();
  return contents.split('\n');
}

class Sensor {
  Pos pos;
  Pos beacon;
  int max_dist;
  Sensor(this.pos, this.beacon, this.max_dist);

  Interval y_interval_at(int x) {
    int to_use = max_dist - (pos.x - x).abs();
    if (to_use <= 0) return Interval(1, -1); // no interval
    return Interval(pos.y - to_use, pos.y + to_use);
  }
}

class Interval {
  int low, hi;
  Interval(this.low, this.hi);
  String toString() => "[$low, $hi]";

  bool intersects(Interval other) {
    return !((other.hi < this.low) || (other.low > this.hi));
  }
  Interval union(Interval other) {
    return Interval(min(this.low, other.low), max(this.hi, other.hi));
  }
}


void main(List<String> args) async {
  List<String> entries = await readLines('data.in');
  List<Sensor> all_sensors = [];

  for (String line in entries) {
    List<String> tokens = line.split(' ');
    String x = tokens[2].replaceAll(',', '');
    String y = tokens[3].replaceAll(':', '');
    Pos sensor = Pos(int.parse(x.substring(2)), int.parse(y.substring(2)));
    x = tokens[8].replaceAll(',', '');
    y = tokens[9];
    Pos beacon =  Pos(int.parse(x.substring(2)), int.parse(y.substring(2)));
    int dist = sensor.manhattan_distance(beacon);
    all_sensors.add(Sensor(sensor, beacon, dist));
  }

  int ans = 0;
  int line = int.parse(args[0]);
  int inf = 5000000;
  Set<Pos> possible = HashSet();
  for (int x = -inf; x < inf; x++) {
    Pos p = Pos(x, line);
    for (var s in all_sensors) {
      if (p.manhattan_distance(s.pos) <= s.max_dist) possible.add(p);
    }
  }
  for (var s in all_sensors) {
    possible.remove(s.pos);
    possible.remove(s.beacon);
  }
  print("Part 1: ${possible.length}");

  for (int x = 0; x <= 4000000; x++) {
    List<Interval> intervals = [];
    for (var s in all_sensors) {
      Interval t = s.y_interval_at(x);
      if (t.low > t.hi) continue;
      intervals.add(t);
    }
    if (intervals.isEmpty) {
      print("WARNING: All points possible");
      continue;
    }
    intervals.sort((a, b) => a.low.compareTo(b.low));
    List<Interval> merged = [];
    Interval cur = intervals[0];
    for (var tmp in intervals) {
      if (cur.intersects(tmp)) {
        cur = cur.union(tmp);
      } else {
        merged.add(cur);
        cur = tmp;
      }
    }
    merged.add(cur);
    for (int i = 0; i < merged.length - 1; i++) {
      int next = merged[i].hi + 1;
      if (next < (merged[i + 1].low)) {
        print("Part 2: ${x * 4000000 + next}");
      }
    }
  }
}
