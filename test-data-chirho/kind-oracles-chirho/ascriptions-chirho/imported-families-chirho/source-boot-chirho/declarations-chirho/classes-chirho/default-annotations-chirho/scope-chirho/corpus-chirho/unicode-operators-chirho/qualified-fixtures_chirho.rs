// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
[
    ("qualified_operator_0_chirho", true, &[("ProviderChirho.hs", r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module ProviderChirho where
(⊗+) :: Int -> Int -> Int
xChirho ⊗+ yChirho = xChirho + yChirho
"###),("Main.hs", r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
import qualified ProviderChirho as PChirho
main :: IO ()
main = print (19 PChirho.⊗+ 23)
"###)]),
    ("qualified_operator_1_chirho", true, &[("ProviderChirho.hs", r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module ProviderChirho where
(⊗.+) :: Int -> Int -> Int
xChirho ⊗.+ yChirho = xChirho + yChirho
"###),("Main.hs", r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
import qualified ProviderChirho as PChirho
main :: IO ()
main = print (19 PChirho.⊗.+ 23)
"###)]),
    ("qualified_operator_2_chirho", true, &[("ProviderChirho.hs", r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module ProviderChirho where
(.⊗) :: Int -> Int -> Int
xChirho .⊗ yChirho = xChirho + yChirho
"###),("Main.hs", r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
import qualified ProviderChirho as PChirho
main :: IO ()
main = print (19 PChirho..⊗ 23)
"###)]),
    ("qualified_operator_3_chirho", true, &[("ProviderChirho.hs", r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module ProviderChirho where
(#⊗) :: Int -> Int -> Int
xChirho #⊗ yChirho = xChirho + yChirho
"###),("Main.hs", r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
import qualified ProviderChirho as PChirho
main :: IO ()
main = print (19 PChirho.#⊗ 23)
"###)]),
    ("operator_result_mismatch_chirho", false, &[("Main.hs", r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
(⊗+) :: Int -> Int -> Bool
xChirho ⊗+ yChirho = xChirho + yChirho
main :: IO ()
main = print (19 ⊗+ 23)
"###)]),
]
