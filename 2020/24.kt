import java.io.File

data class Pos(val x: Int, val y: Int) {
  fun plus(o: Pos): Pos {
    return Pos(this.x + o.x, this.y + o.y)
  }
}

val delta = mapOf(
  "e" to Pos(1, 0),
  "ne" to Pos(1, -1),
  "nw" to Pos(0, -1),
  "w" to Pos(-1, 0),
  "sw" to Pos(-1, 1),
  "se" to Pos(0, 1)
)

fun main() {
  val instructions = File("data/24.in").readLines()
  var tiles = mutableMapOf<Pos, Int>()
  for (i in instructions) {
    var inst = i.toCharArray().toCollection(ArrayDeque())
    var start = Pos(0, 0)
    while (inst.isNotEmpty()) {
      var f = inst.removeFirst().toString()
      if (f in setOf("s", "n") && inst.isNotEmpty() && inst.first() in setOf('e', 'w')) {
        f += inst.removeFirst()
      }
      start = start.plus(delta[f] ?: error("failed to fetch $f"))
    }
    tiles[start] = (tiles[start] ?: 0) xor 1
  }
  println(tiles.values.sum())

  for (i in 0..100) {
    var new = tiles.toMutableMap()
    var boxX = tiles.keys.minOf { it.x } - 10..tiles.keys.maxOf { it.x } + 10
    var boxY = tiles.keys.minOf { it.y } - 10..tiles.keys.maxOf { it.y } + 10
    for (x in boxX) for (y in boxY) {
      var c = Pos(x, y)
      val nei = delta.values.map { tiles[c.plus(it)] ?: 0 }.sum()
      when (tiles[c] ?: 0) {
        1 -> if ((nei == 0) || (nei > 2)) new[c] = 0
        else -> if (nei == 2) new[c] = 1
      }
    }
    tiles = new
    println("Day ${i + 1} ${tiles.values.sum()}")
  }

}
