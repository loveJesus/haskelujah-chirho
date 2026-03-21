-- TEST: compile_and_run
-- EXPECTED: Int\nBool\nInt\ntype error
module Main where
data Type = TInt | TBool | TFun Type Type
data Expr = Lit Int | BoolLit Int | Add Expr Expr | IsZero Expr | IfE Expr Expr Expr
typeEq :: Type -> Type -> Int
typeEq TInt TInt = 1; typeEq TBool TBool = 1
typeEq (TFun a1 r1) (TFun a2 r2) = if typeEq a1 a2 == 1 then typeEq r1 r2 else 0
typeEq _ _ = 0
typeCheck :: Expr -> Maybe Type
typeCheck (Lit _) = Just TInt
typeCheck (BoolLit _) = Just TBool
typeCheck (Add a b) = case typeCheck a of
  Just TInt -> case typeCheck b of { Just TInt -> Just TInt; _ -> Nothing }
  _ -> Nothing
typeCheck (IsZero e) = case typeCheck e of { Just TInt -> Just TBool; _ -> Nothing }
typeCheck (IfE c t f) = case typeCheck c of
  Just TBool -> case typeCheck t of
    Just tt -> case typeCheck f of
      Just tf -> if typeEq tt tf == 1 then Just tt else Nothing
      _ -> Nothing
    _ -> Nothing
  _ -> Nothing
showType TInt = "Int"; showType TBool = "Bool"
showType (TFun a r) = showType a ++ " -> " ++ showType r
showResult Nothing = "type error"; showResult (Just t) = showType t
main :: IO ()
main = do
  putStrLn (showResult (typeCheck (Add (Lit 1) (Lit 2))))
  putStrLn (showResult (typeCheck (IsZero (Lit 0))))
  putStrLn (showResult (typeCheck (IfE (IsZero (Lit 0)) (Lit 1) (Lit 2))))
  putStrLn (showResult (typeCheck (Add (Lit 1) (BoolLit 0))))
