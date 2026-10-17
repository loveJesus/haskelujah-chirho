// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! A named record-pattern field belongs to the selected constructor, including
//! nested matches. workflow: language-features-chirho/rigid-type-variables-chirho

use super::{assert_accepted_chirho, assert_output_chirho, assert_rejected_with_chirho};

#[test]
fn a_nested_record_cannot_claim_its_parent_field_chirho() {
    assert_rejected_with_chirho(
        "NestedFieldOwnerChirho.hs",
        r#"module Main where
data InnerChirho = InnerChirho { flagChirho :: Bool }
data OuterChirho = OuterChirho { innerChirho :: InnerChirho, countChirho :: Int }
readChirho (OuterChirho { innerChirho = InnerChirho { flagChirho = fChirho, countChirho = nChirho } }) = (fChirho, nChirho)
main = print (readChirho (OuterChirho (InnerChirho True) 3))
"#,
        "constructor `InnerChirho` does not have a field `countChirho`",
    );
}

#[test]
fn a_record_pattern_cannot_claim_a_sibling_constructor_field_chirho() {
    assert_rejected_with_chirho(
        "SiblingFieldOwnerChirho.hs",
        r#"module Main where
data ChoiceChirho = LeftChirho { leftChirho :: Int } | RightChirho { rightChirho :: Int }
readChirho valueChirho = case valueChirho of { LeftChirho { rightChirho = nChirho } -> nChirho; _ -> 0 }
main = print (readChirho (LeftChirho 3))
"#,
        "constructor `LeftChirho` does not have a field `rightChirho`",
    );
    assert_rejected_with_chirho(
        "PunnedFieldOwnerChirho.hs",
        r#"{-# LANGUAGE NamedFieldPuns #-}
module Main where
data ChoiceChirho = LeftChirho { leftChirho :: Int } | RightChirho { rightChirho :: Int }
readChirho = \LeftChirho { rightChirho } -> rightChirho
main = print (readChirho (LeftChirho 3))
"#,
        "constructor `LeftChirho` does not have a field `rightChirho`",
    );
}

#[test]
fn nested_fields_puns_wildcards_and_omitted_strict_fields_match_chirho() {
    assert_output_chirho(
        "NestedFieldsChirho.hs",
        r#"{-# LANGUAGE NamedFieldPuns, RecordWildCards #-}
module Main where
data InnerChirho aChirho = InnerChirho { flagChirho :: Bool, payloadChirho :: aChirho }
data OuterChirho = OuterChirho { innerChirho :: InnerChirho Char, countChirho :: !Int }
readChirho (OuterChirho { innerChirho = InnerChirho { flagChirho, .. }, .. }) = (flagChirho, payloadChirho, countChirho)
skipChirho OuterChirho {} = 7 :: Int
main = do
  print (readChirho (OuterChirho (InnerChirho True 'x') 3))
  print (skipChirho (OuterChirho (InnerChirho False 'y') 4))
"#,
        "(True,'x',3)\n7\n",
    );
}

#[test]
fn gadt_record_fields_keep_their_declared_schema_chirho() {
    assert_accepted_chirho(
        "GadtRecordFieldsChirho.hs",
        r#"{-# LANGUAGE GADTs #-}
module Main where
data BoxChirho aChirho where
  BoxChirho :: { payloadChirho :: aChirho, tagChirho :: Int } -> BoxChirho aChirho
readChirho :: BoxChirho aChirho -> aChirho
readChirho BoxChirho { payloadChirho = xChirho } = xChirho
main = print (readChirho (BoxChirho 'x' 3))
"#,
    );
}
