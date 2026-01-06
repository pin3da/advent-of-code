require_relative 'grid'

def find_s(grid)
  result  = grid.each.find { |r, c, v| v == 'S' }
  raise "'S' not found" unless result
  result[0..1]
end

grid = Grid.from_io
count = Grid.fill(grid.height, grid.width, 0)

splits = 0
row, col = find_s(grid)
count[row, col] = 1
while row + 1 < grid.height
  grid.width.times do |c|
    next unless grid[row, c] == "S"

    down = [row + 1, c]
    left = [row + 1, c - 1]
    right = [row + 1, c + 1]

    process = ->(pos) {
      return if grid[*pos] == "^"
      count[*pos] += count[row, c]
      grid[*pos] = "S"
    }

    if grid[*down] == "^"
      [left, right].each(&process)
      splits += 1
    else
      process.call(down)
    end
  end
  row += 1
end

puts "Part 1: #{splits}"
puts "Part 2: #{count.data.last.sum}"
