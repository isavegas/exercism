module PerfectNumbers (classify, Classification(..)) where

data Classification = Deficient | Perfect | Abundant deriving (Eq, Show)

factors :: Int -> [Int]
factors n = [ f | f <- [1..div n 2], rem n f == 0 ]

classify :: Int -> Maybe Classification
classify i
    | i < 1 = Nothing
    | i == a = Just Perfect
    | i < a = Just Abundant
    | i > a = Just Deficient
    | otherwise = Nothing
    where a = sum $ factors i
