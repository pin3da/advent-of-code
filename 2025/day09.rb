require_relative 'grid'

def compress_coord(points, idx)
  points.map { |p| p[idx] }
    .sort
    .uniq
    .each_with_index
    .to_h
    .transform_values { |i| i + 1 }
end

def flood_fill(grid, start_row, start_col, fill_value)
  return if grid[start_row, start_col] == fill_value

  queue = [[start_row, start_col]]
  dirs = [[-1, 0], [1, 0], [0, -1], [0, 1]]

  while queue.any?
    r, c = queue.shift
    next unless grid[r, c] == 2

    grid[r, c] = fill_value
    dirs.each { |dr, dc| queue << [r + dr, c + dc] }
  end
end

def connect_points(grid, points, row_coord, col_coord)
  (points + [points.first]).each_cons(2) do |a, b|
    r1, c1 = row_coord[a[0]], col_coord[a[1]]
    r2, c2 = row_coord[b[0]], col_coord[b[1]]

    ([r1, r2].min..[r1, r2].max).each do |r|
      ([c1, c2].min..[c1, c2].max).each do |c|
        grid[r, c] = 1
      end
    end
  end
end

def prefix_sum_2d(grid)
  acc = grid.dup
  acc.data.each do |row|
    (1...row.size).each { |c| row[c] += row[c - 1] }
  end

  (1...acc.height).each do |r|
    acc.width.times { |c| acc[r, c] += acc[r - 1, c] }
  end
  acc
end

def rect_sum(acc, r1, c1, r2, c2)
  min_r, max_r = [r1, r2].minmax
  min_c, max_c = [c1, c2].minmax

  total = acc[max_r, max_c]
  total -= acc[min_r - 1, max_c] if min_r > 0
  total -= acc[max_r, min_c - 1] if min_c > 0
  total += acc[min_r - 1, min_c - 1] if min_r > 0 && min_c > 0
  total
end

def rect_area(a, b)
  ((a[0] - b[0]).abs + 1) * ((a[1] - b[1]).abs + 1)
end

def rect_filled?(acc, ra, ca, rb, cb)
  min_r, max_r = [ra, rb].minmax
  min_c, max_c = [ca, cb].minmax
  expected = (max_r - min_r + 1) * (max_c - min_c + 1)

  rect_sum(acc, min_r, min_c, max_r, max_c) == expected
end

points = $stdin.each_line.map { |line| line.split(",").map(&:to_i).reverse }

part1 = points.combination(2).map { |a, b| rect_area(a, b) }.max
puts "Part 1: #{part1}"

row_coord = compress_coord(points, 0)
col_coord = compress_coord(points, 1)

grid = Grid.fill(row_coord.size + 2, col_coord.size + 2, 2)
connect_points(grid, points, row_coord, col_coord)
flood_fill(grid, 0, 0, 0)
grid.each { |r, c, v| grid[r, c] = 1 if v == 2 }

acc = prefix_sum_2d(grid)

part2 = points.combination(2).map { |a, b|
  ra, ca = row_coord[a[0]], col_coord[a[1]]
  rb, cb = row_coord[b[0]], col_coord[b[1]]

  rect_filled?(acc, ra, ca, rb, cb) ? rect_area(a, b) : 0
}.max

puts "Part 2: #{part2}"
