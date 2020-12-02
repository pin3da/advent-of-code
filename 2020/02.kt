import java.io.File

data class Policy(val min: Int, val max: Int, val c: Char)

data class Password(val policy: Policy, val content: String)

fun parsePolicy(input: String): Policy {
  val (times, c) = input.split(" ")
  val (min, max) = times.split("-").map { it.toInt() }
  return Policy(min, max, c[0])
}
fun parsePassword(input: String): Password {
  val (policy, content) = input.split(":")
  return Password(parsePolicy(policy), content.trim())
}

fun isValidFreq(password: Password): Boolean {
  var freq = 0;
  for (i in password.content) {
    if (i == password.policy.c) {
      freq++
    }
  }
  return freq >= password.policy.min && freq <= password.policy.max
}

fun isValidPos(password: Password): Boolean {
  val left = password.content[password.policy.min - 1] == password.policy.c
  val right = password.content[password.policy.max - 1] == password.policy.c
  return left xor right
}

fun main() {
  val passwords = File("data/02.in").readLines().map { parsePassword(it) }
  var ans1 = 0
  var ans2 = 0
  for (p in passwords) {
    if (isValidFreq(p)) ans1++
    if (isValidPos(p)) ans2++
  }
  println(ans1)
  println(ans2)
}
