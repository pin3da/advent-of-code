import 'dart:io';
import 'dart:math';

Future<List<String>> readLines(String filename) async {
  String contents = await File(filename).readAsString();
  return contents.split('\n');
}

typedef Char = String;
typedef Rock = List<List<Char>>;

extension Rotations on Rock {
  Rock moveLeft() {
    bool can_move = true;
    for (var line in this) {
      if (line[0] == "#") can_move = false;
    }
    Rock ans = [];
    for (var line in this) {
      if (can_move)
        ans.add(line.sublist(1) + ["."]);
      else
        ans.add(line);
    }

    return ans;
  }

  Rock moveRight() {
    bool can_move = true;
    for (var line in this) {
      if (line[6] == "#") can_move = false;
    }
    Rock ans = [];
    for (var line in this) {
      if (can_move)
        ans.add(["."] + line.sublist(0, 6));
      else
        ans.add(line);
    }
    return ans;
  }
}

class Chamber {
  List<List<Char>> tower = [];
  int highest = -1;
  int numRocks = 0;
  int jet = 0;

  bool canMove(Rock rock, int at_row) {
    if (at_row < 0) return false;
    for (var i = 0; i < rock.length; i++) {
      for (var col = 0; col < 7; col++) {
        if (tower[at_row + i][col] == '#' && rock[i][col] == '#') {
          return false;
        }
      }
    }
    return true;
  }

  void moveAtRow(Rock rock, int at_row) {
    for (var i = 0; i < rock.length; i++) {
      for (var col = 0; col < 7; col++) {
        if (rock[i][col] == '#') {
          tower[at_row + i][col] = rock[i][col];
        }
      }
    }
  }

  void addRock(Rock rock, String pattern) {
    numRocks++;
    while (tower.length <= highest + 3 + rock.length)
      tower.add(".......".split(''));
    int next_row = highest + 3;
    bool can_fall = true;
    while (can_fall) {
      Rock next_rock = rock.moveLeft();
      String dir = pattern[jet++ % pattern.length];
      if (dir == ">") {
        next_rock = rock.moveRight();
      }

      if (canMove(next_rock, next_row + 1)) {
        rock = next_rock;
      }

      if (!canMove(rock, next_row)) {
        break;
      }
      next_row--;
    }
    moveAtRow(rock, next_row + 1);
    highest = max(highest, next_row + rock.length);
  }

  String toString() {
    var buffer = StringBuffer();
    for (var line in tower.reversed) {
      buffer.writeln(line.join());
    }
    buffer.writeln("-------");
    return buffer.toString();
  }
}

// z[i] = length of the longest preffix of s, wich is a prefix of s.substring(i)
List<int> zFunction(List<int> s) {
  List<int> z = List.filled(s.length, 0);
  for (int i = 1; i < s.length; i++) {
    while (i + z[i] < s.length && s[z[i]] == s[i + z[i]]) {
      z[i]++;
    }
  }
  return z;
}

void main(List<String> args) async {
  String pattern = (await readLines(args[0]))[0];
  Chamber chamber = Chamber();
  List<Rock> rocks = [
    ["..####.".split('')],
    ["...#...".split(''), "..###..".split(''), "...#...".split('')],
    ["..###..".split(''), "....#..".split(''), "....#..".split('')],
    [
      "..#....".split(''),
      "..#....".split(''),
      "..#....".split(''),
      "..#....".split('')
    ],
    ["..##...".split(''), "..##...".split('')],
  ];

  int last = 0;
  List<int> diffs = [];
  for (int curr_rock = 0; chamber.numRocks < 5000; curr_rock++) {
    chamber.addRock(rocks[curr_rock % 5], pattern);
    diffs.add(chamber.highest + 1 - last);
    last = chamber.highest + 1;
    if (curr_rock == 2021) print("part 1: ${chamber.highest + 1}");
    // 3118 was too low
    // 3191 correct
  }

  List<int> z = zFunction(diffs.reversed.toList());
  int len_cycle = 0;
  for (int i = 0; i < z.length; i++) {
    if (z[i] > i) {
      len_cycle = i;
      if (len_cycle % (rocks.length * pattern.length) == 0) break;
    }
  }

  int len_prefix = diffs.length % (len_cycle);
  List<int> cycle = diffs.sublist(diffs.length - len_cycle);

  int sum_prefix = 0;
  for (var i = 0; i < len_prefix; i++) {
    sum_prefix += diffs[i];
  }
  int sum_cycle = 0;
  for (var s in cycle) sum_cycle += s;

  int target = 1000000000000;
  target -= len_prefix;

  int times = target ~/ len_cycle;
  int extra = target % len_cycle;
  int sum_extra = 0;
  for (var i = 0; i < extra; i++) {
    sum_extra += cycle[i];
  }
  int ans = sum_prefix + sum_cycle * times + sum_extra;
  print("Part 2: $ans");
}
