require_relative 'grid'

def positions_to_remove(grid)
  grid.each.filter_map do |r, c, v|
    next unless v == "@"

    neighbor_count = grid.neighbors(r, c).count { |_, _, n| n == "@" }
    [r, c] if neighbor_count < 4
  end
end

grid = Grid.from_io($stdin) { |line| line.strip.chars }
removals = positions_to_remove(grid)
puts "Part 1: #{removals.size}"

ans = 0
until removals.empty?
  ans += removals.size
  removals.each { |r, c| grid[r, c] = "." }
  removals = positions_to_remove(grid)
end

puts "Part 2: #{ans}"
