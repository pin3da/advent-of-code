class Cluster
  attr_reader :sizes

  def initialize(max_id)
    @ids = Array.new(max_id, &:itself)
    @sizes = Array.new(max_id, 1)
  end

  def root(id)
    return id if @ids[id] == id
    @ids[id] = root(@ids[id])
  end

  def join(a, b)
    root_a, root_b = root(a), root(b)
    return false if root_a == root_b
    @ids[root_b] = root_a
    @sizes[root_a] += @sizes[root_b]
    true
  end

  def cluster_sizes
    @ids.each_with_object({}) do |id, acc|
      r = root(id)
      acc[r] = @sizes[r]
    end.values
  end
end
