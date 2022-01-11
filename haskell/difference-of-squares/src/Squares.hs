module Squares (difference, squareOfSum, sumOfSquares) where

difference :: Integral a => a -> a
difference n = abs (squareOfSum n - sumOfSquares n)

squareOfSum :: Integral a => a -> a
squareOfSum n = sum [1..n] ^2

sumOfSquares :: Integral a => a -> a
sumOfSquares n = sum $ map (\v -> v ^ 2) [1..n]
