import 'dart:io';
import 'dart:convert';

Future<List<String>> readLines(String filename) async {
  String contents = await File(filename).readAsString();
  const splitter = LineSplitter();
  return splitter.convert(contents);
}

void main() async {
  List<String> lines = await readLines('data.in');
  print(part1(lines));
  print(part2(lines));
}

int part1(List<String> lines) {
 int ans = 0;
  for (var line in lines) {
    Set<String> left = get_seen(line.substring(0, line.length ~/ 2));
    Set<String> right = get_seen(line.substring(line.length ~/ 2));
    for (var k in left) {
      if (right.contains(k)) {
        ans += score(k);
      }
    }
  }
  return ans;
}

int part2(List<String> lines) {
  int ans = 0;
  for (int i = 0; i < lines.length; i += 3) {
    Set<String> current = get_seen(lines[i]);
    current = current.intersection(get_seen(lines[i+1]));
    current = current.intersection(get_seen(lines[i+2]));
    ans += score(current.elementAt(0));
  }
  return ans;
}

Set<String> get_seen(String line) {
  Set<String> seen = Set();
  for (var i = 0; i < line.length; i++) {
    seen.add(line[i]);
  }
  return seen;
}

int score(String c) {
  if (c.compareTo("a") != -1 && c.compareTo("z") != 1) {
    return c.codeUnitAt(0) - "a".codeUnitAt(0) + 1;
  }
  return c.codeUnitAt(0) - "A".codeUnitAt(0) + 27;
}