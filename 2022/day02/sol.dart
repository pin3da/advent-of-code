import 'dart:io';
import 'dart:convert';

Future<List<String>> readLines(String filename) async {
  String contents = await File(filename).readAsString();
  const splitter = LineSplitter();
  return splitter.convert(contents);
}

void main() async {
  List<String> lines = await readLines('data.in');
  int game_1 = 0;
  int game_2 = 0;
  for (var shapes in lines) {
    game_1 += game_score(play(shapes[0], shapes[2]));
    game_1 += get_index(shapes[2]) + 1;
    game_2 += game_score(get_index(shapes[2]));
    game_2 += get_index(find_shape(shapes[0], get_index(shapes[2]))) + 1;
  }
  print(game_1);
  print(game_2);
}

int game_score(int outcome) {
  return outcome * 3;
}

int play(String their, String mine) {
  int a = get_index(their);
  int b = get_index(mine);
  if (a == b) return 1;
  if (((b + 1) % 3) == a) return 0;
  return 2;
}

int get_index(String shape) {
  if (shape == "X" || shape == "A") return 0;
  if (shape == "Y" || shape == "B") return 1;
  if (shape == "Z" || shape == "C") return 2;
  return 10000;
}

String find_shape(String their, int outcome) {
  int a = get_index(their);
  int b = (a + outcome + 2) % 3;
  return "ABC"[b];
}