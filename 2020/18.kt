import java.io.File

fun main() {
  val lines = File("data/18.in").readLines().map { s ->
    s.filterNot { it == ' ' }.toCollection(ArrayDeque())
  }
  val pre1 = mapOf('*' to 1, '+' to 1)
  val total1 = lines.map { eval(Parser(it, pre1).parse(0)) }.sum()
  println("Part 1 $total1")
  val pre2 = mapOf('*' to 1, '+' to 2)
  val total2 = lines.map { eval(Parser(it, pre2).parse(0)) }.sum()
  println("Part 2 $total2")
}

sealed class Expr
data class Mul(val left: Expr, val right: Expr) : Expr()
data class Add(val left: Expr, val right: Expr) : Expr()
data class Num(val value: Long) : Expr()

class Parser(tokensOriginal: ArrayDeque<Char>, private val precedence: Map<Char, Int>) {
  var tokens = ArrayDeque(tokensOriginal)
  fun parse(pre: Int): Expr {
    var left: Expr = when (tokens.first()) {
      '(' -> {
        tokens.removeFirst()
        val v = parse(0)
        assert(tokens.removeFirst() == ')')
        v
      }
      else -> Num(tokens.removeFirst().toString().toLong())
    }
    while (pre < nextPrecedence()) {
      val np = nextPrecedence()
      left = when (tokens.removeFirst()) {
        '*' -> Mul(left, parse(np))
        '+' -> Add(left, parse(np))
        else -> error("Unsupported binary op")
      }
    }
    return left
  }

  private fun nextPrecedence(): Int {
    if (tokens.isEmpty()) return 0
    return precedence.getOrDefault(tokens.first(), 0)
  }
}

fun eval(e: Expr): Long {
  return when (e) {
    is Mul -> eval(e.left) * eval(e.right)
    is Add -> eval(e.left) + eval(e.right)
    is Num -> e.value
  }
}
