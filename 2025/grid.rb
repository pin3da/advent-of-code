class Grid
  attr_reader :width, :height, :data

  def initialize(lines, &parse_line)
    parse_line ||= ->(line) { line.strip.chars }
    @data = lines.map(&parse_line)
    @height = @data.size
    @width = @data.first&.size || 0
  end

  def self.from_io(io = $stdin, &parse_line)
    lines = io.each_line.reject { |line| line.strip.empty? }
    new(lines, &parse_line)
  end

  def self.fill(height, width, value = 0)
    data = Array.new(height) { Array.new(width, value) }
    grid = allocate
    grid.instance_variable_set(:@data, data)
    grid.instance_variable_set(:@height, height)
    grid.instance_variable_set(:@width, width)
    grid
  end

  def valid?(row, col)
    row.between?(0, height - 1) && col.between?(0, width - 1)
  end

  def [](row, col)
    return unless valid?(row, col)
    @data[row][col]
  end

  def []=(row, col, value)
    return unless valid?(row, col)
    @data[row][col] = value
  end

  def each
    return enum_for(:each) unless block_given?

    (0...height).each do |row|
      (0...width).each do |col|
        yield row, col, @data[row][col]
      end
    end
  end

  def neighbors(row, col)
    (-1..1).flat_map do |dr|
      (-1..1).filter_map do |dc|
      next if dr.zero? && dc.zero?

      nr, nc = row + dr, col + dc
      [nr, nc, self[nr, nc]] if valid?(nr, nc)
      end
    end
  end

  def to_s
    @data.map { |row| row.join(" ") }.join("\n")
  end

  def dup
    copy = Grid.fill(height, width)
    each { |r, c, v| copy[r, c] = v }
    copy
  end
end
