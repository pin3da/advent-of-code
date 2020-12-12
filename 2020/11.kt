import java.io.File

fun main() {
  var seats = File("data/11.in").readLines().map { it.toCharArray() }.toTypedArray()
  println("Part 1 ${Simulator(3, 1).simulate(seats)}")
  println("Part 2 ${Simulator(4, 500).simulate(seats)}")
}

class Simulator(private val allowedNeighbors: Int, private val maxSteps: Int) {
  fun simulate(data: Array<CharArray>): Int {
    var seats = data.map { it.copyOf() }.toTypedArray()
    var next = applyRules(seats)
    while (!(seats contentDeepEquals next)) {
      seats = next
      next = applyRules(seats)
    }
    return seats.flatMap { it.asIterable() }.filter { it == '#' }.count()
  }

  private fun applyRules(seats: Array<CharArray>): Array<CharArray> {
    val next = seats.map { it.copyOf() }.toTypedArray()
    for ((i, row) in seats.withIndex()) {
      for ((j, seat) in row.withIndex()) {
        next[i][j] = when (seat) {
          'L' -> occupySeat(seats, 0, i, j)
          '#' -> occupySeat(seats, allowedNeighbors, i, j)
          else -> seat
        }
      }
    }
    return next
  }

  private fun occupySeat(
    seats: Array<CharArray>, maxNeighbors: Int, i: Int, j: Int
  ): Char {
    var occ = 0
    for (k in dx.indices) {
      for (s in 1..maxSteps) {
        val nx = i + s * dx[k]
        val ny = j + s * dy[k]
        if ((nx in seats.indices) && (ny in seats[0].indices)) {
          if (seats[nx][ny] == '#') {
            occ++
            break
          }
          if (seats[nx][ny] == 'L') break
        }
      }
    }

    if (occ <= maxNeighbors) return '#'
    return 'L'
  }

  private val dx = arrayOf(-1, -1, -1, 0, 0, 1, 1, 1)
  private val dy = arrayOf(0, -1, 1, -1, 1, 0, -1, 1)
}
