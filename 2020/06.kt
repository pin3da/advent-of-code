import java.io.File

fun countAny(ans: List<String>): Int {
  return ans.flatMap { it.asIterable() }.toSet().size
}

fun countAll(ans: List<String>): Int {
  return ans.map { it.asIterable().toSet() }.reduce { acc, set -> acc.intersect(set) }.size
}

fun main() {
  var lines = File("data/06.in").readLines()
  var answers = mutableListOf<String>()
  var any = 0
  var all = 0
  for (line in lines) {
    if (line.isEmpty()) {
      any += countAny(answers)
      all += countAll(answers)
      answers = mutableListOf()
    } else {
      answers.add(line)
    }
  }
  any += countAny(answers)
  all += countAll(answers)
  println("Part 1 = $any, part 2 = $all")
}
