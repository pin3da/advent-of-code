import java.io.File

fun main() {
  val lines = File("data/19.in").readLines().toCollection(ArrayDeque())
  var rules = mutableMapOf<Int, Rule>()
  while (lines.first().isNotEmpty()) {
    addRule(rules, lines.removeFirst())
  }
  assert(lines.removeFirst().isEmpty())
  val ans1 = lines.filter {
    Parser(rules, it.toCollection(ArrayDeque())).parse()
  }.count()
  println("Part 1: $ans1")

  // Part 2: The recursive letters form strings of the way {42}^n{42}^m{31}^m, s.t n > 0, m > 0.
  // Rules 42 and 31 parse a finite set of strings, so each line can be parsed greedily and check that the number of
  // matches of 42 is > than the number of matches of 31.
  // Alternative for problem 1: matches {42}{42}{31}

  val set42 = gen(rules, 42).toSet()
  val set31 = gen(rules, 31).toSet()
  assert(set31.union(set42).size == (set31.size + set42.size))
  val wordSize = set31.first().length
  val ans2 = lines.filter {
    var cur = it
    when (it.length % wordSize) {
      0 -> {
        var first = 0
        while (cur.isNotEmpty() && cur.substring(0, wordSize) in set42) {
          first++
          cur = cur.substring(wordSize)
        }
        var second = 0
        while (cur.isNotEmpty() && cur.substring(0, wordSize) in set31) {
          second++
          cur = cur.substring(wordSize)
        }
        cur.isEmpty() && first > second && second > 0
      }
      else -> false
    }
  }.count()
  println("Part 2: $ans2")
}

sealed class Rule
class Composite(val rules: Array<Int>) : Rule()
class Literal(val c: Char) : Rule()
class Optional(val rules: Array<Rule>) : Rule()

fun addRule(rules: MutableMap<Int, Rule>, line: String) {
  val (id, rhs) = line.split(":").map { it.trim() }
  rules[id.toInt()] = when {
    rhs.startsWith("\"") -> Literal(rhs[1])
    rhs.contains('|') -> parseOptional(rhs)
    else -> parseComposite(rhs)
  }
}

fun parseComposite(line: String): Rule {
  return Composite(
    line.split(" ").filterNot { it.isEmpty() }
      .map { it.toInt() }.toTypedArray()
  )
}

fun parseOptional(line: String): Rule {
  return Optional(
    line.split("|").map {
      parseComposite(it)
    }.toTypedArray()
  )
}

fun gen(rules: MutableMap<Int, Rule>, idx: Int): List<String> {
  return gen(rules, rules[idx]!!)
}

fun gen(rules: MutableMap<Int, Rule>, rule: Rule): List<String> {
  return when (rule) {
    is Literal -> mutableListOf(rule.c.toString())
    is Composite -> {
      var seq = ArrayDeque(rule.rules.map { gen(rules, it) })
      while (seq.size > 1) {
        seq.addFirst(merge(seq.removeFirst(), seq.removeFirst()))
      }
      seq.removeFirst()
    }
    is Optional -> {
      rule.rules.flatMap { gen(rules, it) }.toList()
    }
  }
}

fun merge(left: List<String>, right: List<String>): List<String> {
  var ans = mutableListOf<String>()
  for (l in left) {
    for (r in right) {
      ans.add(l + r)
    }
  }
  return ans
}


class Parser(private val rules: Map<Int, Rule>, private var tokens: ArrayDeque<Char>) {
  fun parse(): Boolean {
    return parse(0) && tokens.isEmpty()
  }

  private fun parse(id: Int): Boolean {
    return parse(rules[id]!!)
  }

  private fun parse(rule: Rule): Boolean {
    val pre = ArrayDeque(tokens)
    val matches = when (rule) {
      is Literal -> tokens.removeFirst() == rule.c
      is Composite -> rule.rules.all { parse(it) }
      is Optional -> rule.rules.any { parse(it) }
    }
    if (matches) {
      return true
    }
    tokens = pre
    return false
  }
}
