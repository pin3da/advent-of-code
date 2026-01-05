ranges = $stdin.each_line
  .take_while { |line| !line.strip.empty? }
  .map {|line| line.split("-").map(&:to_i).then {|a, b| a..b}}

ids = $stdin.each_line.map(&:to_i)


part1 = ids.count {|id| ranges.any? {|r| r.cover?(id)}}

puts "Part 1: #{part1}"

def merge_all(ranges)
  ranges.sort_by(&:min).each_with_object([]) do |r, merged|
    if merged.empty? || merged.last.max < r.min - 1
      merged << r
    else 
      merged[-1] = merged.last.min..[merged.last.max, r.max].max
    end
  end
end

part2  = merge_all(ranges).sum(&:size)

puts "Part 2: #{part2}"
