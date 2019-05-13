import System.Environment

main = do
    cs <- getContents
    args <- getArgs
    putStrLn $ unlines $ grep (head args) $ lines cs

grep :: String -> [String] -> [String]
grep key cs = filter (f key) cs
    where
        f :: String -> String -> Bool
        f [] _ = True
        f _ [] = False
        f (k0:k1) (c0:c1) = if k0 == c0
            then k1 == (take (length k1) c1) || f (k0:k1) c1
            else f (k0:k1) c1

