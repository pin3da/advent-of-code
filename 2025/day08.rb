require_relative 'point'
require_relative 'cluster'

def pairs_by_distance(data)
  data.each_with_index.to_a.combination(2).map { | (a, i), (b, j) |
    [a.distance(b), i, j]
  }.sort_by(&:first)
end

data = $stdin.each_line.map { |line| Point.new(line) }
cluster = Cluster.new(data.size)
pairs = pairs_by_distance(data)

pairs.each_with_index do | (dist, a, b), i|
  cluster.join(a, b)

  if i == 1000
    ans = cluster.cluster_sizes.max(3).reduce(:*)
    puts "Part 1: #{ans}"
  end

  if cluster.sizes[cluster.root(a)] == data.size
    puts "Part 2: #{data[a].x * data[b].x}"
    break
  end
end
