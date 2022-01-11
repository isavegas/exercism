module SumOfMultiples (sumOfMultiples, calc, isMultiple, checkFactor) where

-- |Utility math function
isMultiple :: Integer -> Integer -> Bool
isMultiple factor multiple = (rem multiple factor) == 0

-- |Check if a given multiple has a factor in the factor list
checkFactor :: [Integer] -> Integer -> Bool
checkFactor factorList multiple = any (\f -> isMultiple f multiple) factorList

-- |Iterate through valid values and return all matching multiples
calc factorList limit = do
    -- Filter out invalid factors here to ensure we don't need to
    -- re-evaluate the list for every iteration
    let validFactors = filter (\f -> f > 0) factorList
    filter (\value -> checkFactor validFactors value) [1..(limit - 1)]

-- |Invoke the calculation function and sum the results
sumOfMultiples :: [Integer] -> Integer -> Integer
sumOfMultiples factorList limit = sum (calc factorList limit)
