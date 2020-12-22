import java.io.File

fun main() {
  var lines = File("data/22.in").readLines().toCollection(ArrayDeque())
  var player1 = parsePlayer(lines)
  var player2 = parsePlayer(lines)
  var part1 = Game(player1.copy(), player2.copy(), false).play()
  println("Part1 ${score(part1)}")
  var part2 = Game(player1, player2, true).play()
  println("Part2 ${score(part2)}")
}

typealias Deck = ArrayDeque<Int>
data class Round(val p1: Deck, val p2: Deck)

class Game(private var p1: Deck, private val p2: Deck, private val isRecursive: Boolean) {
  var seenRounds = mutableSetOf<Round>()
  fun play() : Deck {
    if (p1.isEmpty()) return p2
    if (p2.isEmpty()) return p1
    while (p1.isNotEmpty() && p2.isNotEmpty()) {
      val round = Round(p1, p2)
      if (round in seenRounds) {
        return p1
      }
      seenRounds.add(round)
      var a = p1.removeFirst()
      var b = p2.removeFirst()
      var winner = when {
        (isRecursive && (a <= p1.size) && (b <= p2.size)) -> {
          var subP1 = p1.subList(0, a).toCollection(Deck())
          var subP2 = p2.subList(0, b).toCollection(Deck())
          when (Game(subP1, subP2, true).play()) {
            subP1 -> p1
            subP2 -> p2
            else -> error("no winner")
          }
        }
        else -> {
          if (a > b) p1 else p2
        }
      }
      if (winner == p2) {
        a = b.also { b = a }
      }
      winner.addLast(a)
      winner.addLast(b)
    }
    return if (p1.isNotEmpty()) p1 else p2
  }
}

private fun <E> ArrayDeque<E>.copy(): ArrayDeque<E> {
  return ArrayDeque(this)
}

private fun score(player: Deck): Long {
  if (player.isEmpty()) return 0
  val l = player.size.toLong()
  return player.mapIndexed {idx, c -> c * (l - idx) }.reduce(Long::plus)
}

private fun parsePlayer(lines: ArrayDeque<String>) : Deck {
  assert(lines.removeFirst().startsWith("Player"))
  var player1 = Deck()
  while (lines.firstOrNull()?.isNotEmpty() == true) {
    player1.addLast(lines.removeFirst().toInt())
  }
  lines.removeFirstOrNull()
  return player1
}
