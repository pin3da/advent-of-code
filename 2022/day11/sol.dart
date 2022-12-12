
import 'dart:io';
import 'dart:convert';
import 'dart:collection';
import 'dart:math';

Future<List<String>> readLines(String filename) async {
  String contents = await File(filename).readAsString();
  const splitter = LineSplitter();
  return splitter.convert(contents);
}

class Monkey {
  Queue<Item> items = Queue();
  var operation = (int a) => a + 0;
  int test = 1;
  int go_true = 1;
  int go_false = 1;
  int seen = 0;

  Monkey() {}
}

class Item {
  // all numbers in example and real test cases.
  Map<int, int> mod = {
    11: 0,
    13: 0,
    17: 0,
    19: 0,
    2: 0,
    23: 0,
    3: 0,
    5: 0,
    7: 0,
  };
  int val = 0;

  Item(this.val) {
    for (var key in mod.keys) {
      mod[key] = this.val % key;
    }
  }

  void apply(var operation) {
    for (var key in mod.keys) {
      mod[key] = operation(mod[key]!) % key;
    }
    val = operation(val);
  }
}

void main(List<String> args) async {
  Queue<String> lines = Queue.from(await readLines('example.in'));
  List<Monkey> monkeys = [];
  while (lines.length > 0) {
    monkeys.add(parseMonkey(lines));
  }
  int num_rounds = (args[0] == "first") ? 20 : 10000;
  bool should_divide = (args[0] == "first") ? true : false;

  print("Parsed ${monkeys.length} monkeys");
  for (int round = 0; round < num_rounds; round++) {
    for (var monkey in monkeys) {
      while (monkey.items.isNotEmpty) {
        Item item = monkey.items.removeFirst();
        item.apply(monkey.operation);
        if (should_divide) {
          item.val = item.val ~/ 3;
        }
        int target = monkey.go_false;
        if (should_divide) {
          if ((item.val % monkey.test) == 0) {
            target = monkey.go_true;
          }
        } else {
          if (item.mod[monkey.test]! == 0) {
            target = monkey.go_true;
          }
        }
        monkeys[target].items.addLast(item);
        monkey.seen++;
      }
    }
  }

  var ans = [];
  for (var monkey in monkeys) {
    print("Saw ${monkey.seen}");
    ans.add(monkey.seen);
  }
  ans.sort();
  ans = ans.reversed.toList();
  print(ans[0] * ans[1]);
}

Monkey parseMonkey(Queue<String> lines) {
  while (lines.first.length == 0) {
    lines.removeFirst();
  }
  lines.removeFirst(); // Monkey #:
  Monkey monkey = Monkey();
  var tokens = lines.removeFirst().split(':');
  monkey.items = Queue.from(tokens[1].split(',').map((var it) => Item(int.parse(it))));
  tokens = lines.removeFirst().split('=');
  var operand = tokens[1].substring(7);
  if (tokens[1][5] == '+') {
    if (operand == 'old') monkey.operation = (var a) => a + a;
    else monkey.operation = (var a) => a + int.parse(operand);
  }  else {
    if (operand == 'old') monkey.operation = (var a) => a * a;
    else monkey.operation = (var a) => a * int.parse(operand);
  }
  monkey.test = int.parse(getStrAfter(lines.removeFirst(), "Test: divisible by "));
  monkey.go_true =  int.parse(getStrAfter(lines.removeFirst(), "If true: throw to monkey "));
  monkey.go_false =  int.parse(getStrAfter(lines.removeFirst(), "If false: throw to monkey "));

  return monkey;
}


String getStrAfter(String text, String prefix) {
  return text.trim().substring(prefix.length);
}