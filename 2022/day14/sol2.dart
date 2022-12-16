
import 'dart:io';
import 'dart:convert';
import 'dart:collection';
import 'dart:math';
import '../pos.dart';

Future<List<String>> readLines(String filename) async {
  String contents = await File(filename).readAsString();
  return contents.split('\n');
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