spaceStop = 2

main = do   cs <- getContents
            putStrLn $ expand cs

expand :: String->String
expand s = concatMap convert s

convert :: Char -> String 
convert ' ' = replicate spaceStop ' '
convert c = [c]
