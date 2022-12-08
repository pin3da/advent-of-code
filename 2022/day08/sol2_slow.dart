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

  int ans = 0;

  for (int row = 1; row < rows - 1; row++) {
    for (int col = 1; col < cols - 1; col++) {
      int i = row - 1;
      int up = 1;
      for (; i >= 0 && trees[i][col] < trees[row][col]; i--) up++;
      if (i == -1) up--;

      i = row + 1;
      int down = 1;
      for (; i < rows && trees[i][col] < trees[row][col]; i++) down++;
      if (i == rows) down--;

      i = col - 1;
      int left = 1;
      for (; i >= 0 && trees[row][i] < trees[row][col]; i--) left++;
      if (i == -1) left--;
      
      i = col + 1;
      int right = 1;
      for (; i < cols && trees[row][i] < trees[row][col]; i++) right++;
      if (i == cols) right--;

      ans = max(ans, up * down * left * right);
    }
  }

  print("part 2: $ans"); 

}