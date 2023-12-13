def find_mirror_horizontal(input : Array(String)) : (Int32 | Nil)
  input.each_index(start: 1, count: input.size) do |i|
    before = input[...i]
    after = input[i..]

    if before.size > after.size
      before.reverse!
      if after == before[...after.size]
        return i
      end
    else # after.size >= before.size
      before.reverse!
      if before == after[...before.size]
        return i
      end
    end
  end

  nil
end

def get_column(input, column)
  String.build do |str|
    input.each do |line|
      str << line[column]
    end
  end
end

def transpose(input)
  res = [] of String
  input[0].chars.each_index do |c|
    res << get_column(input, c)
  end
  res
end

def parse_inputs(input)
  inputs = Array(Array(String)).new

  lines = [] of String
  input.each_line do |line|
    if line.empty?
      inputs << lines
      lines = [] of String
    else
      lines << line
    end
  end
  inputs << lines

  inputs
end

enum ReflectionType
  Horizontal
  Vertical
end

def find_reflection(input) : Tuple(Int32, ReflectionType) | Nil
  r = find_mirror_horizontal(input)
  if !r.nil?
    return {r, ReflectionType::Horizontal}
  else
    r = find_mirror_horizontal(transpose(input))
    if r.nil?
      return nil
    else
      return {r, ReflectionType::Vertical}
    end
  end
end

def part1(input)
  inputs = parse_inputs(input.strip)

  result = 0

  inputs.each do |input|
    r = find_reflection(input).not_nil!("")
    if r[1] == ReflectionType::Horizontal
      result += 100 * r[0]
    else
      result += r[0]
    end
  end

  result
end

def char_arr_to_s(char_arr : Array(Char)) : String
  String.build do |str|
    char_arr.each do |c|
      str << c
    end
  end
end

def visualize(input)
  input.each do |line|
    puts line
  end
  puts ""
end

def each_part2(input)
  input.each_index do |i|
    arr = input[i].chars
    arr.each_index do |j|
      b = arr[j]
      if arr[j] == '.'
        arr[j] = '#'
      else
        arr[j] = '.'
      end

      yield input[...i] + [char_arr_to_s(arr)] + input[i + 1..]

      arr[j] = b
    end
  end
end

def find_all_mirror_horizontal(input : Array(String)) : Array(Int32)
  res = [] of Int32

  input.each_index(start: 1, count: input.size) do |i|
    before = input[...i]
    after = input[i..]

    if before.size > after.size
      before.reverse!
      if after == before[...after.size]
        res << i
      end
    else # after.size >= before.size
      before.reverse!
      if before == after[...before.size]
        res << i
      end
    end
  end

  res
end

def find_all_reflection(input) : Array(Tuple(Int32, ReflectionType))
  r1 = find_all_mirror_horizontal(input)
  r2 = find_all_mirror_horizontal(transpose(input))

  res = [] of Tuple(Int32, ReflectionType)

  r1.each do |r|
    res << {r, ReflectionType::Horizontal}
  end

  r2.each do |r|
    res << {r, ReflectionType::Vertical}
  end

  res
end

def try_all_part2(input)
  real_reflection = find_reflection(input).not_nil!("")

  each_part2(input) do |inp|
    visualize(inp)
    r = find_all_reflection(inp)
    r.each do |r|
      if !r.nil? && r != real_reflection
        return r
      end
    end
  end
end

def part2(input)
  inputs = parse_inputs(input.strip)

  result = 0

  inputs.each do |input|
    r = try_all_part2(input).not_nil!("")
    if r[1] == ReflectionType::Horizontal
      result += 100 * r[0]
    else
      result += r[0]
    end
  end

  result
end
