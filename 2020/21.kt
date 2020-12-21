import java.io.File

fun main() {
  var foods = File("data/21.in").readLines().map { line ->
    val (ingredients, allergens) = line.split("(", ")")
    Food(
      ingredients.split(" ").filter { it.isNotEmpty() }.toSet(),
      allergens.split(" ", ",").filter { it.isNotEmpty() }.drop(1).toSet()
    )
  }
  val known = mutableMapOf<String, String>()
  while (true) {
    var ingByAller = mutableMapOf<String, Set<String>>()
    for (f in foods) {
      for (a in f.allergens) {
        if (ingByAller.containsKey(a)) {
          ingByAller[a] = f.ingredients.intersect(ingByAller[a]!!)
        } else {
          ingByAller[a] = f.ingredients.toSet()
        }
      }
    }
    var deduced = ingByAller.filter { it.value.size == 1 }
    println("Sure of $deduced")
    for (d in deduced) {
      known[d.key] = d.value.first()
    }
    if (deduced.isEmpty()) break
    var nextFoods = foods.map { food ->
      var ing = food.ingredients subtract deduced.values.map { it.first() }
      var alle = food.allergens subtract deduced.keys
      Food(ing, alle)
    }
    foods = nextFoods
  }
  println(foods.flatMap{ it.ingredients }.count())
  println(known.toSortedMap().map {it.value}.joinToString(separator=","))
}

data class Food(val ingredients: Set<String>, val allergens: Set<String>)
