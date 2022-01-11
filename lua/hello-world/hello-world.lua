return {
  hello = function(name)
   return ("Hello, %s!"):format(name or "world")
  end
}