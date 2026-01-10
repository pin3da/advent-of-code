require 'open3'

class Machine
  attr_reader :target, :buttons, :joltage

  def initialize(line)
    @target = line[/\[([.#]+)\]/, 1].chars.map { |c| c == '#' ? 1 : 0 }
    @buttons = line.scan(/\(([^)]+)\)/).map { |m| m[0].split(',').map(&:to_i) }
    @joltage = line[/\{([^}]+)\}/, 1]&.split(',')&.map(&:to_i) || []
  end
end

def solve_part1(machine)
  target = machine.target
  buttons = machine.buttons

  find_min = lambda do |mask, button_idx|
    return 0 if mask == target
    return Float::INFINITY if button_idx >= buttons.size

    skip = find_min.call(mask, button_idx + 1)
    press = find_min.call(toggle(mask, buttons[button_idx]), button_idx + 1) + 1

    [skip, press].min
  end

  find_min.call(Array.new(target.size, 0), 0)
end

def toggle(mask, positions)
  mask.dup.tap { |m| positions.each { |pos| m[pos] ^= 1 } }
end

def solve_part2(machine)
  target = machine.joltage
  buttons = machine.buttons

  return 0 if target.empty? || target.all?(&:zero?)

  lp_problem = build_lp_problem(buttons, target)
  solve_with_glpsol(lp_problem)
end

def build_lp_problem(buttons, target)
  num_positions = target.size
  num_buttons = buttons.size

  # Build coefficient matrix: coeffs[pos][btn] = how many times btn affects pos
  coeffs = Array.new(num_positions) { Array.new(num_buttons, 0) }
  buttons.each_with_index do |btn, btn_idx|
    btn.each { |pos| coeffs[pos][btn_idx] += 1 }
  end

  variables = (0...num_buttons).map { |i| "x#{i}" }

  # Build LP in CPLEX format
  lines = []
  lines << "Minimize"
  lines << "  obj: #{variables.join(' + ')}"
  lines << "Subject To"

  num_positions.times do |pos|
    terms = (0...num_buttons)
      .filter_map { |btn| "#{coeffs[pos][btn]} x#{btn}" if coeffs[pos][btn] > 0 }
    lines << "  c#{pos}: #{terms.join(' + ')} = #{target[pos]}"
  end

  lines << "General"
  lines << "  #{variables.join(' ')}"
  lines << "End"

  lines.join("\n")
end

def solve_with_glpsol(lp_problem)
  stdout, stderr, status = Open3.capture3("glpsol --lp /dev/stdin", stdin_data: lp_problem)
  output = stdout + stderr

  return nil unless status.success? && output.include?("INTEGER OPTIMAL SOLUTION FOUND")

  parse_objective_value(output)
end

def parse_objective_value(output)
  # glpsol outputs objective in two formats:
  #   "mip =   5.000000000e+00 >=" (branch-and-bound)
  #   "Objective value =   5.000000000e+00" (preprocessor)
  case output
  when /mip =\s+([\d.e+\-]+)\s+>=/
    $1.to_f.round
  when /Objective value =\s+([\d.e+\-]+)/
    $1.to_f.round
  end
end

machines = $stdin.each_line.map { |line| Machine.new(line) }

part1 = machines.sum { |m| solve_part1(m) }
puts "Part 1: #{part1}"

part2 = machines.sum { |m| solve_part2(m) }
puts "Part 2: #{part2}"
