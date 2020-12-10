import java.io.File

fun main() {
  val input = File("data/10.in").readLines().map {
    it.toLong()
  }.sorted().toTypedArray()
  val data = input.plus(input.last() + 3)

  var cur = 0L
  var delta = arrayOf(0, 0, 0, 0)
  for (i in data) {
    assert(i - cur <= 3)
    delta[(i - cur).toInt()]++
    cur = i
  }
  println("Part1 ${delta[1] * delta[3]}")

  var ways = mutableMapOf(Pair(data.last(), 1L))
  println(ways)
  for (i in (data.size - 2) downTo 0) {
    val cur = data[i]
    ways[cur] = 0
    for (j in 1..3) {
      ways[cur] = ways[cur]!! + ways.getOrDefault(cur + j, 0L)
    }
  }
  val total = ways[1]!! + ways[2]!! + ways[3]!!
  println("Part2 $total")
}
