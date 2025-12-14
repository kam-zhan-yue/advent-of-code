local main_file = io.open("../inputs/11.txt", "r")

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

local function fuck_around(map)
  local cache = {}
  local function dfs(key)
    if key == "out" then
      return 1
    end

    if cache[key] ~= nil then
      return cache[key]
    end

    local total = 0
    for _, v in ipairs(map[key]) do
      total = total + dfs(v)
    end
    cache[key] = total
    return total
  end

  return dfs("you")
end


local function find_out(map)
  local cache = {}

  local function get_key(key, fft, dac)
    local fft_str = fft ~= nil and "true" or "false"
    local dac_str = dac ~= nil and "true" or "false"
    return key..fft_str..dac_str
  end

  local function dfs(key, fft, dac)
    if key == "out" then
      if fft and dac then
        return 1
      else
        return 0
      end
    end

    local table_key = get_key(key, fft, dac)
    if cache[table_key] ~= nil then
      return cache[table_key]
    end

    local has_fft = key == "fft" or fft
    local has_dac = key == "dac" or dac

    local total = 0
    for _, v in ipairs(map[key]) do
      total = total + dfs(v, has_fft, has_dac)
    end

    cache[table_key] = total
    return total
  end

  return dfs("svr")
end

local function solve(map, part)
  if part == 1 then
    print("Part One is", fuck_around(map))
  else
    print("Part Two is", find_out(map))
  end
end

local main_map = build_map(main_file)

solve(main_map, 1)
solve(main_map, 2)
