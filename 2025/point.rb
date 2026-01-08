class Point
  attr_reader :x, :y, :z

  def initialize(line)
    @x, @y, @z = line.strip.split(",").map(&:to_i)
  end

  def to_s
    "{#{@x}, #{@y}, #{@z}}"
  end

  def distance(o)
    (@x - o.x)**2 + (@y - o.y)**2 + (@z - o.z)**2
  end
end
