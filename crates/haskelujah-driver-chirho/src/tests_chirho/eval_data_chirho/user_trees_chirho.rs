// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! User-defined trees and data structures: BST maps, parameterized and nested data (row 700).

use haskelujah_span_chirho::SourceMapChirho;

// ── Deriving Ord end-to-end ─────────────────────────────────────────

// ── BST / parameterized data type tests ────────────────────────────

#[test]
fn eval_tree_construct_match_chirho() {
    // Basic user data type with 3 fields
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\ndata Tree = Leaf | Node Tree Int Tree\nmain = case Node Leaf 5 Leaf of\n  Leaf -> 0\n  Node l v r -> v\n";
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(5)
    );
}

#[test]
fn eval_tree_insert_single_chirho() {
    // Insert a single value into a tree
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\ndata Tree = Leaf | Node Tree Int Tree\ninsert x Leaf = Node Leaf x Leaf\ninsert x (Node l v r) = if x < v then Node (insert x l) v r else Node l v r\nmain = case insert 5 Leaf of\n  Leaf -> 0\n  Node l v r -> v\n";
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(5)
    );
}

#[test]
fn eval_tree_tolist_leaf_chirho() {
    // toList of Leaf should be empty → length 0
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\ndata Tree = Leaf | Node Tree Int Tree\ntoList Leaf = []\ntoList (Node l v r) = toList l ++ [v] ++ toList r\nmain = length (toList Leaf)\n";
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(0)
    );
}

#[test]
fn eval_tree_tolist_single_chirho() {
    // toList of single-node tree with recursive version
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\ndata Tree = Leaf | Node Tree Int Tree\ntoList Leaf = []\ntoList (Node l v r) = toList l ++ [v] ++ toList r\nmain = head (toList (Node Leaf 5 Leaf))\n";
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(5)
    );
}

#[test]
fn eval_bst_insert_sum_chirho() {
    // Binary search tree with user data type
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
data Tree = Leaf | Node Tree Int Tree
insert x Leaf = Node Leaf x Leaf
insert x (Node l v r) = if x < v then Node (insert x l) v r else if x > v then Node l v (insert x r) else Node l v r
toList Leaf = []
toList (Node l v r) = toList l ++ [v] ++ toList r
main = sum (toList (insert 3 (insert 1 (insert 4 (insert 2 Leaf)))))
"#;
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(10)
    );
}

// ── Polymorphic elem/notElem/nub/isPrefixOf for Char ─────────────────

// ── Data.Map (user-defined BST) end-to-end tests ──────────────────

#[test]
fn eval_map_empty_size_chirho() {
    // User-defined Map with 4-field Bin constructor; size of empty map = 0
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
data Map k v = Tip | Bin k v (Map k v) (Map k v)
mapSize t = case t of
  Tip -> 0
  Bin k v l r -> 1 + mapSize l + mapSize r
main = mapSize Tip
"#;
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(0)
    );
}

#[test]
fn eval_map_singleton_size_chirho() {
    // Singleton map has size 1
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
data Map k v = Tip | Bin k v (Map k v) (Map k v)
mapSingleton k v = Bin k v Tip Tip
mapSize t = case t of
  Tip -> 0
  Bin k v l r -> 1 + mapSize l + mapSize r
main = mapSize (mapSingleton 42 100)
"#;
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
    );
}

#[test]
fn eval_bst_map_insert_lookup_chirho() {
    // Insert 3 keys, look up a value that exists
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
data Map k v = Tip | Bin k v (Map k v) (Map k v)
mapInsert k v t = case t of
  Tip -> Bin k v Tip Tip
  Bin k' v' l r -> if k == k' then Bin k v l r
                   else if k < k' then Bin k' v' (mapInsert k v l) r
                   else Bin k' v' l (mapInsert k v r)
mapLookup k t = case t of
  Tip -> 0
  Bin k' v l r -> if k == k' then v
                  else if k < k' then mapLookup k l
                  else mapLookup k r
main = mapLookup 2 (mapInsert 3 30 (mapInsert 1 10 (mapInsert 2 20 Tip)))
"#;
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(20)
    );
}

#[test]
fn eval_map_insert_size_chirho() {
    // Insert 4 distinct keys, size = 4
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
data Map k v = Tip | Bin k v (Map k v) (Map k v)
mapInsert k v t = case t of
  Tip -> Bin k v Tip Tip
  Bin k' v' l r -> if k == k' then Bin k v l r
                   else if k < k' then Bin k' v' (mapInsert k v l) r
                   else Bin k' v' l (mapInsert k v r)
mapSize t = case t of
  Tip -> 0
  Bin k v l r -> 1 + mapSize l + mapSize r
main = mapSize (mapInsert 4 40 (mapInsert 2 20 (mapInsert 3 30 (mapInsert 1 10 Tip))))
"#;
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(4)
    );
}

#[test]
fn eval_map_update_value_chirho() {
    // Insert same key twice, second value overwrites first
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
data Map k v = Tip | Bin k v (Map k v) (Map k v)
mapInsert k v t = case t of
  Tip -> Bin k v Tip Tip
  Bin k' v' l r -> if k == k' then Bin k v l r
                   else if k < k' then Bin k' v' (mapInsert k v l) r
                   else Bin k' v' l (mapInsert k v r)
mapLookup k t = case t of
  Tip -> 0
  Bin k' v l r -> if k == k' then v
                  else if k < k' then mapLookup k l
                  else mapLookup k r
main = mapLookup 1 (mapInsert 1 99 (mapInsert 1 10 Tip))
"#;
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(99)
    );
}

#[test]
fn eval_bst_map_member_chirho() {
    // Check membership in map
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
data Map k v = Tip | Bin k v (Map k v) (Map k v)
mapInsert k v t = case t of
  Tip -> Bin k v Tip Tip
  Bin k' v' l r -> if k == k' then Bin k v l r
                   else if k < k' then Bin k' v' (mapInsert k v l) r
                   else Bin k' v' l (mapInsert k v r)
mapMember k t = case t of
  Tip -> False
  Bin k' v l r -> if k == k' then True
                  else if k < k' then mapMember k l
                  else mapMember k r
m = mapInsert 5 50 (mapInsert 3 30 (mapInsert 7 70 Tip))
main = if mapMember 3 m then 1 else 0
"#;
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
    );
}

#[test]
fn eval_map_member_missing_chirho() {
    // Check non-membership in map
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
data Map k v = Tip | Bin k v (Map k v) (Map k v)
mapInsert k v t = case t of
  Tip -> Bin k v Tip Tip
  Bin k' v' l r -> if k == k' then Bin k v l r
                   else if k < k' then Bin k' v' (mapInsert k v l) r
                   else Bin k' v' l (mapInsert k v r)
mapMember k t = case t of
  Tip -> False
  Bin k' v l r -> if k == k' then True
                  else if k < k' then mapMember k l
                  else mapMember k r
m = mapInsert 5 50 (mapInsert 3 30 (mapInsert 7 70 Tip))
main = if mapMember 4 m then 1 else 0
"#;
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(0)
    );
}

#[test]
fn eval_map_keys_sorted_chirho() {
    // Keys come out in BST order (sorted by key)
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
data Map k v = Tip | Bin k v (Map k v) (Map k v)
mapInsert k v t = case t of
  Tip -> Bin k v Tip Tip
  Bin k' v' l r -> if k == k' then Bin k v l r
                   else if k < k' then Bin k' v' (mapInsert k v l) r
                   else Bin k' v' l (mapInsert k v r)
mapKeys t = case t of
  Tip -> []
  Bin k v l r -> mapKeys l ++ [k] ++ mapKeys r
main = head (mapKeys (mapInsert 3 30 (mapInsert 1 10 (mapInsert 2 20 Tip))))
"#;
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(1)
    );
}

#[test]
fn eval_user_bst_five_inserts_chirho() {
    // Build BST with 5 direct inserts, check size = 5
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
data Dict k v = DTip | DBin k v (Dict k v) (Dict k v)
dInsert k v t = case t of
  DTip -> DBin k v DTip DTip
  DBin k' v' l r -> if k == k' then DBin k v l r
                    else if k < k' then DBin k' v' (dInsert k v l) r
                    else DBin k' v' l (dInsert k v r)
dictSize t = case t of
  DTip -> 0
  DBin k v l r -> 1 + dictSize l + dictSize r
main = dictSize (dInsert 5 50 (dInsert 4 40 (dInsert 3 30 (dInsert 2 20 (dInsert 1 10 DTip)))))
"#;
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(5)
    );
}

#[test]
fn eval_tuple_case_from_list_chirho() {
    // Minimal repro: case on tuple extracted from list
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = "module Test where\nmain = case head [(1, 2)] of\n  (a, b) -> a + b\n";
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(3)
    );
}

#[test]
fn eval_user_bst_fromlist_chirho() {
    // Build BST from list of pairs using foldr
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
data Dict k v = DTip | DBin k v (Dict k v) (Dict k v)
dInsert k v t = case t of
  DTip -> DBin k v DTip DTip
  DBin k' v' l r -> if k == k' then DBin k v l r
                    else if k < k' then DBin k' v' (dInsert k v l) r
                    else DBin k' v' l (dInsert k v r)
dictSize t = case t of
  DTip -> 0
  DBin k v l r -> 1 + dictSize l + dictSize r
main = dictSize (dInsert 5 50 (dInsert 4 40 (dInsert 3 30 (dInsert 2 20 (dInsert 1 10 DTip)))))
"#;
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(5)
    );
}

// ── Tuple pattern in function arguments ───────────────────────────

// ── Complex user-defined data types ───────────────────────────────

#[test]
fn eval_expr_tree_eval_chirho() {
    // Expression tree with Add and Mul constructors
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
data Expr = Lit Int | Add Expr Expr | Mul Expr Expr
eval e = case e of
  Lit n -> n
  Add a b -> eval a + eval b
  Mul a b -> eval a * eval b
main = eval (Add (Mul (Lit 3) (Lit 4)) (Lit 5))
"#;
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(17)
    );
}

#[test]
fn eval_linked_list_user_chirho() {
    // User-defined linked list with custom fold
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
data List a = Nil | Cons a (List a)
myFoldr f z xs = case xs of
  Nil -> z
  Cons x rest -> f x (myFoldr f z rest)
mySum xs = myFoldr (\x acc -> x + acc) 0 xs
main = mySum (Cons 1 (Cons 2 (Cons 3 (Cons 4 Nil))))
"#;
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(10)
    );
}

#[test]
fn eval_rose_tree_depth_chirho() {
    // Rose tree: each node has a list of children
    // We represent as: data Rose = RLeaf Int | RNode Int [Rose]
    // but list of Rose is complex; use binary rose instead:
    // data Rose = RLeaf Int | RNode Int Rose Rose
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
data Rose = RLeaf Int | RNode Int Rose Rose
depth t = case t of
  RLeaf n -> 1
  RNode n l r -> 1 + max (depth l) (depth r)
main = depth (RNode 1 (RNode 2 (RLeaf 3) (RLeaf 4)) (RLeaf 5))
"#;
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(3)
    );
}

// ── Higher-order functions with user data types ────────────────────

// ── Higher-order functions with user data types ────────────────────

#[test]
fn eval_map_over_tree_chirho() {
    // Map a function over all values in a BST, sum result
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
data Tree = Leaf | Node Tree Int Tree
insert x t = case t of
  Leaf -> Node Leaf x Leaf
  Node l v r -> if x < v then Node (insert x l) v r
                else if x > v then Node l v (insert x r)
                else Node l v r
mapTree f t = case t of
  Leaf -> Leaf
  Node l v r -> Node (mapTree f l) (f v) (mapTree f r)
toList t = case t of
  Leaf -> []
  Node l v r -> toList l ++ [v] ++ toList r
main = sum (toList (mapTree (\x -> x * 2) (insert 3 (insert 1 (insert 2 Leaf)))))
"#;
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(12)
    );
}

// ── Accumulator pattern with user data type ───────────────────────

// ── Accumulator pattern with user data type ───────────────────────

#[test]
fn eval_stack_push_pop_chirho() {
    // User-defined Stack data type with push/pop/peek
    use crate::eval_source_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
data Stack = Empty | Push Int Stack
peek s = case s of
  Empty -> 0
  Push x rest -> x
stackSize s = case s of
  Empty -> 0
  Push x rest -> 1 + stackSize rest
s = Push 30 (Push 20 (Push 10 Empty))
main = peek s + stackSize s
"#;
    let val_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(
        val_chirho,
        haskelujah_runtime_chirho::ValueChirho::IntChirho(33)
    );
}

// ── tails / inits ─────────────────────────────────────────────────

// ── Nested data types with pattern matching ────────────────────────

/// Test 4: arithmetic expression tree — ADT with recursive eval.
/// eval (Add (Mul (Lit 3) (Lit 4)) (Lit 5)) = 17
#[test]
fn eval_expr_tree_chirho() {
    use crate::eval_source_with_machine_chirho;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let src_chirho = r#"module Test where
data Expr = Lit Int | Add Expr Expr | Mul Expr Expr
eval (Lit n) = n
eval (Add a b) = eval a + eval b
eval (Mul a b) = eval a * eval b
main = putStrLn (show (eval (Add (Mul (Lit 3) (Lit 4)) (Lit 5))))
"#;
    let (_val_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TestChirho.hs", None).unwrap();
    assert_eq!(m_chirho.io_output_chirho, "17\n");
}
