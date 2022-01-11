module Prime (nth) where

-- |Is `b` a factor of `a`?
factor :: Int -> Int -> Bool
factor a b = rem a b == 0

-- |Recursive function to build a list of all found prime
--  numbers. Track last found prime and stop when limit
--  is reached.
primes :: Int -> Int -> [Int] -> [Int]
primes limit lastPrime primeList
    | length primeList > limit - 1 = primeList
    | otherwise = do
                let nextPrime = head
                         $ filter (\candidate -> not $ any (\foundPrime -> factor candidate foundPrime) primeList) [lastPrime+1..]
                primes limit nextPrime (nextPrime : primeList)

nth :: Int -> Maybe Integer
nth n
    | n <= 0 = Nothing
    | n == 1 = Just 2
    | otherwise = Just
                $ toInteger
                $ head
                $ primes n 2 [2]
