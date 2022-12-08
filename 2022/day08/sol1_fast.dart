import 'dart:io';
import 'dart:convert';
import 'dart:collection';
import 'dart:math';

Future<List<List<int>>> readLines(String filename) async {
  String contents = await File(filename).readAsString();
  const splitter = LineSplitter();
  return splitter.convert(contents).map(
    (line) => line.split("").map(int.parse).toList()
  ).toList();
}


void main(List<String> args) async {
  List<List<int>> trees = await readLines('data.in');
  int rows = trees.length;
  int cols = trees[0].length;

  List<List<bool>> visible_by_row = [];
  List<List<bool>> visible_by_col = [];
  for (var row in trees) {
    visible_by_row.add(List.filled(cols, false));
    visible_by_col.add(List.filled(cols, false));
  }


  for (int i = 0; i < rows; i++) {
    int max_from_left = -1;
    int max_from_right = -1;
    for (int j = 0; j < cols; j++) {
      visible_by_row[i][j] |= (trees[i][j] > max_from_left);
      max_from_left = max(max_from_left, trees[i][j]);

      visible_by_row[i][cols - 1 - j] |= (trees[i][cols - 1 - j] > max_from_right);
      max_from_right = max(max_from_right, trees[i][cols - 1 - j]);
    }
  }

  for (int j = 0; j < cols; j++) {
    int max_from_up = -1;
    int max_from_down = -1;
    for (int i = 0; i < rows; i++) {
      visible_by_col[i][j] |= (trees[i][j] > max_from_up);
      max_from_up = max(max_from_up, trees[i][j]);

      visible_by_col[rows - 1 - i][j] |= (trees[rows - 1 -i][j] > max_from_down);
      max_from_down = max(max_from_down, trees[rows - 1 - i][j]);
    }
  }

  int ans = 0;
  for (int i = 0; i < rows; i++) {
    for (int j = 0; j < cols; j++) {
      if (visible_by_col[i][j] || visible_by_row[i][j]) ans++;
    }
  }
  print("part 1: $ans"); 

}