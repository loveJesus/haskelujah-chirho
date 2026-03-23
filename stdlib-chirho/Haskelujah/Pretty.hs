-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16

-- | Built-in pretty printer. No package install needed.
module Haskelujah.Pretty (
    Doc,
    text,
    int,
    nest,
    line,
    (<+>),
    ($$),
    hsep,
    vsep,
    indent,
    render,
    parens,
    brackets,
    braces,
) where

-- | A simple document type for pretty printing.
data Doc
    = Text String
    | Line
    | Nest Int Doc
    | Cat Doc Doc
    | Empty
    deriving (Show)

text :: String -> Doc
text = Text

int :: Int -> Doc
int n = Text (show n)

line :: Doc
line = Line

nest :: Int -> Doc -> Doc
nest = Nest

-- | Horizontal concatenation with space.
(<+>) :: Doc -> Doc -> Doc
Empty <+> y = y
x <+> Empty = x
x <+> y = Cat x (Cat (Text " ") y)

-- | Vertical concatenation.
($$) :: Doc -> Doc -> Doc
x $$ y = Cat x (Cat Line y)

hsep :: [Doc] -> Doc
hsep [] = Empty
hsep [x] = x
hsep (x:xs) = x <+> hsep xs

vsep :: [Doc] -> Doc
vsep [] = Empty
vsep [x] = x
vsep (x:xs) = x $$ vsep xs

indent :: Int -> Doc -> Doc
indent n doc = nest n doc

parens :: Doc -> Doc
parens d = Cat (Text "(") (Cat d (Text ")"))

brackets :: Doc -> Doc
brackets d = Cat (Text "[") (Cat d (Text "]"))

braces :: Doc -> Doc
braces d = Cat (Text "{") (Cat d (Text "}"))

-- | Render a Doc to a String.
render :: Doc -> String
render = go 0
  where
    go _ Empty = ""
    go _ (Text s) = s
    go i Line = "\n" ++ replicate i ' '
    go i (Nest n d) = go (i + n) d
    go i (Cat a b) = go i a ++ go i b
