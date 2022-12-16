
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

  Pos nextTowards(Pos t) {
    int dx = sign(t.x - this.x);
    int dy = sign(t.y - this.y);
    return Pos(this.x + dx, this.y + dy);
  }

  Pos nextDown() => Pos(x + 1, y);
  Pos nextDownLeft() => Pos(x + 1, y - 1);
  Pos nextDownRight() => Pos(x + 1, y + 1);
}