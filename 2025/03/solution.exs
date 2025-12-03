defmodule Solution do
  def part_one(body) do 
    lines = String.split(body)
    process(lines, 0)
  end

  def process([head | tail], result) do
    reversed_list = String.graphemes(head) |> Enum.reverse()
    local_max = find_max(reversed_list, nil, 0)
    process(tail, result + local_max)
  end

  def process([], result) do IO.puts("Result is #{result}") end

  def find_max([head | tail], right, max) when is_nil(right) do
    head_val = String.to_integer(head)
    find_max(tail, head_val, max)
  end

  def find_max([head | tail], right, max) do
    head_val = String.to_integer(head)
    value = head_val * 10 + right
    highest = if head_val > right do head_val else right end 
    cond do
      value > max -> find_max(tail, highest, value)
      true -> find_max(tail, highest, max)
    end
  end

  def find_max([], _right, max) do max end
end

{:ok, test} = File.read("inputs/test.txt")
{:ok, main} = File.read("inputs/main.txt")
Solution.part_one(test)
Solution.part_one(main)

