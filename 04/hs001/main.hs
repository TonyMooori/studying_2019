main = do   cs <- getContents
            putStrLn $ unlines $ take 10 $ lines cs
