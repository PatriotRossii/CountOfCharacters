equal :: Char -> Char -> Int
equal x y
    | x == y = 1
    | otherwise = 0

count :: String -> Char -> Int
count string char = (foldr) (+) (0) (map (equal char) (string))

main = do
    line <- getLine
    character <- getChar
    putStrLn (character : " occurs " ++ ((show) (count line character)) ++ " times")