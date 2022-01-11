return function(input, func)
  assert(type(input) == "table", "invalid argument")
  local output = {}
  for k,v in pairs(input) do
    output[k] = func(v)
  end
  return output
end