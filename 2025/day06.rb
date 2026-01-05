lines = $stdin.each_line.reject { |l| l.strip.empty? }

seps = (0...lines.first.length).select { |i| lines.all? { |l| l[i] == ' ' } }
seps = [0] + seps.map{|i| i + 1} + [lines.first.length]

grid = lines.map do |line|
  seps.each_cons(2).map { |s, e| line[s...(e-1)] }
end

grid.last.map!(&:strip)

last = grid.length - 1

total = 0
grid.first.each_index do |col|
  vals = (0...last).map do |row|
    grid[row][col].to_i
  end
  op = grid[last][col]
  total += vals.inject(op)
end

puts "Part 1: #{total}"

total = 0

grid.first.each_index do |col|
  block_size = grid.first[col].length

  vals = (0..block_size-1).map do |id_in_block|
    num = 0
    (0..last-1).each do |row|
      digit = grid[row][col][id_in_block]
      next if digit == " "
      num *= 10
      num += digit.to_i
    end
    num
  end
  op = grid[last][col]
  total += vals.inject(op)
end

puts "Part 2: #{total}"


