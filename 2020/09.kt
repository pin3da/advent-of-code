import java.io.File

fun part1(data :Array<Long>): Long {
  for (i in 25 until data.size) {
    var seen = false
    for (a in (i - 25) until i) {
      for (b in a until i) {
        if (data[a] + data[b] == data[i]) seen = true
      }
    }
    if (!seen) {
      return data[i]
    }
  }
  assert(false)
  return 0
}

fun main() {
  val data = File("data/09.in").readLines().map {
    it.toLong()
  }.toTypedArray()
  val target = part1(data)
  println("Part 1 $target")
  var seen = mutableMapOf<Long, Int>()
  var acc = 0L
  seen[0] = 0
  for (i in data.indices) {
    acc += data[i]
    val missing = acc - target
    if (data[i] != target && seen.contains(missing)) {
      val interval = data.copyOfRange(seen[missing]!!, i)
      val ans = interval.min()!! + interval.max()!!
      println("Part 2 $ans")
    }
    seen[acc] = i + 1
  }
}
