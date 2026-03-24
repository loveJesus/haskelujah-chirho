<!-- For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. — John 3:16 -->

# Haskelujah Standard Library Reference

All modules are built in — no `haskelujah install` needed.

## Haskelujah.JSON

Encode and decode JSON values.

```haskell
import Haskelujah.JSON

main = putStrLn (encode val)
  where val = object ["name" .= String "haskelujah", "version" .= Number 0.1]
```

**Types:** `Value` (Object, Array, String, Number, Bool, Null)
**Functions:** `encode`, `object`, `array`, `(.=)`

## Haskelujah.Test

Built-in test framework.

```haskell
import Haskelujah.Test

main = runTests
    [ test "math" (assertEqual 4 (2 + 2))
    , test "bool" (assertBool "should be true" True)
    ]
```

**Functions:** `test`, `runTests`, `assertEqual`, `assertBool`, `assertFailure`

## Haskelujah.Text

String manipulation utilities.

```haskell
import Haskelujah.Text

main = do
    putStrLn (toUpper "hello")        -- "HELLO"
    putStrLn (capitalize "world")     -- "World"
    print (splitOn "," "a,b,c")       -- ["a","b","c"]
    print (contains "hello" "ell")    -- True
```

**Functions:** `toLower`, `toUpper`, `capitalize`, `contains`, `startsWith`, `endsWith`, `splitOn`, `lines'`, `unlines'`, `padLeft`, `padRight`, `center`

## Haskelujah.Map

Association-list map.

```haskell
import Haskelujah.Map

main = print (lookup "b" m)
  where m = fromList [("a", 1), ("b", 2), ("c", 3)]
```

**Functions:** `empty`, `singleton`, `fromList`, `toList`, `insert`, `lookup`, `delete`, `member`, `size`, `keys`, `elems`, `mapValues`, `filterMap`, `unionWith`

## Haskelujah.Set

Sorted-list set.

```haskell
import Haskelujah.Set

main = print (toList (union s1 s2))
  where s1 = fromList [1,2,3]
        s2 = fromList [3,4,5]
```

**Functions:** `empty`, `singleton`, `fromList`, `toList`, `insert`, `member`, `delete`, `size`, `union`, `intersection`, `difference`

## Haskelujah.Pretty

Pretty printer combinator library.

```haskell
import Haskelujah.Pretty

main = putStrLn (render doc)
  where doc = text "hello" <+> nest 2 (line $$ text "world")
```

**Functions:** `text`, `int`, `nest`, `line`, `(<+>)`, `($$)`, `hsep`, `vsep`, `indent`, `render`, `parens`, `brackets`, `braces`

## Haskelujah.Random

Pseudo-random number generation.

```haskell
import Haskelujah.Random

main = print (fst (randomList 5 (mkGen 42)))
```

**Functions:** `mkGen`, `nextInt`, `nextDouble`, `randomList`, `shuffle`, `choice`

## Haskelujah.Args

Command-line argument parsing.

```haskell
import Haskelujah.Args
import System.Environment (getArgs)

main = do
    args <- getArgs
    if getFlag "verbose" args
        then putStrLn "verbose mode"
        else putStrLn "quiet mode"
```

**Functions:** `getFlag`, `getOption`, `getPositional`

## Haskelujah.Debug

Debugging utilities.

```haskell
import Haskelujah.Debug

main = do
    traceIO "starting..."
    let x = assert (2 + 2 == 4) 42
    print x
```

**Functions:** `trace`, `traceShow`, `traceIO`, `assert`, `todo`, `unreachable`

## Haskelujah.HTTP, .File, .Process, .Time, .Concurrent, .Prelude

See source files in `stdlib-chirho/Haskelujah/` for full API documentation.
