import java.io.File

class Instruction {
  var op: String = "nop"
  var arg: Int = 0

  constructor(line: String) {
    val (op, arg) = line.split(" ")
    this.arg = arg.toInt()
    this.op = op
  }
}

class Machine(private val instructions: Array<Instruction>) {
  var memory = 0
  var curId = 0

  private fun runOne() {
    val inst = instructions[curId]
    when (inst.op) {
      "acc" -> {
        memory += inst.arg
        curId++
      }
      "jmp" -> curId += inst.arg
      "nop" -> curId++
      else -> error("unsupported op")
    }
  }

  fun run(): Machine {
    var seen = mutableSetOf<Int>()
    while (curId < instructions.size) {
      if (seen.contains(curId)) {
        return this
      }
      seen.add(curId)
      runOne()
    }
    return this
  }

  fun hasFinished(): Boolean {
    return curId == instructions.size
  }
}

fun main() {
  val instructions = File("data/08.in").readLines().map {
    Instruction(it)
  }.toTypedArray()
  println("Part 1 ${Machine(instructions).run().memory}")

  for (i in instructions.indices) {
    instructions[i].op = swapOp(instructions[i].op)
    var machine = Machine(instructions).run()
    if (machine.hasFinished()) {
      println("Part 2 ${machine.memory}")
    }
    instructions[i].op = swapOp(instructions[i].op)
  }
}

fun swapOp(op: String): String {
  return when (op) {
    "jmp" -> "nop"
    "nop" -> "jmp"
    else -> op
  }
}
