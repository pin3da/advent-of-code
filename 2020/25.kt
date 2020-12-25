const val mod = 20201227L

fun main() {
  val card = 1327981L
  val door = 2822615L

  val cardSize = findSize(card)
  println(cardSize)
  println(transform(door, cardSize))
}

fun transform(subject: Long, size: Long): Long {
  var ans = 1L
  for (i in 0 until size) {
    ans = (ans * subject) % mod
  }
  return ans
}

fun findSize(card: Long): Long {
  var cur = 1L
  var ans = 0L
  while (cur != card) {
    cur = (cur * 7L) % mod
    ans++
  }
  return ans
}
