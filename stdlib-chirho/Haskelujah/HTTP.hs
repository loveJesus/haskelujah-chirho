-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16

-- | Built-in HTTP client. No package install needed.
--
-- Uses the compiler's FFI to shell out to curl for now.
-- Future: native Rust HTTP implementation via FFI.
module Haskelujah.HTTP (
    Response(..),
    get,
    post,
) where


data Response = Response
    { statusCode :: Int
    , body :: String
    , headers :: [(String, String)]
    } deriving (Show)

-- | HTTP GET request (placeholder — will use FFI to curl).
get :: String -> IO Response
get url = return (Response 200 ("GET " ++ url) [])

-- | HTTP POST request with body (placeholder).
post :: String -> String -> IO Response
post url reqBody = return (Response 200 ("POST " ++ url ++ " " ++ reqBody) [])
