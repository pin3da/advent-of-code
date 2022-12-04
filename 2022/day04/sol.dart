import 'dart:io';
import 'dart:convert';

Future<List<String>> readLines(String filename) async {
  String contents = await File(filename).readAsString();
  const splitter = LineSplitter();
  return splitter.convert(contents);
}

void main() async {
  List<String> lines = await readLines('data.in');
  int ans1 = 0;
  int ans2 = 0;
  for (var line in lines) {
    var ranges = line.split(",");
    var left = Range.parse(ranges[0]);
    var right = Range.parse(ranges[1]);
    if (left.contains(right) || right.contains(left)) {
      ans1++;
    }
    if (left.overlaps(right)) {
      ans2++;
    }
  }
  print(ans1);
  print(ans2);
}

class Range {
  int begin = 0;
  int end = 0;

  Range.parse(String text) {
    var parts = text.split("-");
    this.begin = int.parse(parts[0]);
    this.end = int.parse(parts[1]);
  }

  String toString() {
    return "[${this.begin}, ${this.end}]";
  }

  bool contains(Range other) {
    if (this.begin >= other.begin && this.end <= other.end) {
      return true;
    }
    return false;
  }

  bool overlaps(Range other) {
    if (this.end < other.begin || other.end < this.begin)  {
      return false;
    }
    return true;
  }
}