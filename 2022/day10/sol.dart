
import 'dart:io';
import 'dart:convert';
import 'dart:collection';
import 'dart:math';

Future<List<String>> readLines(String filename) async {
  String contents = await File(filename).readAsString();
  const splitter = LineSplitter();
  return splitter.convert(contents);
}

void main(List<String> args) async {
  List<String> lines = await readLines('data.in');
  int time = 1;
  int pixel = 0;
  int x = 1;
  int ans = 0;
  for (var line in lines) {
    var inst = line.split(' ');
    if (inst[0] == "noop") {
      ans += value(time, x);
      pixel = crt(pixel, time, x);
      time++;
    } else {
      ans += value(time, x);
      pixel = crt(pixel, time, x);
      time++;
      ans += value(time, x);
      pixel = crt(pixel, time, x);
      time++;
      x += int.parse(inst[1]);
    }
  }
  print('Part 1 $ans');
}

int crt(int pixel, int time, int x) {
  if ((pixel - x).abs() <= 1) {
    stdout.write('#');
  } else {
    stdout.write('.');
  }
  if (pixel == 39) {
    stdout.write('\n');
    return 0;
  }
  return pixel + 1;
}

int value(int time, int x) {
  if ({20, 60, 100, 140, 180, 220}.contains(time)) {
    return time * x;
  }
  return 0;
}