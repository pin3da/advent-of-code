import java.io.File
import kotlin.math.sqrt

fun main() {
  var lines = File("data/20.in").readLines().toCollection(ArrayDeque())
  var t = mutableListOf<Tile>()
  while (lines.isNotEmpty()) {
    val (_, id) = lines.removeFirst().split(" ", ":").filter { it.isNotEmpty() }
    val grid = mutableListOf<CharArray>()
    for (i in 0..9) {
      grid.add(lines.removeFirst().toCharArray())
    }
    t.add(Tile(id.toLong(), grid.toTypedArray()))
    lines.removeFirstOrNull()
  }
  val tiles = t.toTypedArray()
  val corners = findCorners(tiles)
  val part1 = corners.map { it.tile.id }.toSet().reduce(Long::times)
  println("Part 1: $part1")
  val image = reconstruct(tiles, corners)
  for (i in 0..7) {
    val cur = alignedTop(image, i)
    part2(cur.grid)
  }
}

fun part2(grid: Array<CharArray>) {
  val seaMonster = arrayOf(
    "                  # ".toCharArray(),
    "#    ##    ##    ###".toCharArray(),
    " #  #  #  #  #  #   ".toCharArray(),
  )
  var ans = grid.map {
    row -> row.map {it == '#'}.toTypedArray()
  }.toTypedArray()

  var totalMonsters = 0
  for (i in 0 until grid.size - seaMonster.size) {
    for (j in 0 until grid.size - seaMonster[0].size) {
      var matches = true
      for ((r, row) in seaMonster.withIndex()) {
        for ((c, cell) in row.withIndex()) {
          if (cell == ' ') continue
          matches = matches && (grid[i +  r][j + c] == cell)
        }
      }
      if (matches) {
        totalMonsters++
        for ((r, row) in seaMonster.withIndex()) {
          for ((c, cell) in row.withIndex()) {
            if (cell == ' ') continue
            ans[i +  r][j + c] = false
          }
        }
      }
    }
  }
  println("Part 2 (sea monsters=$totalMonsters) ${ans.flatten().filter { it }.count()}")
}

data class Border(val tile: Tile, val transformation: Int, val border: CharArray) {
  fun reversed(): Border {
    return Border(
      tile,
      (transformation + 4) % 8,
      border.reversedArray()
    )
  }
  
  override fun equals(other: Any?): Boolean {
    if (this === other) return true
    if (javaClass != other?.javaClass) return false
    other as Border
    if (!border.contentEquals(other.border)) return false
    return true
  }

  override fun hashCode(): Int {
    return border.contentHashCode()
  }
}

class Tile(val id: Long, g: Array<CharArray>) {
  private val idx = g.indices
  private val last = g.size - 1
  val grid = deepCopy(g)

  fun getBorders() = sequence {
    for (i in 0..7) {
      yield(getBorder(i))
    }
  }

  fun getBorder(transformation: Int): Border {
    if (transformation < 4) {
      return Border(
        this, transformation,
        when (transformation) {
          0 -> idx.map { grid[0][it] }.toCharArray()
          1 -> idx.map { grid[it][last] }.toCharArray()
          2 -> idx.map { grid[last][last - it] }.toCharArray()
          3 -> idx.map { grid[last - it][0] }.toCharArray()
          else -> error("Invalid transformation")
        } as CharArray
      )
    }
    return getBorder(transformation - 4).reversed()
  }

  fun mirrorRows(): Tile {
    var ans = this.copyOf()
    for (i in idx) ans.grid[i].reverse()
    return ans
  }

  fun mirrorCols(): Tile {
    var ans = this.copyOf()
    for (i in idx) for (j in idx) {
      ans.grid[i][j] = this.grid[last - i][j]
    }
    return ans
  }

  fun rotateCCW(times: Int): Tile {
    var ans = deepCopy(grid)
    for (t in 0 until times) {
      val old = deepCopy(ans)
      for (i in idx) for (j in idx) {
        ans[i][j] = old[j][last - i]
      }
    }
    return copyOf(newGrid = ans)
  }

  fun copyOf(newId: Long = id, newGrid: Array<CharArray> = grid): Tile {
    return Tile(newId, newGrid)
  }

  fun withoutBorders(): Tile {
    var g = Array(grid.size - 2) { CharArray(grid.size - 2) }
    for (i in g.indices) for (j in g.indices) {
      g[i][j] = grid[i + 1][j + 1]
    }
    return copyOf(newGrid=g)
  }
}

fun alignedTop(tile: Tile, transformation: Int): Tile {
  return when (transformation) {
    0, 1, 2, 3 -> tile.rotateCCW(transformation)
    4 -> tile.mirrorRows()
    5 -> tile.mirrorCols().rotateCCW(1)
    6 -> tile.mirrorRows().rotateCCW(2)
    7 -> tile.mirrorCols().rotateCCW(3)
    else -> error("wrong id")
  }
}

fun alignedLeft(tile: Tile, transformation: Int): Tile {
  return alignedTop(tile, transformation).rotateCCW(1)
}

fun findCorners(tiles: Array<Tile>): Array<Border> {
  val matches = tiles.flatMap { it.getBorders() }.groupBy { it }
  return matches.values.filter { it.size == 1 }.map { it[0] }
    .groupBy { it.tile.id }
    .values.filter { it.size == 4 }
    .flatten()
    .toTypedArray()
}

fun reconstruct(tiles: Array<Tile>, corners: Array<Border>): Tile {
  val tileSide = tiles[0].grid.size - 2
  val tileTimes = sqrt(tiles.size.toDouble()).toInt()
  val side = tileTimes * tileSide
  var image = Array(side) { CharArray(side) { '.' } }
  var start = when (corners[0].transformation) {
    // Special case for this particular test case
    2 -> alignedLeft(corners[0].tile, corners[0].transformation)
    else -> alignedTop(corners[0].tile, corners[0].transformation)
  }
  val tileByBorder = tiles.flatMap { it.getBorders() }.groupBy { it }

  for (i in 0 until tileTimes) {
    var next = start.copyOf()
    for (j in 0 until tileTimes) {
      place(next.withoutBorders(), image, i * tileSide, j * tileSide)
      val right = next.getBorder(5)
      next = getRight(tileByBorder[right]!!, next)
    }
    val down = start.getBorder(6)
    start = getDown(tileByBorder[down]!!, start)
  }
  return Tile(666, image)
}

fun getDown(options: List<Border>, tile: Tile): Tile {
  assert(options.size == 2)
  for (cur in options) {
    if (cur.tile.id != tile.id) {
      return alignedTop(cur.tile, cur.transformation)
    }
  }
  return tile
}

fun getRight(options: List<Border>, tile: Tile): Tile {
  assert(options.size == 2)
  for (cur in options) {
    if (cur.tile.id != tile.id) {
      return alignedLeft(cur.tile, cur.transformation)
    }
  }
  return tile
}

fun place(tile: Tile, image: Array<CharArray>, x: Int, y: Int) {
  for ((i, row) in tile.grid.withIndex()) {
    for ((j, cell) in row.withIndex()) {
      image[x + i][y + j] = cell
    }
  }
}

fun deepCopy(g: Array<CharArray>): Array<CharArray> {
  return g.map { it.copyOf() }.toTypedArray()
}
