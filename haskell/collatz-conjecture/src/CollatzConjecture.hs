module CollatzConjecture (collatz) where

isEven :: Int -> Bool
isEven v = 0 == (rem v 2)

doCollatz :: Int -> Int -> (Int, Int)
doCollatz i v = if v > 1 then
                    if isEven v then
                        doCollatz (i + 1) (div v 2)
                    else
                        doCollatz (i + 1) ((v * 3) + 1)
                else
                    (i, v)
collatz :: Integer -> Maybe Integer
collatz i
    | i <= 0 = Nothing
    | otherwise = Just ( toInteger (fst (doCollatz 0 (fromIntegral i))))
