require 'set'

class Graph
  attr_reader :nodes

  def initialize
    @nodes = Hash.new { |h, k| h[k] = Set.new }
  end

  def add_edge(from, to)
    @nodes[from] << to
  end

  def to_s
    @nodes.map { |key, val|
      "#{key}: #{val.join(", ")}"
    }.join("\n")
  end
end

graph = Graph.new

$stdin.each_line { |line|
  from, edges = line.split(":").map(&:strip)
  edges.split(" ").each { |to|
    graph.add_edge(from, to)
  }
}


def part1(graph)
  memo = {}
  dfs = ->(node) {
    return 1 if node == "out"
    return memo[node] if memo.key?(node)

    memo[node] = graph.nodes[node].sum { |to| dfs.call(to) }
  }

  dfs.call("you")
end

unless ARGV.include?("part2")
  puts "Part 1 #{part1(graph)}"
end

def part2(graph)
  memo = {}
  dfs = ->(node, dac, fft) {
    if node == "out"
      return dac && fft ?  1 : 0
    end
    key = [node, dac, fft]
    return memo[key] if memo.key?(key)

    memo[key] = graph.nodes[node].sum { |to| 
      dfs.call(to, dac || (node == "dac"), fft || (node == "fft"))
    }
  }

  dfs.call("svr", false, false)
end

unless ARGV.include?("part1")
  puts "Part 2 #{part2(graph)}"
end

