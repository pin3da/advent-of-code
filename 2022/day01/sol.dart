import 'dart:math';
import 'dart:io';
import 'dart:convert';

void main() {
  File('data.in').readAsString().then((String contents) {
    const splitter = LineSplitter();
    var sorted_data = process(splitter.convert(contents));
    var ans1 = sorted_data[0];
    var ans2 = sorted_data[0] + sorted_data[1]  + sorted_data[2];
    print("Part 1: $ans1");  
    print("Part 2: $ans2");  
  });
}

List<int> process(Iterable<String> input) {
  var ans = [0];
  var curr = 0;  
  for (var line in input) {
    if (line == "") {
      ans.add(curr);
      curr = 0;
    } else {
      curr += int.parse(line);
    }
  }
  ans.sort((a, b) => b.compareTo(a));
  return ans;
}