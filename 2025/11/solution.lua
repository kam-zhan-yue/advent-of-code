local main_file = io.open("inputs/main.txt", "r")
local test_file = io.open("inputs/test.txt", "r")


local function build_map(file)
  local map = {}

  for line in file:lines() do
    local _, i, key = string.find(line, "(%a+):")
    if i ~= nil then
      local rest = string.sub(line, i, string.len(line))
      local t = {}
      for w in string.gmatch(rest, "%a+") do
        table.insert(t, w)
      end
      map[key] = t
    end
  end
  return map
end

local function solve_map(map)
  local cache = {}
  local function dfs(key)
    if key == "out" then
      return 1
    end

    local total = 0
    for _, v in ipairs(map[key]) do
      if cache[v] ~= nil then
        total = total + cache[v]
      else
        total = total + dfs(v)
      end
    end
    cache[key] = total
    return total
  end

  return dfs("you")
end

local function solve(file)
  local map = build_map(file)
  local part_one = solve_map(map)
  print("Part One is", part_one)
end

solve(test_file)
solve(main_file)
