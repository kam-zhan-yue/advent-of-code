defmodule Solution do
  def solve(body, digits) do
    lines = String.split(body)
    solve(lines, digits, 0)
  end

  def solve([head | tail], digits, result) do
    reversed_list = String.graphemes(head) 
      |> Enum.reverse() 
      |> Enum.map(fn x -> String.to_integer(x) end)
    local_max = find_max(reversed_list, digits, [])
    solve(tail, digits, result + local_max)
  end

  def solve([], _digits, result) do IO.puts("Result is #{result}") end

  # Find the maximum in a string
  def find_max([], _digits, vals) do get_max(Enum.reverse(vals), 1, 0) end
  def find_max([head | tail], digits, vals) when length(vals) < digits do
    find_max(tail, digits, [head | vals])
  end
  def find_max([head | tail], digits, vals) do
    updated = update_list(vals, head)
    find_max(tail, digits, updated)
  end

  # Need to loop through the values and find a minimum that can be deleted
  def update_list([], _) do [] end
  def update_list([head | tail], target) when target > head do
    [target | push_down(tail, head)]
  end
  def update_list([head | tail], target) when target < head do
    [head | tail]
  end
  def update_list([head | tail], target) do
    [head | update_list(tail, target)]
  end

  def push_down([], _) do [] end
  def push_down([head | tail], target) when target < head do
    [head | tail]
  end
  def push_down([head | tail], target) do
    [target | push_down(tail, head)]
  end

  # Return the maximum built from the values
  def get_max([], _, result) do result end
  def get_max([head | tail], exponent, result) do
    get_max(tail, exponent * 10, result + head * exponent)
  end
end

{:ok, test} = File.read("inputs/test.txt")
{:ok, main} = File.read("inputs/main.txt")

IO.puts("===Part One===")
Solution.solve(test, 2)
Solution.solve(main, 2)

IO.puts("===Part Two===")
Solution.solve(test, 12)
Solution.solve(main, 12)
