class Grid
  attr_reader :width, :height

  def initialize(lines)
    @data = lines.map { |line| line.strip.chars }
    @height = @data.size
    @width = @data.first&.size || 0
  end

  def self.from_io(io = $stdin)
    lines = io.each_line.reject { |line| line.strip.empty? }
    new(lines)
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
end
