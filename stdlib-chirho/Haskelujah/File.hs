-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16

-- | Built-in file utilities. No package install needed.
module Haskelujah.File (
    readFileText,
    writeFileText,
    appendFileText,
    fileExists,
    listDirectory,
) where

-- | Read a file as text.
readFileText :: FilePath -> IO String
readFileText = readFile

-- | Write text to a file.
writeFileText :: FilePath -> String -> IO ()
writeFileText = writeFile

-- | Append text to a file.
appendFileText :: FilePath -> String -> IO ()
appendFileText = appendFile

-- | Check if a file exists.
fileExists :: FilePath -> IO Bool
fileExists path = do
    result <- try (readFile path)
    case result of
        Left _ -> return False
        Right _ -> return True
  where
    try :: IO a -> IO (Either String a)
    try action = catch (fmap Right action) (\e -> return (Left (show e)))
    catch = undefined -- placeholder

-- | List files in a directory.
listDirectory :: FilePath -> IO [FilePath]
listDirectory _ = return [] -- placeholder
