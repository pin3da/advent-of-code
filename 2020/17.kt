import java.io.File

data class Coordinate(val x: Int, val y: Int, val z: Int, val w: Int, val useForth: Boolean) {
  fun plus(x: Int, y: Int, z: Int, w: Int): Coordinate {
    return Coordinate(this.x + x, this.y + y, this.z + z, this.w + w, this.useForth)
  }

  fun neighbors() = sequence {
    val start = this@Coordinate
    val rangeW = if (useForth) -1..1 else 0..0
    for (i in -1..1) for (j in -1..1) for (k in -1..1) {
      for (h in rangeW) {
        if (i != 0 || j != 0 || k != 0 || h != 0) {
          yield(start.plus(i, j, k, h))
        }
      }
    }
  }
}

fun main() {
  var grid = mutableMapOf<Coordinate, Boolean>()
  val start = File("data/17.in").readLines().map { it.toCharArray() }.toTypedArray()
  for ((i, row) in start.withIndex()) {
    for ((j, v) in row.withIndex()) {
      grid[Coordinate(i, j, 0, 0, false)] = (v == '#')
    }
  }
  var grid2 = mutableMapOf<Coordinate, Boolean>()
  grid2.putAll(grid.map { Pair(it.key.copy(useForth = true), it.value) })
  println("Part 1 ${computeAll(grid)}")
  println("Part 2 ${computeAll(grid2)}")
}

private fun computeAll(grid: MutableMap<Coordinate, Boolean>): Int {
  var g = grid
  for (i in 0..5) {
    g = next(g, genEmptyGrid(grid.keys.first().useForth))
  }
  return g.values.filter { it }.count()
}

private fun genEmptyGrid(useForth: Boolean) = sequence {
  val max = 13
  val rangeW = if (useForth) -max..max else 0..0
  for (x in -max..max) for (y in -max..max) for (z in -max..max) {
    for (w in rangeW)
      yield(Coordinate(x, y, z, w, useForth))
  }
}

private fun next(
  grid: MutableMap<Coordinate, Boolean>,
  coordinates: Sequence<Coordinate>
): MutableMap<Coordinate, Boolean> {
  var n = mutableMapOf<Coordinate, Boolean>()
  for (c in coordinates) {
    val totalNeighbors = c.neighbors().filter { grid[it] == true }.count()
    if (totalNeighbors == 3) n[c] = true
    if (totalNeighbors == 2 && grid[c] == true) n[c] = true
  }
  return n
}
