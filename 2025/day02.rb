require 'set'

Interval = Struct.new(:start, :end) do 
  def cover?(n)
    start <= n and n <= self.end
  end
end

intervals = $stdin.each_line.flat_map do |line|
  line = line.strip
  next [] if line.empty? 

  line.split(",").map do |interval|
    Interval.new(*interval.split("-").map(&:to_i))
  end
end

def idfy(i, times)
  (i.to_s * times).to_i
end

def try_invalid(intervals, times)
  ans = Set.new
  intervals.each do |interval|
    for t in 2..times do
      digits = (interval.start ** (1.0/t)).floor.to_s.length - 1
      s = 10 ** digits
      ss = idfy(s, t)
      while ss <= interval.end do
        ans.add(ss) if interval.cover?(ss)
        s += 1
        ss = idfy(s, t)
      end
    end
  end
  ans.sum
end

puts "Part 1 #{try_invalid(intervals, 2)}"
puts "Part 2 #{try_invalid(intervals, 10)}"
