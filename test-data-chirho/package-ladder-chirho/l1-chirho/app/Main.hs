-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

module Main (main) where

import App.MessageChirho (messageChirho)
import HaskellSay (haskellSay)

main :: IO ()
main = haskellSay messageChirho
