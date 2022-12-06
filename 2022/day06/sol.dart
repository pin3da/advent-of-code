import 'dart:io';
import 'dart:convert';

void main(List<String> args) async {
  String line = await File('data.in').readAsString();
  var len = int.parse(args[0]);
  for (var i = 0; i < line.length - len; i++) {
    Set<String> diff = Set.from(line.substring(i, i + len).split(''));
    if (diff.length == len) {
      print(i + len);
      return;
    }
  } 
}