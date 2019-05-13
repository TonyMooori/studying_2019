main = countbyte

countbyte = do  cs <- getContents
                print $ length cs

countword = do  cs <- getContents
                print $ length $ words cs
                         
