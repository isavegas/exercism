module Acronym (abbreviate) where

import Data.Char
import Data.List
import qualified Data.Text as T
import Data.Text (Text, pack, unpack)

parse_word :: Text -> Text
parse_word s =
    pack
    $ map T.head
    $ filter (\t -> isUpper $ T.head t)
    $ T.groupBy (\a b -> isUpper(a) && isUpper b) s

-- Super janky, but it works!
first_cap s = T.concat ([pack [toUpper $ T.head s]] ++ [(T.tail s)])

abbreviate :: Text -> Text
abbreviate xs = T.concat
              $ map parse_word
              $ map first_cap
              $ filter (\x -> T.length x > 0)
              $ T.split (\x -> x == ' ' || x == '-') xs
