import java.io.File
import java.math.BigInteger

fun main() {
  var lines = File("data/13.in").readLines()
  val time = lines[0].toInt()
  lines = lines.drop(1).flatMap { it.split(",") }
  val ids = lines.filter { it != "x" }.map { it.toInt() }
  val times = ids.map {
    val next = ((time + it - 1) / it) * it
    Pair(next, it)
  }.sortedWith(compareBy({ it.first }, { it.second }))

  val earliest = times.first()
  println("Part 1 ${earliest.second * (earliest.first - time)}")
  println("Part 2 ${part2(lines)}")
}

data class Rem(val rem: Long, val mod: Long)

fun part2(lines: List<String>): Long {
  val data = lines.mapIndexed { idx, value -> Pair(idx, value) }.filter { it.second != "x" }.map {
    Pair(it.first, it.second.toLong())
  }
  val reminders = data.map { Rem(it.second - (it.first % it.second), it.second) }
  val prodMods = reminders.map { it.mod }.reduce { acc, cur -> acc * cur }
  val otherMods = reminders.map { prodMods / it.mod }
  val invOtherMods = reminders.zip(otherMods) { r, y -> modInv(y, r.mod) }
  return reminders
    .zip(otherMods) { r, y -> (r.rem * y) % prodMods }
    .zip(invOtherMods) { t, z -> (t * z) % prodMods }
    .sum() % prodMods
}

fun modInv(y: Long, mod: Long): Long {
  val a = BigInteger.valueOf(y)
  val m = BigInteger.valueOf(mod)
  return a.modInverse(m).longValueExact()
}
