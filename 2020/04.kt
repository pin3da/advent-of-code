import java.io.File

fun isValidPassport(passport: List<String>): Boolean {
  val data = passport.flatMap { it.split(" ") }.map { it.split(":") }
  val policies = mapOf(
    "byr" to yearValidator(1920, 2002),
    "iyr" to yearValidator(2010, 2020),
    "eyr" to yearValidator(2020, 2030),
    "hgt" to heightValidator,
    "hcl" to colorValidator,
    "ecl" to eyeColorValidator,
    "pid" to idValidator
  ).withDefault { fun(_: String): Boolean { return true } }

  if (!(data.map { it[0] }.containsAll(policies.keys))) {
    return false
  }
  // first part:
  // return true

  // second part:
  return data.all {
    policies.getValue(it[0]).invoke(it[1])
  }
}

fun yearValidator(min: Int, max: Int): ((String) -> Boolean) {
  return fun(s: String): Boolean {
    if (s.length != 4) return false
    val year = s.toInt()
    return year in min..max;
  }
}

val heightValidator = fun(s: String): Boolean {
  if (!Regex("[0-9]+(in|cm)").matches(s)) {
    return false
  }
  val idxEnd = s.length - 2
  val number = s.substring(0, idxEnd).toInt()
  val unit = s.substring(idxEnd)
  if (unit == "in") {
    return number in 59..76
  }
  return number in 150..193
}

val colorValidator = fun(s: String): Boolean {
  return Regex("#[0-9a-f]{6}").matches(s)
}

val eyeColorValidator = fun(s: String): Boolean {
  return setOf("amb", "blu", "brn", "gry", "grn", "hzl", "oth").contains(s)
}

val idValidator = fun(s: String): Boolean {
  return Regex("[0-9]{9}").matches(s)
}

fun main() {
  var lines = File("data/04.in").readLines()
  var passport = mutableListOf<String>()
  var ans = 0
  for (line in lines) {
    if (line.isEmpty()) {
      if (isValidPassport(passport)) {
        ans++
      }
      passport = mutableListOf()
    } else {
      passport.add(line)
    }
  }
  if (isValidPassport(passport)) {
    ans++
  }
  println(ans)
}
