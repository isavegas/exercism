module DNA (toRNA) where

import Data.Maybe
import Data.List

comp :: Char -> Char
comp c
    | c == 'G' = 'C'
    | c == 'C' = 'G'
    | c == 'T' = 'A'
    | c == 'A' = 'U'
    | otherwise = error "Invalid character!"

valid :: Char -> Bool
valid c
    | c == 'G' = True
    | c == 'C' = True
    | c == 'T' = True
    | c == 'A' = True
    | otherwise = False

transcribe :: [Char] -> Either Char String
transcribe s = if all valid s then
        Right (map comp s)
    else
        Left (fromMaybe ' ' (find (\x -> not (valid x)) s))

toRNA :: String -> Either Char String
toRNA xs = transcribe xs
