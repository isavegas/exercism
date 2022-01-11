module Bob (responseFor) where

import           Prelude   (otherwise, (==), (&&), (||), all, any, filter)
import           Data.Char (isAlpha, isUpper, isSpace)
import           Data.Text (Text, empty, last, pack, unpack, strip)

responseFor :: Text -> Text
responseFor xs
    | str == pack "How are you?" = pack "Sure."
    | any isAlpha (unpack str) && all isUpper (filter isAlpha (unpack str)) =
        if last str == '?' then
            pack "Calm down, I know what I'm doing!"
        else
            pack "Whoa, chill out!"
    | str == empty || all isSpace (unpack str) = pack "Fine. Be that way!"
    | last str == '?' = pack "Sure."
    | otherwise = pack "Whatever."
    where str = strip xs
