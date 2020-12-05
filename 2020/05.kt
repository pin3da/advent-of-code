import java.io.File

fun main() {
  var numbers = File("data/05.in").readLines().map { line ->
    var id = 0
    var cur = 64
    for (c in line.dropLast(3)) {
      if (c == 'B') id += cur
      cur /= 2
    }
    id *= 8
    cur = 4
    for (c in line.takeLast(3)) {
      if (c == 'R') id += cur
      cur /= 2
    }
    id
  }.sorted()

  println("Part 1 ${numbers.last()}")
  for (i in 1 until numbers.size) {
    val prev = numbers[i - 1] + 2
    if (prev == numbers[i]) {
      println("Part 2 ${numbers[i] - 1}")
    }
  }
}
