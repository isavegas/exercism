module CryptoSquare (encode) where

import Data.Char
import Data.List

chunk :: Int -> [a] -> [[a]]
chunk _ [] = []
chunk n l = (take n l) : (chunk n $ drop n l)

tabulate :: [Char] -> [[Char]]
tabulate s = chunk ((ceiling $ sqrt $ fromIntegral $ length s) :: Int) s

normalize :: [Char] -> [Char]
normalize s =
            map toLower
            $ filter isAlphaNum s

encode :: String -> String
encode xs =
            unwords
            $ transpose
            $ tabulate
            $ normalize xs
