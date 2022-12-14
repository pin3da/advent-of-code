import 'dart:io';
import 'dart:convert';
import 'dart:collection';
import 'dart:math';

Future<List<String>> readLines(String filename) async {
  String contents = await File(filename).readAsString();
  return contents.split('\n\n');
}

class Packet {
  List<Packet> items = [];
  int val = -1;

  bool isInt() {
    return val != -1;
  }

  String toString() {
    if (isInt()) {
      return "$val";
    }
    String inner = items.map((it) => it.toString()).join(',');
    return "[$inner]";
  }

  static Packet parse(String input) {
    Queue stack = Queue();
    stack.addLast(Packet());
    Queue<String> buffer = Queue.from(input.split(''));
    while (buffer.isNotEmpty) {
      String token = buffer.first;
      if (token == '[') {
        stack.addLast(Packet());
        buffer.removeFirst();
        continue;
      } 
      if (token == ']') {
        Packet p = stack.removeLast();
        assert(stack.length > 0);
        stack.last.items.add(p);
        buffer.removeFirst();
        continue;
      }
      if (token == ',') {
        buffer.removeFirst();
        continue;
      }
      int value = 0;
      while (isDigit(token)) {
        int digit = int.parse(token);
        value *= 10;
        value += digit;
        buffer.removeFirst();
        token = buffer.first;
      }
      Packet wrap = Packet();
      wrap.val = value;
      stack.last.items.add(wrap);
    }

    assert(stack.length == 1);
    return stack.removeLast().items[0];
  }
}

bool isDigit(String token) {
  try {
    int.parse(token);
    return true;
  } catch (e) {
    return false;
  }
}

Packet makeList(Packet val) {
  Packet p = Packet();
  p.items.add(val);
  return p;
}

// ok, none, notok
int compare(Packet left, Packet right) {
  if (left.isInt() && right.isInt()) {
    if (left.val < right.val) return -1;
    if (left.val > right.val) return 1;
    return 0;
  }
    
  if (!left.isInt() && !right.isInt()) {
    int top = min(left.items.length, right.items.length);
    for (int i = 0; i < top; i++) {
      int order = compare(left.items[i], right.items[i]);
      if (order != 0) return order;
    }
    if (left.items.length < right.items.length) return -1;
    if (left.items.length > right.items.length)  return 1;
    return 0;
  }

  if (left.isInt()) return compare(makeList(left), right);
  return compare(left, makeList(right));
}

void main(List<String> args) async {
  List<String> entries = await readLines('data.in');
  int id = 1;
  int ans = 0;
  List<Packet> all_packets = [];
  for (String entry in entries) {
    List<String> lines = entry.split('\n');
    Packet first = Packet.parse(lines[0]);
    Packet second = Packet.parse(lines[1]);
    all_packets.add(first);
    all_packets.add(second);
    int order = compare(first, second);
    if (order == -1) ans += id;
    id++;
  }
  print("Part 1 $ans");
  String div1 = "[[2]]";
  String div2 = "[[6]]";
  all_packets.add(Packet.parse("[[2]]"));
  all_packets.add(Packet.parse("[[6]]"));
  int ans2 = 1;
  all_packets.sort(compare);
  for (int i = 0; i < all_packets.length; i++) {
    if ([div1, div2].contains(all_packets[i].toString())) {
      ans2 *= (i + 1);
    }
  }
  print("part 2 $ans2");
}