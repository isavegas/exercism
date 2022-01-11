module Grains (square, total) where

import Data.Maybe

square :: Integer -> Maybe Integer
square n
    | v < 0 = Nothing
    | v >= 64 = Nothing
    | otherwise = Just (2 ^ v)
    where v = n - 1

total :: Integer
total = sum (map (\x -> fromMaybe 0 (square x)) [1..64])
