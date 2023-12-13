alias Note = Array(Array(Char))

enum ReflectionType
  Horizontal
  Vertical
end

def parse_input(input : String) : Array(Note)
  notes = [[] of Array(Char)]
  input.each_line do |line|
    if line.empty?
      notes << [] of Array(Char)
    else
      notes[-1] << line.chars
    end
  end
  notes
end

def each_horizontal_mirror(note : Note, &)
  note.each_index(start: 1, count: note.size) do |i|
    before = note[...i]
    after = note[i..]

    if before.size > after.size && after == before.reverse[...after.size]
      yield i
    elsif before.reverse == after[...before.size]
      yield i
    end
  end
end

def each_reflection(note : Note, &)
  each_horizontal_mirror(note) do |r|
    yield Tuple.new(r, ReflectionType::Horizontal)
  end

  each_horizontal_mirror(note.transpose) do |r|
    yield Tuple.new(r, ReflectionType::Vertical)
  end
end

def each_part2(note : Note, &)
  note.each_index do |i|
    note[i].each_index do |j|
      b = note[i][j]

      if note[i][j] == '.'
        note[i][j] = '#'
      else
        note[i][j] = '.'
      end
      yield note

      note[i][j] = b
    end
  end
end

def part1(input : String)
  notes = parse_input(input.strip)

  result = 0

  notes.each do |note|
    each_reflection(note) do |r|
      case r[1]
      in .horizontal?
        result += 100 * r[0]
      in .vertical?
        result += r[0]
      end
      break
    end
  end

  result
end

def part2_aux(note : Note)
  real_reflection = each_reflection(note) { |r| break r }
  each_part2(note) do |n|
    each_reflection(note) do |r|
      unless r == real_reflection
        case r[1]
        in .horizontal?
          return 100 * r[0]
        in .vertical?
          return r[0]
        end
        break
      end
    end
  end

  # unreachable
  0
end

def part2(input : String)
  notes = parse_input(input.strip)

  result = 0

  notes.each do |note|
    result += part2_aux(note)
  end

  result
end
