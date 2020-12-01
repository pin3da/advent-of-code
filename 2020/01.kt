import java.io.File

fun main() {
  var numbers = File("data/01.in").readLines().map { it.toInt() }
  var complementTuple = hashMapOf<Int,Int>();
  for (i in numbers) {
    for (j in numbers) {
      complementTuple[i + j] = i * j;
    }
  }
  println("First part:" + complementTuple[2020])
  for (i in numbers) {
    if (complementTuple.containsKey(2020 - i)) {
      println("Second part: " + complementTuple[2020 - i]!! * i);
    }
  }
}
