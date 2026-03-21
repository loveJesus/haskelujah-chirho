-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
-- TEST: compile_and_run
-- EXPECTED: localhost:8080\nexample.com:443\n443
module Main where
data Config = Config
  { cfgHost :: String
  , cfgPort :: Int
  , cfgTls  :: Bool
  }
defaultCfg :: Config
defaultCfg = Config "localhost" 8080 False
showCfg :: Config -> String
showCfg c = cfgHost c ++ ":" ++ show (cfgPort c)
withTls :: Config -> Config
withTls c = Config (cfgHost c) 443 True
main :: IO ()
main = do
  putStrLn (showCfg defaultCfg)
  let prod = withTls (Config "example.com" 80 False)
  putStrLn (showCfg prod)
  print (cfgPort prod)
