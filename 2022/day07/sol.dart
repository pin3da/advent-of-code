import 'dart:io';
import 'dart:convert';
import 'dart:collection';
import 'dart:math';

Future<List<String>> readLines(String filename) async {
  String contents = await File(filename).readAsString();
  const splitter = LineSplitter();
  return splitter.convert(contents);
}


class Node {
  String path;
  bool is_dir;
  int size = 0;
  Map<String, Node> children = {};
  Node? parent;

  Node(this.path, this.is_dir, this.parent) {}
}

void main(List<String> args) async {
  List<String> lines = await readLines('data.in');

  Node root = Node("/", true, null);
  Node current = root;

  int curr_line = 0;
  while (curr_line < lines.length) {
    var parts = lines[curr_line].split(" ");
    assert(parts[0] == "\$");
    if (parts[1] == "cd") {
      CmdResult res = parseCd(parts[2], root, current);
      current = res.node;
      curr_line += res.lines;
    } else {
      assert(parts[1] == "ls");
      CmdResult res = parseLs(lines, curr_line, current);
      current = res.node;
      curr_line += res.lines;
    }
  }

  precomuteSize(root);
  print("Part 1 ${solve1(root)}");
  var to_free = 30000000 - (70000000 - root.size);
  print("Used ${root.size}, need to free ${to_free}");
  print("Part 1 ${solve2(root, to_free)}");
}


int precomuteSize(Node root) {
  if (!root.is_dir) {
    return root.size;
  }
  int total = 0;
  for (var child in root.children.values) {
    total += precomuteSize(child);
  }
  root.size = total;
  return total;
}

int solve1(Node root) {
  if (!root.is_dir) {
    return 0;
  }
  int total = (root.size <= 100000) ? root.size : 0;
  for (var child in root.children.values) {
    total += solve1(child);
  }
  return total;
}

int solve2(Node root, int at_least) {
  if (!root.is_dir) {
    return 70000666;
  }
  int best = (root.size >= at_least) ? root.size : 70000666;
  for (var child in root.children.values) {
    best = min(solve2(child, at_least), best);
  }
  return best;
}


class CmdResult {
  int lines;
  Node node;
  CmdResult(this.lines, this.node) {}
}

CmdResult parseCd(String target, Node root, Node current) {
  if (target == "/") {
    current = root;
  } else if (target == "..") {
    current = current.parent ?? current;
  } else {
    if (current.children.containsKey(target)) {
      current = current.children[target]!;
    } else {
      assert(false, "children not found!");
    }
  }
  return CmdResult(1, current);
}

CmdResult parseLs(List<String> lines, int curr_line, Node current) {
  int lines_read = 1;
  curr_line++;
  while (curr_line < lines.length && lines[curr_line][0] != "\$") {
    List<String> file_desc = lines[curr_line].split(" ");
    String name = file_desc[1];
    if (file_desc[0] == "dir") {
      current.children[name] =  Node(current.path + name + "/", true, current);
    } else {
      Node child = Node(current.path + name + "/", false, current);
      child.size = int.parse(file_desc[0]);
      current.children[name] = child;
    }
    curr_line++;
    lines_read++;
  }

  return CmdResult(lines_read, current);
}