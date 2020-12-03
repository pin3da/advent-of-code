import java.io.File

data class Delta(val x: Int, val y: Int)

fun main() {
  var grid = File("data/03.in").readLines()
  var global = 1L
  val deltas = arrayOf(
    Delta(1,1),Delta(3,1),Delta(5,1), Delta(7 , 1), Delta(1,2))
  for (delta in deltas) {
    var x = 0
    var y = 0
    var ans = 0
    while (y < grid.size) {
      if (grid[y][x] == '#') ans++
      x = (x + delta.x) % grid[y].length
      y += delta.y
    }
    println(ans)
    global *= ans
  }
  println(global)
}
