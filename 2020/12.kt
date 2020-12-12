import java.io.File

data class Action(val type: Char, val value: Int)

class Pos(var x: Int, var y: Int, var dir: Int) {
  fun update(action: Action) {
    when (action.type) {
      'R' -> updateDir(action.value)
      'L' -> updateDir(-action.value)
      'F' -> move(action.value)
      'N' -> y += action.value
      'S' -> y -= action.value
      'E' -> x += action.value
      'W' -> x -= action.value
    }
  }

  private fun updateDir(degrees: Int) {
    check((degrees % 90) == 0) { "Degree must be multiple of 90" }
    val d = degrees / 90
    dir = (dir + d + 4) % 4
  }

  private fun move(value: Int) {
    x += dx[dir] * value
    y += dy[dir] * value
  }

  fun rotateCCW(degrees: Int) {
    for (i in 1..(degrees / 90)) x = -y.also { y = x }
  }

  fun rotateCW(degrees: Int) {
    for (i in 1..(degrees / 90)) x = y.also { y = -x }
  }

  private val dx = arrayOf(1, 0, -1, 0)
  private val dy = arrayOf(0, -1, 0, 1)
}

fun main() {
  var actions = File("data/12.in").readLines().map { Action(it[0], it.substring(1).toInt()) }
  var ship = Pos(0, 0, 0)
  for (action in actions) {
    ship.update(action)
  }
  println("Part 1 ${kotlin.math.abs(ship.x) + kotlin.math.abs(ship.y)}")

  ship = Pos(0, 0, 0)
  var wayPoint = Pos(10, 1, 0)
  for (action in actions) {
    when (action.type) {
      'N', 'S', 'W', 'E' -> wayPoint.update(action)
      'L' -> wayPoint.rotateCCW(action.value)
      'R' -> wayPoint.rotateCW(action.value)
      'F' -> {
        for (i in 1..action.value) {
          ship.x += wayPoint.x
          ship.y += wayPoint.y
        }
      }
      else -> assert(false)
    }
  }
  println("Part 2 ${kotlin.math.abs(ship.x) + kotlin.math.abs(ship.y)}")
}
