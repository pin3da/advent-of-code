import 'dart:collection';
import 'dart:io';
import 'dart:math';

Future<List<String>> readLines(String filename) async {
  String contents = await File(filename).readAsString();
  return contents.split('\n');
}

class Number {
  int n, id;
  Number(this.n, this.id);
}

void main(List<String> args) async {
  List<int> data = (await readLines("data.in")).map(int.parse).toList();
  List<Number> state = [];
  for (int i = 0; i < data.length; i++) state.add(Number(data[i], i));
  
}
