import 'dart:io';
import 'dart:convert';
import 'dart:collection';
import 'dart:math';


Future<List<String>> readLines(String filename) async {
  String contents = await File(filename).readAsString();
  const splitter = LineSplitter();
  return splitter.convert(contents);
}

class Pos {
  int x;
  int y;
  Pos(this.x, this.y) {}
  int dist(Pos other) {
    return max((this.x - other.x).abs(), (this.y - other.y).abs());
  }
  Pos add(int x, int y) {
    return Pos(this.x + x, this.y + y);
  }

  @override 
  String toString() {
    return "($x, $y)";
  }
}

int sign(int s) {
  if (s == 0) return 0;
  if (s < 0) return -1;
  else return 1;
}

Map<String, Pos> delta = {
  'U': Pos(-1, 0),
  'D': Pos(1, 0),
  'R': Pos(0, 1),
  'L': Pos(0, -1),
};

void main(List<String> args) async {
  List<String> lines = await readLines('data.in');
  List<Pos> rope = [];
  for (var i = 0; i < int.parse(args[0]); i++) {
    rope.add(Pos(0, 0));
  }
  Set<String> seen = Set();
  seen.add(rope.last.toString());
  for (var line in lines) {
    var l = line.split(' ');
    var dir = l[0];
    var steps = int.parse(l[1]);
    for (var i = 0; i < steps; i++) {
      var d = delta[dir]!;
      rope[0] = rope[0].add(d.x, d.y);
      for (int r = 1; r < rope.length; r++) {
        if (rope[r - 1].dist(rope[r]) > 1) {
          rope[r] = rope[r].add(sign(rope[r - 1].x - rope[r].x),  sign(rope[r - 1].y - rope[r].y));
        }
      }
      seen.add(rope.last.toString());
    }
  }
  print(seen.length);
}