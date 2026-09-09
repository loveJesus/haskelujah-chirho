// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Transformer payloads remain lazy until their result is inspected.
//! Expected IO results independently checked with GHC 9.14.1.

use haskelujah_driver::eval_source_with_machine_chirho;
use haskelujah_span_chirho::SourceMapChirho;

fn assert_output_chirho(source_chirho: &str, expected_chirho: &str) {
    let (_, machine_chirho) = eval_source_with_machine_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "TransformerChirho.hs",
        None,
    )
    .unwrap();
    assert_eq!(machine_chirho.io_output_chirho, expected_chirho);
}

#[test]
fn maybe_transformer_short_circuits_io_chirho() {
    assert_output_chirho(
        r#"import Control.Monad.Trans.Maybe (MaybeT(..))
comp :: MaybeT IO Int
comp = bindMaybeT (MaybeT (return Nothing)) (\_ -> error "continuedChirho")
main = do
  resultChirho <- runMaybeT comp
  case resultChirho of
    Just _ -> print 1
    Nothing -> print 0
"#,
        "0\n",
    );
}

#[test]
fn maybe_transformer_binds_an_explicit_io_payload_chirho() {
    assert_output_chirho(
        r#"import Control.Monad.Trans.Maybe (MaybeT(..))
comp :: MaybeT IO Int
comp = bindMaybeT (MaybeT (return (Just 10))) (\x -> returnMaybeT (x + 5))
main = do
  resultChirho <- runMaybeT comp
  case resultChirho of
    Just valueChirho -> print valueChirho
    Nothing -> print 0
"#,
        "15\n",
    );
}

#[test]
fn io_payload_is_lazy_until_a_case_demands_it_chirho() {
    assert_output_chirho(
        r#"main = do
  unusedChirho <- return (error "forcedChirho" :: Int)
  putStrLn "alive"
  valueChirho <- return (Just (error "fieldForcedChirho" :: Int))
  case valueChirho of
    Just _ -> putStrLn "outer"
    Nothing -> putStrLn "wrong"
  numberChirho <- return (20 + 22 :: Int)
  case numberChirho of
    42 -> putStrLn "literal"
    _ -> putStrLn "wrong"
"#,
        "alive\nouter\nliteral\n",
    );
}

#[test]
fn maybe_transformer_uses_the_underlying_maybe_dictionary_chirho() {
    assert_output_chirho(
        r#"import Control.Monad.Trans.Maybe (MaybeT(..))
compChirho :: MaybeT Maybe Int
compChirho = bindMaybeT (MaybeT (Just (Just 10))) (\valueChirho -> returnMaybeT (valueChirho + 5))
stoppedChirho :: MaybeT Maybe Int
stoppedChirho = bindMaybeT (MaybeT Nothing) (\_ -> error "continuedChirho")
main = do
  case runMaybeT compChirho of
    Just (Just valueChirho) -> print valueChirho
    _ -> print 0
  case runMaybeT stoppedChirho of
    Nothing -> putStrLn "stopped"
    _ -> putStrLn "wrong"
"#,
        "15\nstopped\n",
    );
}

#[test]
fn maybe_transformer_preserves_the_underlying_list_choices_chirho() {
    assert_output_chirho(
        r#"import Control.Monad.Trans.Maybe (MaybeT(..))
compChirho :: MaybeT [] Int
compChirho = bindMaybeT (MaybeT [Just 10, Nothing, Just 20]) (\valueChirho -> returnMaybeT (valueChirho + 5))
main = case runMaybeT compChirho of
  [Just firstChirho, Nothing, Just secondChirho] -> do
    print firstChirho
    print secondChirho
  _ -> putStrLn "wrong"
"#,
        "15\n25\n",
    );
}
