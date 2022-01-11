module Pangram (isPangram) where

import Data.Char

alphabet :: [Char]
alphabet = "abcdefghijklmnopqrstuvwxyz"

isPangram :: String -> Bool
isPangram text = all (True==) (map (\x -> elem x (map toLower text)) alphabet)
