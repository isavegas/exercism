return {
  compute = function(a, b)
    assert(type(a) == "string" and type(b) == "string", "invalid argument(s)")
    local l = a:len()
    assert(l == b:len(), "inputs not of same length")
    local c = 0
    for i = 0, l do
      if a:sub(i,i) ~= b:sub(i,i) then c = c + 1 end
    end
    return c
  end
}