
import 'dart:math';

int sign(int x) {
  if (x > 0) return 1;
  if (x < 0) return -1;
  return 0;
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

  int manhattan_distance(Pos o) {
    return (o.x - x).abs() + (o.y - y).abs();
  }

  Pos nextTowards(Pos t) {
    int dx = sign(t.x - this.x);
    int dy = sign(t.y - this.y);
    return Pos(this.x + dx, this.y + dy);
  }

  Iterable<Pos> getNeighbors() sync* {
    for (int i = -1; i <= 1; i++) {
      for (int j = -1; j <= 1; j++) {
        if (i == 0 && j == 0) continue;
        yield Pos(x + i, y + j);
      }
    }
  }

  Pos nextDown() => Pos(x + 1, y);
  Pos nextDownLeft() => Pos(x + 1, y - 1);
  Pos nextDownRight() => Pos(x + 1, y + 1);
}