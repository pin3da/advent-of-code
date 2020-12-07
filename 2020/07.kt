import java.io.File

data class Edge(val quantity: Int, val to: String)
class Node(val id: String, val edges: Array<Edge>) {
  var containsGold = -1
  var totalBags = -1
}

class Parser {
  var id: String = "error"
  var edges: MutableList<Edge> = mutableListOf()

  constructor(tokens: ArrayDeque<String>) {
    parseId(tokens)
    tokens.removeFirst() // "contain"
    parseEdges(tokens)
  }

  fun build(): Node {
    return Node(id, edges.toTypedArray())
  }

  private fun parseId(tokens: ArrayDeque<String>) {
    check(tokens.size >= 2)
    id = tokens.removeFirst() + " " + tokens.removeFirst()
    tokens.removeFirst() // "bag(s)"
  }

  private fun parseEdges(tokens: ArrayDeque<String>) {
    if (tokens.size == 0) return
    if (tokens.first() == "no") {
      check(tokens.size == 3)
      tokens.clear()
      return
    }
    val q = tokens.removeFirst().toInt()
    val to = tokens.removeFirst() + " " + tokens.removeFirst()
    tokens.removeFirst() // "bag(s)"
    edges.add(Edge(q, to))
    parseEdges(tokens)
  }
}

fun findGold(node: Node, nodesByIds: Map<String, Node>) {
  if (node.containsGold != -1) return
  node.containsGold = 0
  for (edge in node.edges) {
    val to = nodesByIds[edge.to] ?: error("expected node from ${edge.to}")
    findGold(to, nodesByIds)
    node.containsGold = node.containsGold.or(to.containsGold)
  }
}

fun countAll(node: Node, nodesByIds: Map<String, Node>) {
  if (node.totalBags != -1) return
  node.totalBags = 0
  for (edge in node.edges) {
    val to = nodesByIds[edge.to] ?: error("expected node from ${edge.to}")
    countAll(to, nodesByIds)
    node.totalBags += edge.quantity * (to.totalBags + 1)
  }
}

fun main() {
  var nodes = File("data/07.in").readLines().map {
    Parser(ArrayDeque(it.split(" "))).build()
  }
  val nodesById = nodes.map { Pair(it.id, it) }.toMap()

  var gold = nodesById.getValue("shiny gold")
  gold.containsGold = 1

  var ans = 0
  for (node in nodes) {
    findGold(node, nodesById)
    ans += node.containsGold
  }
  countAll(gold, nodesById)
  println("First part ${ans - 1}")
  println("Second part ${gold.totalBags}")
}
