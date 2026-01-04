
data = $stdin.each_line.map { |line| line.strip.chars.map(&:to_i) }


INVALID = -10 ** 18
def joltage(bank, digits)
  memo = {}

  find_max = lambda do |pos, digits|
    key = [pos, digits]
    return memo[key] if memo.key?(key)

    return 0 if digits.zero?
    return INVALID if pos >= bank.size

    cur = bank[pos] * (10 ** (digits - 1))
    take = cur + find_max.call(pos + 1, digits - 1)
    notake = find_max.call(pos + 1, digits)

    memo[key] = [take, notake].max
  end 

  find_max.call(0, digits)
end

def answer(data, digits)
  ans = data.map { |bank| joltage(bank, digits) }.sum
end

puts "Part 1 #{answer(data, 2)}"
puts "Part 2 #{answer(data, 12)}"
