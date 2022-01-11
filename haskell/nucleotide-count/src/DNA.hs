module DNA (nucleotideCounts, Nucleotide(..)) where

import           Data.Map   as M (Map, adjust, empty, fromList, insert, lookup)
import           Data.Maybe (fromMaybe)

data Nucleotide = A | C | G | T deriving (Eq, Ord, Show)

toNuc :: Char -> Nucleotide
toNuc c
        | c == 'G' = G
        | c == 'A' = A
        | c =='C' = C
        | c == 'T' = T

count :: String -> (M.Map Nucleotide Int) -> Int -> (M.Map Nucleotide Int)
count xs m i = if i >= length xs then
                    m
                else
                    do
                        let v = (toNuc (xs !! i))
                        let nm = M.adjust (\x -> x + 1) v m
                        count xs nm (i + 1)

valid :: Char -> Bool
valid c
    | c == 'G' = True
    | c == 'C' = True
    | c == 'T' = True
    | c == 'A' = True
    | otherwise = False

nucleotideCounts :: String -> Either String (Map Nucleotide Int)
nucleotideCounts xs = if all valid xs then
                        Right (count xs (fromList
                            [ (A, 0)
                            , (C, 0)
                            , (G, 0)
                            , (T, 0) ]
                        ) 0)
                else
                    Left ""
