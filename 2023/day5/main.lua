---@type function
---@param file string
local function file_exists(file)
    local f = io.open(file, "rb")
    if f then f:close() end
    return f ~= nil
end

---@param numbers string
local function parse_numbers(numbers)
    local parsed = {}
    for n in numbers:gmatch("(%d+)") do
        table.insert(parsed, n)
    end

    return parsed
end

---@param key string
local function parse_key(key)
    local fr_to = {};
    for k in key:gmatch("(%w+)") do
        if k ~= "to" and k ~= "map" then
            table.insert(fr_to, k)
        end
    end
    return fr_to
end

---@param key string
---@return integer, integer, integer 
local function parse_map_key(key)
    local source_idx = key:find(":")
    local source = math.tointeger(key:sub(1, source_idx - 1))
    key = key:sub(source_idx + 1)

    local dest_idx = key:find(":")
    local desc = math.tointeger(key:sub(1, dest_idx - 1))
    key = key:sub(dest_idx + 1)

    local step = math.tointeger(key)

    if source == nil or desc == nil or step == nil then
        return 0, 0, 0
    end

    return source, desc, step
end


---@type function
---@param file string
---@return table, table, table
local function parse_file(file)
    if not file_exists(file) then return {}, {}, {} end

    local glossary = {}
    local seeds = {}
    local order = {}
    local last_key = ""

    ---@param line string
    for line in io.lines(file) do
        if line:match("seeds") then
            local idx = line:find(":")
            local numbers = line:sub(idx + 2, line:len())
            seeds = parse_numbers(numbers)
        elseif line:match("to") then
            last_key = line
            local from, to = table.unpack(parse_key(line))
            table.insert(order, {from, to})
            glossary[from] = {
                [to] = {}
            }
        elseif line:match(" ") then
            local dest, source, step = table.unpack(parse_numbers(line))
            local from, to = table.unpack(parse_key(last_key))
            table.insert(glossary[from][to], source .. ":" .. dest .. ":" .. step)
        end
    end

    return glossary, seeds, order
end
  

local file = './2023/day5/day5.txt'
local glossary, seeds, order = parse_file(file)

local lowest_location = math.tointeger(-1)

for i = 1, #seeds do
    local seed = math.tointeger(seeds[i])
    print("For seed:" .. seed)
    local next_dest = seed
    for j = 1, #order do
        local fr, to = table.unpack(order[j])
        local t = glossary[fr][to]

        local find = false
        print("==============".. fr .. " to " .. to .. "==============")
        for k = 1, #t do
            local source, dest, step = parse_map_key(t[k])
            if next_dest >= source and next_dest < (source + step) then
                print("Find " .. fr .. " at " .. next_dest)
                local delta = next_dest - source
                next_dest = dest + delta
                find = true
                print("Next " .. to .. " is (" .. next_dest ..  "):" .. source .. ":" .. dest .. ":" .. step)
                goto continue
            end
        end

        ::continue::

        if find ~= true then
            print("Find " .. fr .. " at " .. next_dest)
            print("Next " .. to .. " is " .. next_dest)
        end

    end

    if lowest_location == -1 then
        lowest_location = next_dest
    end

    if lowest_location > next_dest then
        lowest_location = next_dest
    end

    print("")
end

print("Lowest location is " .. lowest_location)

-- glossary
-- fr -> to -> [v...]

-- seeds
-- 1 -> i, 2 -> j, ...

-- order
-- 1 -> (fr, to), 2 -> (fr, to), ...


