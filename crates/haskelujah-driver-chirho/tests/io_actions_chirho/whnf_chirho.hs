-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
main = do
  let actionChirho = putStrLn "must-not-run-chirho"
  actionChirho `seq` putStrLn "whnf-chirho"
  return (error "then-payload-chirho" :: Int) >> putStrLn "then-chirho"
  return (error "bind-payload-chirho" :: Int) >>= \_ -> putStrLn "bind-chirho"
