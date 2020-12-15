class LastDiff {
  var lastSeen = mutableMapOf<Int, Int>()
  var diff = mutableMapOf<Int, Int>()
  var id = 1

  fun record(a : Int) {
    if (lastSeen.containsKey(a)) {
      diff[a] = id - lastSeen[a]!!
    }
    lastSeen[a] = id
    id++
  }

  fun get(a: Int): Int {
    return diff.getOrDefault(a, 0)
  }
}

fun main () {
  val data = arrayOf(0,1,4,13,15,12,16)
  val diff = LastDiff()
  for (d in data) {
    diff.record(d)
  }
  var last = data.last()
  for (i in data.size until 30000000) {
    last = diff.get(last)
    diff.record(last)
    if (i + 1 == 2020) println("Part 1 $last")
  }
  println("Part 2 $last")
}
