import 'dart:collection';
import 'dart:io';

Future<List<String>> readLines(String filename) async {
  String contents = await File(filename).readAsString();
  return contents.split('\n');
}

class Cube {
  late List<int> pos;

  Cube(Iterable<int> t) {
    pos = List.from(t);
  }

  static Cube parse(String line) {
    return Cube(line.split(",").map(int.parse));
  }

  bool operator ==(Object o) {
    Cube other = o as Cube;
    return pos[0] == other.pos[0] &&
        pos[1] == other.pos[1] &&
        pos[2] == other.pos[2];
  }

  int get hashCode => Object.hash(pos[0], pos[1], pos[2]);

  String toString() => "$pos";

  Iterable<Cube> getNeighbors() sync* {
    for (int d in [0, 1, 2]) {
      for (int delta in [-1, 1]) {
        Cube c = Cube(this.pos);
        c.pos[d] += delta;
        yield c;
      }
    }
  }
}

void main(List<String> args) async {
  List<String> entries = await readLines('data.in');
  var cubes = entries.map((i) => Cube.parse(i)).toSet();
  int ans = 6 * cubes.length;
  for (var c in cubes) {
    for (var next in c.getNeighbors()) {
      if (cubes.contains(next)) ans--;
    }
  }
  print("Part 1: $ans");

  Cube min_corner = Cube([-1, -1, -1]);
  Cube max_corner = Cube([23, 23, 23]);
  Queue next = Queue();
  next.add(min_corner);
  Set<Cube> exterior = HashSet();
  exterior.add(min_corner);
  while (next.isNotEmpty) {
    Cube cur = next.removeFirst();
    for (var c in cur.getNeighbors()) {
      bool valid = true;
      for (int d in [0, 1, 2]) {
        if (c.pos[d] < min_corner.pos[d] || c.pos[d] > max_corner.pos[d])
          valid = false;
      }
      if (cubes.contains(c)) valid = false;
      if (exterior.contains(c)) valid = false;
      if (valid) {
        next.addLast(c);
        exterior.add(c);
      }
    }
  }

  ans = 0;
  for (var c in cubes) {
    for (var next in c.getNeighbors()) {
      if (exterior.contains(next)) ans++;
    }
  }
  print("Part 2: $ans");
  // 2549 too low
  // 2554 correct
}
