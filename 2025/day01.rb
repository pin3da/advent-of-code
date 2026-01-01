MOD = 100

def parse(io = STDIN)
  data = []
  STDIN.each_line do |line|
    line = line.strip
    next if line.empty?

    data << line[1..].to_i * (line[0] == "L" ? -1 : 1)
  end
  data
end

def sing(n)
  n <=> 0
end

data = parse

pos = 50
part1 = 0
part2 = 0

data.each do |d|
  prev = pos
  pos = (pos + d) % MOD
  part1 += 1 if pos == 0

  part2 += d.abs / 100
  part2 += 1 if sing(d) == sing(prev - pos)

end

puts "Part 1: #{part1}"
puts "Part 2: #{part2}"
