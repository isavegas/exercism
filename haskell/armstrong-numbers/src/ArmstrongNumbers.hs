module ArmstrongNumbers (armstrong) where

digits :: Integral t => t -> [t]
digits 0 = []
digits value = digits (div value 10) ++ [mod value 10]

calc :: Integral t => t -> t
calc value = do
    let digit_list = digits value
    sum (map (\d -> d ^ (length digit_list)) digit_list)

armstrong :: Integral a => a -> Bool
armstrong value = calc value == value
