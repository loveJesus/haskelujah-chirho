// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
[
    ("operator_0_chirho", r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE UnicodeSyntax #-}
module Main where
(⊗+) :: Int -> Int -> Int
xChirho ⊗+ yChirho = xChirho + yChirho
main :: IO ()
main = print (19 ⊗+ 23)
"###, "42\n"),
    ("operator_1_chirho", r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE UnicodeSyntax #-}
module Main where
(+⊗) :: Int -> Int -> Int
xChirho +⊗ yChirho = xChirho + yChirho
main :: IO ()
main = print (19 +⊗ 23)
"###, "42\n"),
    ("operator_2_chirho", r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE UnicodeSyntax #-}
module Main where
(⊗⊕) :: Int -> Int -> Int
xChirho ⊗⊕ yChirho = xChirho + yChirho
main :: IO ()
main = print (19 ⊗⊕ 23)
"###, "42\n"),
    ("operator_3_chirho", r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE UnicodeSyntax #-}
module Main where
(->⊗) :: Int -> Int -> Int
xChirho ->⊗ yChirho = xChirho + yChirho
main :: IO ()
main = print (19 ->⊗ 23)
"###, "42\n"),
    ("operator_4_chirho", r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE UnicodeSyntax #-}
module Main where
(=⊗) :: Int -> Int -> Int
xChirho =⊗ yChirho = xChirho + yChirho
main :: IO ()
main = print (19 =⊗ 23)
"###, "42\n"),
    ("operator_5_chirho", r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE UnicodeSyntax #-}
module Main where
(--⊗) :: Int -> Int -> Int
xChirho --⊗ yChirho = xChirho + yChirho
main :: IO ()
main = print (19 --⊗ 23)
"###, "42\n"),
    ("operator_6_chirho", r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE UnicodeSyntax #-}
module Main where
(→⊗) :: Int -> Int -> Int
xChirho →⊗ yChirho = xChirho + yChirho
main :: IO ()
main = print (19 →⊗ 23)
"###, "42\n"),
    ("operator_7_chirho", r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE UnicodeSyntax #-}
module Main where
(⊗→) :: Int -> Int -> Int
xChirho ⊗→ yChirho = xChirho + yChirho
main :: IO ()
main = print (19 ⊗→ 23)
"###, "42\n"),
    ("unicode_whitespace_chirho", r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE UnicodeSyntax #-}
module Main where
(⊗+) :: Int -> Int -> Int
xChirho ⊗+ yChirho = xChirho + yChirho
main :: IO ()
main = print (19 ⊗+ 23)
"###, "42\n"),
    ("unicode_identifier_boundary_chirho", r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE UnicodeSyntax #-}
module Main where
(⊗+) :: Int -> Int -> Int
αChirho ⊗+ βChirho = αChirho + βChirho
main :: IO ()
main = print (19 ⊗+ 23)
"###, "42\n"),
    ("symbol_category_4_chirho", r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE UnicodeSyntax #-}
module Main where
(∀→) :: Int -> Int -> Int
xChirho ∀→ yChirho = xChirho + yChirho
main :: IO ()
main = print (19 ∀→ 23)
"###, "42\n"),
    ("standalone_forall_chirho", r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE UnicodeSyntax, ExplicitForAll #-}
module Main where
identityChirho ∷ ∀ aChirho . aChirho → aChirho
identityChirho xChirho = xChirho
main :: IO ()
main = print (identityChirho (42 :: Int))
"###, "42\n"),
    ("dash_whitespace_boundary_chirho", r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
(--|) :: Int -> Int -> Int
xChirho --| yChirho = xChirho + yChirho
-- | this is a comment, not an operator
---- > this is also a comment
main :: IO ()
main = print (19 --| 23)
"###, "42\n"),
]
