import java.io.File
import java.util.BitSet

class Entry(val mask: CharArray?, val addr: Long?, val value: Long?)

fun main() {
  val addRegex = "mem\\[(\\d+)]".toRegex()
  val entries = File("data/14.in").readLines()
    .map { l ->
      l.split("=").map { it.trim() }
    }
    .map {
      when (it[0]) {
        "mask" -> Entry(it[1].toCharArray(), null, null)
        else -> {
          val (addr) = addRegex.find(it[0])!!.destructured
          val value = it[1].toLong()
          Entry(null, addr.toLong(), value)
        }
      }
    }
  part1(entries)
  part2(entries)
}

private fun part1(entries: List<Entry>) {
  var mask = CharArray(36)
  var memory = mutableMapOf<Long, Long>()
  for (entry in entries) {
    if (entry.mask != null) {
      mask = entry.mask
    } else {
      memory[entry.addr!!] = applyMask(entry.value!!, mask)
    }
  }
  println("Part 1 ${memory.values.sum()}")
}

private fun part2(entries: List<Entry>) {
  var mask = CharArray(36)
  var memory = mutableMapOf<Long, Long>()
  for (entry in entries) {
    if (entry.mask != null) {
      mask = entry.mask
      continue
    }
    var addr = mask.copyOf()
    val tmp = BitSet.valueOf(longArrayOf(entry.addr!!.toLong()))
    for (i in mask.indices) {
      if (mask[i] != '0') continue
      addr[i] = if (tmp.get(35 - i)) '1' else '0'
    }
    var all = mutableListOf<Long>()
    genAllAddr(addr, 0, 0L, all)
    for (a in all) {
      memory[a] = entry.value!!
    }
  }
  println("Part 2 ${memory.values.sum()}")
}

fun genAllAddr(addr: CharArray, idx: Int, curr: Long, collector: MutableList<Long>) {
  if (idx >= addr.size) {
    collector.add(curr)
    return
  }
  if (addr[idx] == 'X') {
    genAllAddr(addr, idx + 1, curr.shl(1), collector)
    genAllAddr(addr, idx + 1, curr.shl(1) + 1, collector)
    return
  }
  val value = if (addr[idx] == '1') 1 else 0
  genAllAddr(addr, idx + 1, curr.shl(1) + value, collector)
}

fun applyMask(value: Long, mask: CharArray): Long {
  val ans = BitSet.valueOf(longArrayOf(value))
  for ((i, c) in mask.withIndex()) {
    if (c != 'X') {
      ans.set((36 - i), c == '1')
    }
  }
  return ans.toLongArray()[1]
}
