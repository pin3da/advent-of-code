
import 'dart:io';
import 'dart:convert';
import 'dart:collection';
import 'dart:math';

int sign(int x) {
  if (x > 0) return 1;
  if (x < 0) return -1;
  return 0;
}

Future<List<String>> readLines(String filename) async {
  String contents = await File(filename).readAsString();
  return contents.split('\n');
}

class Pos {
  int x, y;
  Pos(this.x, this.y);
  
  String toString() {
    return "{$x, $y}";
  }
  
  bool operator ==(Object o) {
    Pos other = o as Pos;
    return x == o.x && y == o.y;
  }

  int get hashCode => Object.hash(x, y);

  Pos nextTowards(Pos t) {
    int dx = sign(t.x - this.x);
    int dy = sign(t.y - this.y);
    return Pos(this.x + dx, this.y + dy);
  }

  Pos nextDown() => Pos(x + 1, y);
  Pos nextDownLeft() => Pos(x + 1, y - 1);
  Pos nextDownRight() => Pos(x + 1, y + 1);
}

void main(List<String> args) async {
  List<String> entries = await readLines('data.in');
  Set<Pos> rocks = LinkedHashSet();
  int limit = 0;
  for (String line in entries) {
    List<Pos> steps =line.split('->').map((p) {
      List<int> pos = p.split(',').map(int.parse).toList();
      limit = max(limit, pos[1] + 2);
      return Pos(pos[1], pos[0]);
    }).toList();

    for (int i = 1; i < steps.length; i++) {
      Pos start = steps[i - 1];
      rocks.add(start);
      while (start != steps[i]) {
        start = start.nextTowards(steps[i]);
        rocks.add(start);
      }
    }
  }

  int ans = 0;
  for (int y = -1000; y < 1000; y++) rocks.add(Pos(limit, y));
  while (!rocks.contains(Pos(0, 500))) {
    ans++;
    Pos sand = Pos(0, 500);
    while (true) {
      if (!rocks.contains(sand.nextDown())) {
        sand = sand.nextDown();
        continue;
      }
      if (!rocks.contains(sand.nextDownLeft())) {
        sand = sand.nextDownLeft();
        continue;
      }
      if (!rocks.contains(sand.nextDownRight())) {
        sand = sand.nextDownRight();
        continue;
      }
      rocks.add(sand);
      break;
    }
  }
  print("$ans");
}