import java.io.File

data class Rule(val name: String, val opt1: IntRange, val opt2: IntRange) {
  fun isValid(field: Int): Boolean {
    return (field in opt1) || (field in opt2)
  }
}

fun main() {
  val lines = ArrayDeque(File("data/16.in").readLines())
  var rules = mutableListOf<Rule>()
  while (lines.first().isNotEmpty()) {
    rules.add(parseRule(lines.removeFirst()))
  }
  lines.removeFirst()
  lines.removeFirst()
  val ticket = parseTicket(lines.removeFirst())
  lines.removeFirst()
  lines.removeFirst()
  val nearby = lines.map { parseTicket(it) }
  part1(rules.toTypedArray(), nearby)
  part2(rules.toTypedArray(), nearby, ticket)
}

fun parseRule(line: String): Rule {
  val (name, r) = line.split(":")
  val (first, _, second) = r.split(" ").filter { it.isNotEmpty() }
  return Rule(name, parseRange(first), parseRange(second))
}

fun parseRange(s: String): IntRange {
  val (a, b) = s.split("-")
  return a.toInt()..b.toInt()
}

fun parseTicket(s: String): Array<Int> {
  return s.split(",").map { it.toInt() }.toTypedArray()
}

fun part1(rules: Array<Rule>, nearby: List<Array<Int>>) {
  val ans = nearby.map { ticket ->
    ticket.filterNot { field ->
      rules.any { it.isValid(field) }
    }.sum()
  }.sum()
  println("Part 1 $ans")
}

fun part2(rules: Array<Rule>, nearby: List<Array<Int>>, ticket: Array<Int>) {
  val valid = nearby.filter { ticket ->
    ticket.all { field ->
      rules.any {
        it.isValid(field)
      }
    }
  }.plusElement(ticket).toTypedArray()

  val validColumns = Array(rules.size) { BooleanArray(rules.size) }
  for ((i, rule) in rules.withIndex()) {
    rules.indices.filter { column ->
      valid.all { ticket ->
        rule.isValid(ticket[column])
      }
    }.forEach { validColumns[i][it] = true }
  }
  val assignment = bipartiteMatching(validColumns)
  var ans = rules.withIndex().filter {
    it.value.name.startsWith("departure")
  }.map {
    ticket[assignment[it.index]].toLong()
  }.reduce(Long::times)

  println("Part 2 $ans")
}

fun bipartiteMatching(validColumns: Array<BooleanArray>): IntArray {
  var rows = IntArray(validColumns.size) { -1 }
  var cols = IntArray(validColumns.size) { -1 }

  var total = 0
  for (i in validColumns.indices) {
    var seen = BooleanArray(validColumns.size)
    if (findMatch(i, validColumns, rows, cols, seen)) {
      total++
    }
  }
  assert(total == validColumns.size)
  return rows
}

fun findMatch(
  i: Int,
  m: Array<BooleanArray>,
  rows: IntArray,
  cols: IntArray,
  seen: BooleanArray
): Boolean {
  for (j in m[0].indices) {
    if (m[i][j] && !seen[j]) {
      seen[j] = true
      if (cols[j] < 0 || findMatch(cols[j], m, rows, cols, seen)) {
        rows[i] = j
        cols[j] = i
        return true
      }
    }
  }
  return false
}
