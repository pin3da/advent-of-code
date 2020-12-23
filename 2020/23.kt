data class Node(val value: Int) {
  var next: Node = this
}

class CirList(start: Int) {
  var head = Node(start)
  var nodeById = mutableMapOf(start to head)
  var lastAdded = head
  var size = 1
  fun add(id: Int) {
    var n = Node(id)
    lastAdded.next = n
    n.next = head
    lastAdded = n
    size++
    nodeById[id] = n
  }

  fun pickUpAfter(cur: Node): Node {
    var a = cur.next
    var b = a.next
    var c = b.next
    cur.next = c.next
    return a
  }

  fun insertAfter(id: Int, pickup: Node) {
    var cur = nodeById[id]!!
    var oldNext = cur.next
    cur.next = pickup
    pickup.next.next.next = oldNext
  }
}

fun findNext(cur: Int, start: Node, m: Int): Int {
  val seen = setOf(start.value, start.next.value, start.next.next.value)
  return (1..4).map {
    val d = cur - it
    if (d > 0) d else m + d
  }.first { it !in seen }
}

fun main() {
  // var init = mutableListOf(3, 8, 9, 1, 2, 5, 4, 6, 7) // test data
  var init = mutableListOf(8, 7, 2, 4, 9, 5, 1, 3, 6)
  var part1 = simulate(init.toMutableList(), 100)
  for (i in 1..9) {
    part1 = part1.next
    print(part1.value)
  }
  println()
  while (init.size < 1_000_000) init.add(init.size + 1)
  var part2 = simulate(init.toMutableList(), 10_000_000)
  println("${part2.value}, ${part2.next.value}, ${part2.next.next.value}")
  println(1L * part2.next.value * part2.next.next.value)
}

private fun simulate(init: MutableList<Int>, times: Int): Node {
  var first = init.removeAt(0)
  var circle = CirList(first)
  for (i in init) circle.add(i)
  var start = circle.nodeById[first]!!
  for (i in 1..times) {
    var extra = circle.pickUpAfter(start)
    var next = findNext(start.value, extra, circle.size)
    circle.insertAfter(next, extra)
    start = start.next
  }
  return circle.nodeById[1]!!
}
