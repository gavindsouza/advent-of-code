# frozen_string_literal: true

NUM = { 'one' => 1, 'two' => 2, 'three' => 3, 'four' => 4, 'five' => 5, 'six' => 6, 'seven' => 7, 'eight' => 8,
        'nine' => 9 }.freeze
PAT = /(?=(#{Regexp.union(NUM.keys + NUM.values.map(&:to_s))}))/.freeze

# Line class to scan the line and hold the result
class Line
  def initialize(line)
    @line = line
  end

  def scan
    @scan ||= @line.scan(PAT)
  end
end

# Day1b: Calculate the sum of the first and last "digit" of each line
module Day1b
  def self.lines
    @lines ||= File.readlines('./inputs/test.txt').map { |line| Line.new(line) }
  end

  def self.calculate
    puts(lines.sum { |line| ((first_digit(line) * 10) + last_digit(line)) })
  end

  def self.find_number(line, reverse: false)
    return line.scan.first&.first unless reverse

    line.scan.last&.first
  end

  def self.first_digit(line)
    s = find_number(line)
    NUM.fetch(s.to_s, s.to_i)
  end

  def self.last_digit(line)
    s = find_number(line, reverse: true)
    NUM.fetch(s.to_s, s.to_i)
  end
end

Day1b.calculate if __FILE__ == $PROGRAM_NAME
