lines = $stdin.each_line.reject { |l| l.strip.empty? }

seps = (0...lines.first.length).select { |i| lines.all? { |l| l[i] == ' ' } }
seps = [0] + seps.map{ |i| i + 1 } + [lines.first.length]

grid = lines.map do |line|
  seps.each_cons(2).map { |s, e| line[s...(e-1)] }
end

ops = grid.pop.map(&:strip)

part1 = grid.first.each_index.sum { |col|
  grid.map {|row| row[col].to_i }.inject(ops[col])
}

puts "Part 1: #{part1}"

part2 = grid.first.each_index.sum { |col|
  block_size = grid.first[col].length

  vals = block_size.times.map do |id_in_block|
    grid.map {|row| row[col][id_in_block] }.join.tr(' ', '').to_i
  end
  vals.inject(ops[col])
}

puts "Part 2: #{part2}"


