require 'set'

class Buffer
  attr_reader :lines

  def initialize(capacity)
    @capacity = capacity
    @lines = []
  end

  def append(e)
    @lines.push(e)
    if @lines.length > @capacity
      @lines.unshift
    end
  end
end

rx = Regexp.new(Regexp.escape(ARGV[0]));
file = File.open(ARGV[1])
matches = Set.new
buffer = Buffer.new(1000)
splitter = Regexp.new('\A\[(?<context>[^\]]+)\](?: \[[^\]]+\])? \[(?<id>[a-z0-9]+…|[a-f0-9-]+)\] (?<rest>.*)')

count = 0

file.each_line do |line|
  count = count + 1
  if captures = splitter.match(line)
    if matches.include?(captures[:id])
      puts line
    elsif rx.match?(line)
      matches.add(captures[:id])

      buffer.lines.each do |previous|
        if previous_captures = splitter.match(previous)
          if captures[:id] == previous_captures[:id]
            puts previous
          end
        end
      end

      puts line
    end
  end

  buffer.append(line)
end

puts "Searched through #{count} lines";
