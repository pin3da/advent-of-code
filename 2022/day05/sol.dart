import 'dart:io';
import 'dart:convert';
import 'dart:collection';

Future<List<String>> readLines(String filename) async {
  String contents = await File(filename).readAsString();
  const splitter = LineSplitter();
  return splitter.convert(contents);
}

void main(List<String> args) async {
  List<String> lines = await readLines('data.in');
  int i = 0;
  int num_crates = (lines[0].length + 1) ~/ 4;
  List<Queue<String>> crates = List.generate(num_crates, (_) => Queue());
  while (lines[i] != "") {
    for (int j = 0; j < lines[i].length; j += 4) {
      String? crate = parseCrate(lines[i], j);
      if (crate != null) {
        crates[j ~/ 4].addFirst(crate);
      }
    }
    i++;
  }
  i++;
  while (i < lines.length) {
    List<String> buffer = lines[i].split(" ");
    int blocks = int.parse(buffer[1]);
    int from = int.parse(buffer[3]) - 1;
    int to = int.parse(buffer[5]) - 1;
    Queue<String> tmp = Queue();
    for (int b = 0; b < blocks; b++) {
      if (args[0] == "first") {
        tmp.addLast(crates[from].removeLast());
      } else {
        tmp.addFirst(crates[from].removeLast());
      }
    }
    crates[to].addAll(tmp);
    i++;
  }
  for (var crate in crates) {
    stdout.write(crate.last);
  }
}

String? parseCrate(String line, int start) {
  if (start + 2 >= line.length) {
    return null;
  }
  if (line[start] != "[") {
    return null;
  }
  return line[start + 1];
}