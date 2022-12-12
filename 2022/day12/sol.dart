
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
  int x, y;
  Pos(this.x, this.y) {}

  String toString() {
    return "{$x, $y}";
  }

  bool isValid(int max_x, int max_y) {
    return x >= 0 && x < max_x && y >= 0 && y < max_y; 
  }

}

List<int> dx = [-1, 0, 1, 0];
List<int> dy = [0, -1, 0, 1];

void main(List<String> args) async {
  List<String> grid = await readLines('example.in');
  int rows = grid.length;
  int cols = grid[0].length;
  Pos start = Pos(0, 0);
  Pos end = Pos(0, 0);
  for (int i = 0; i < rows; i++) {
    for (int j = 0; j < cols; j++) {
      if (grid[i][j] == 'S') end = Pos(i, j);      
      if (grid[i][j] == 'E') start = Pos(i, j);
    }
  }
  Queue pending = Queue();
  Map<String, int> best = {};

  pending.addLast(start);
  best[start.toString()] = 0;

  while(pending.isNotEmpty) {
    Pos cur = pending.removeFirst();
    if (cur == end) {
      break;
    }
    for (int k = 0; k < 4; k++) {
      Pos next = Pos(cur.x + dx[k], cur.y + dy[k]);
      if (best.containsKey(next.toString())) continue;
      if (next.isValid(rows, cols) && ((getVal(grid, next) + 1) >= getVal(grid, cur))) {
        best[next.toString()] = best[cur.toString()]! + 1;
        pending.addLast(next);
      }
    }
  }
  print("Part 1 ${best[end.toString()]}");
  int ans2 = 122223;
  for (int i = 0; i < rows; i++) {
    for (int j = 0; j < cols; j++) {
      Pos p = Pos(i, j);
      if (getVal(grid, p) == 0 && best.containsKey(p.toString())) {
        ans2 = min(ans2, best[p.toString()]!);
      }
    }
  }
  print("Part 2 ${ans2}");
}

int getVal(List<String> grid, Pos pos) {
  String v = grid[pos.x][pos.y];
  if (v == 'S') return 0;
  if (v == 'E') return 25;
  return v.codeUnitAt(0) - 'a'.codeUnitAt(0);
}