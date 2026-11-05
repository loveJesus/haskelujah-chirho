// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
//! Row 694 deriving policy on the interpreter, against root's fresh GHC 9.14.1 contracts
//! (#31749): the 18 policy probes and gpt's three direct-method controls (#31759), each source
//! byte for byte, with the sha256 its plan pins. Six probes wrote a bare type variable `a`;
//! their product copies write it `aChirho` (alpha-renamed, the same programs), and each one's
//! comment keeps the measured source's hash without claiming these bytes are that input. A
//! refusal must name its reason through the frontend; an accepted program must print GHC's
//! output.
//! workflow: compiler-pipeline-chirho/deriving-strategies-chirho

use crate::{compile_source_chirho, eval_source_with_machine_chirho};
use haskelujah_span_chirho::SourceMapChirho;

/// What the interpreter prints for an accepted probe.
fn interpreted_chirho(file_name_chirho: &str, source_chirho: &str) -> String {
    let (_, machine_chirho) = eval_source_with_machine_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        file_name_chirho,
        None,
    )
    .unwrap_or_else(|error_chirho| panic!("{file_name_chirho}: interpreter: {error_chirho}"));
    machine_chirho.io_output_chirho
}

/// The frontend refuses a probe GHC refuses, naming every reason.
fn assert_refused_chirho(file_name_chirho: &str, source_chirho: &str, reasons_chirho: &[&str]) {
    let message_chirho = match compile_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        file_name_chirho,
    ) {
        Ok(_) => panic!("{file_name_chirho}: GHC 9.14.1 refuses this probe; it compiled"),
        Err(error_chirho) => error_chirho.to_string(),
    };
    for reason_chirho in reasons_chirho {
        assert!(
            message_chirho.contains(reason_chirho),
            "{file_name_chirho}: refused, but not for `{reason_chirho}`: {message_chirho}"
        );
    }
}

/// A1: sha256 8803c29d66a884ba27be4f555c10ed9990764716182da21a3c29a9a4e1cdeac1.
const STOCK_KEYWORD_DEFAULT_EDITION_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
data ColorChirho = RedChirho | BlueChirho deriving stock (Show)

main :: IO ()
main = print RedChirho
"#;

#[test]
fn policy_a1_stock_keyword_default_edition_chirho() {
    assert_refused_chirho(
        "StockKeywordDefaultEditionChirho.hs",
        STOCK_KEYWORD_DEFAULT_EDITION_CHIRHO,
        &["E0209", "Illegal deriving strategy: stock"],
    );
}

/// A2: sha256 15ba6980d724317f4731676c4cee8a808f1a576f4a3ec8c31e55300cfcd84fac.
const STOCK_KEYWORD_EXTENSION_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DerivingStrategies #-}
data ColorChirho = RedChirho | BlueChirho deriving stock (Show)

main :: IO ()
main = print RedChirho
"#;

#[test]
fn policy_a2_stock_keyword_extension_chirho() {
    assert_eq!(
        interpreted_chirho(
            "StockKeywordExtensionChirho.hs",
            STOCK_KEYWORD_EXTENSION_CHIRHO
        ),
        "RedChirho\n"
    );
}

/// A3: sha256 3f4f5fcd6dab9f1caabf9ea32ff49e9132878d27faaf6bd247f78ad6fedd68f6.
const STOCK_KEYWORD_GHC_2_0_2_4_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GHC2024 #-}
data ColorChirho = RedChirho | BlueChirho deriving stock (Show)

main :: IO ()
main = print RedChirho
"#;

#[test]
fn policy_a3_stock_keyword_ghc_2_0_2_4_chirho() {
    assert_eq!(
        interpreted_chirho(
            "StockKeywordGhc2024Chirho.hs",
            STOCK_KEYWORD_GHC_2_0_2_4_CHIRHO
        ),
        "RedChirho\n"
    );
}

/// B1: alpha-renamed from root's measured source (sha256 e4601b9ec08be1aa80425989758f9d45c168c142fb72e333c58c6b9905093c33), bare `a` written
/// `aChirho`; these bytes are not the measured GHC input.
const ANY_CLASS_UNWRITTEN_OFF_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
class DescribeChirho aChirho where
  describeChirho :: aChirho -> String
  describeChirho _ = "described"

data PointChirho = PointChirho deriving (DescribeChirho)

main :: IO ()
main = putStrLn (describeChirho PointChirho)
"#;

#[test]
fn policy_b1_any_class_unwritten_off_chirho() {
    assert_refused_chirho(
        "AnyClassUnwrittenOffChirho.hs",
        ANY_CLASS_UNWRITTEN_OFF_CHIRHO,
        &[
            "E0209",
            "`DescribeChirho PointChirho`",
            "not a stock derivable class",
        ],
    );
}

/// B2: alpha-renamed from root's measured source (sha256 9f74cf9ca1f01b61d2c09f58c44e3c6e11eee8b4d386eac72dc61012afa90b32), bare `a` written
/// `aChirho`; these bytes are not the measured GHC input.
const ANY_CLASS_UNWRITTEN_ON_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DeriveAnyClass #-}
class DescribeChirho aChirho where
  describeChirho :: aChirho -> String
  describeChirho _ = "described"

data PointChirho = PointChirho deriving (DescribeChirho)

main :: IO ()
main = putStrLn (describeChirho PointChirho)
"#;

#[test]
fn policy_b2_any_class_unwritten_on_chirho() {
    assert_eq!(
        interpreted_chirho(
            "AnyClassUnwrittenOnChirho.hs",
            ANY_CLASS_UNWRITTEN_ON_CHIRHO
        ),
        "described\n"
    );
}

/// B3: alpha-renamed from root's measured source (sha256 5cbeaf8ccc373b533c34508d1ae2db5a7c39f72cfe5d5d9c3446fdacadfd9aec), bare `a` written
/// `aChirho`; these bytes are not the measured GHC input.
const ANY_CLASS_WRITTEN_OFF_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DerivingStrategies #-}
class DescribeChirho aChirho where
  describeChirho :: aChirho -> String
  describeChirho _ = "described"

data PointChirho = PointChirho deriving anyclass (DescribeChirho)

main :: IO ()
main = putStrLn (describeChirho PointChirho)
"#;

#[test]
fn policy_b3_any_class_written_off_chirho() {
    assert_refused_chirho(
        "AnyClassWrittenOffChirho.hs",
        ANY_CLASS_WRITTEN_OFF_CHIRHO,
        &[
            "E0209",
            "`DescribeChirho PointChirho` with the anyclass strategy",
        ],
    );
}

/// B4: alpha-renamed from root's measured source (sha256 201f421efeefce909c858f8716062e64f21d8785b4d343d524cb34caf9f8e8be), bare `a` written
/// `aChirho`; these bytes are not the measured GHC input.
const ANY_CLASS_WRITTEN_ON_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DerivingStrategies, DeriveAnyClass #-}
class DescribeChirho aChirho where
  describeChirho :: aChirho -> String
  describeChirho _ = "described"

data PointChirho = PointChirho deriving anyclass (DescribeChirho)

main :: IO ()
main = putStrLn (describeChirho PointChirho)
"#;

#[test]
fn policy_b4_any_class_written_on_chirho() {
    assert_eq!(
        interpreted_chirho("AnyClassWrittenOnChirho.hs", ANY_CLASS_WRITTEN_ON_CHIRHO),
        "described\n"
    );
}

/// C1: sha256 438cd51c9c7388ecd2f585c81cfc9ec8f2f5983c12839d9e0cae224cae8aa837.
const GND_UNWRITTEN_HASKELL_2_0_1_0_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE Haskell2010 #-}
newtype AgeChirho = AgeChirho Int deriving (Show, Num)

main :: IO ()
main = print (AgeChirho 3 + AgeChirho 4)
"#;

#[test]
fn policy_c1_gnd_unwritten_haskell_2_0_1_0_chirho() {
    assert_refused_chirho(
        "GndUnwrittenHaskell2010Chirho.hs",
        GND_UNWRITTEN_HASKELL_2_0_1_0_CHIRHO,
        &["E0209", "`Num AgeChirho`", "not a stock derivable class"],
    );
}

/// C2: sha256 108ce1fe53f58cb3ea82159d835c9dd37b0db0ec3b809f38dd2a8c0230b3ebef.
const GND_UNWRITTEN_DEFAULT_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
newtype AgeChirho = AgeChirho Int deriving (Show, Num)

main :: IO ()
main = print (AgeChirho 3 + AgeChirho 4)
"#;

#[test]
fn policy_c2_gnd_unwritten_default_chirho() {
    assert_eq!(
        interpreted_chirho("GndUnwrittenDefaultChirho.hs", GND_UNWRITTEN_DEFAULT_CHIRHO),
        "AgeChirho 7\n"
    );
}

/// C3: sha256 6475450929724ced050f94993c2c17c19c39ba97838c496181a51475299692db.
const GND_UNWRITTEN_GHC_2_0_2_4_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GHC2024 #-}
newtype AgeChirho = AgeChirho Int deriving (Show, Num)

main :: IO ()
main = print (AgeChirho 3 + AgeChirho 4)
"#;

#[test]
fn policy_c3_gnd_unwritten_ghc_2_0_2_4_chirho() {
    assert_eq!(
        interpreted_chirho(
            "GndUnwrittenGhc2024Chirho.hs",
            GND_UNWRITTEN_GHC_2_0_2_4_CHIRHO
        ),
        "AgeChirho 7\n"
    );
}

/// C4: sha256 882d1b20414b94d0d62e2dc8d1b4cf23ed854e70b9a03d76feca9de54266fef8.
const GND_UNWRITTEN_EXPLICIT_OFF_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE NoGeneralizedNewtypeDeriving #-}
newtype AgeChirho = AgeChirho Int deriving (Show, Num)

main :: IO ()
main = print (AgeChirho 3 + AgeChirho 4)
"#;

#[test]
fn policy_c4_gnd_unwritten_explicit_off_chirho() {
    assert_refused_chirho(
        "GndUnwrittenExplicitOffChirho.hs",
        GND_UNWRITTEN_EXPLICIT_OFF_CHIRHO,
        &["E0209", "`Num AgeChirho`", "not a stock derivable class"],
    );
}

/// C5: sha256 b9e19d2bf64a22ff8a55396d4c7329c3b22baff7d76b5d03febeb6ef7692a9cc.
const GND_UNWRITTEN_HASKELL_2_0_1_0_ON_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE Haskell2010, GeneralizedNewtypeDeriving #-}
newtype AgeChirho = AgeChirho Int deriving (Show, Num)

main :: IO ()
main = print (AgeChirho 3 + AgeChirho 4)
"#;

#[test]
fn policy_c5_gnd_unwritten_haskell_2_0_1_0_on_chirho() {
    assert_eq!(
        interpreted_chirho(
            "GndUnwrittenHaskell2010OnChirho.hs",
            GND_UNWRITTEN_HASKELL_2_0_1_0_ON_CHIRHO
        ),
        "AgeChirho 7\n"
    );
}

/// D1: alpha-renamed from root's measured source (sha256 927cbca742d3cb32c4df07a69afec6f2e6b6c2de0008b5ff00c6d71b08ebe279), bare `a` written
/// `aChirho`; these bytes are not the measured GHC input.
const NEWTYPE_ANY_CLASS_PRECEDENCE_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DeriveAnyClass #-}
class DescribeChirho aChirho where
  describeChirho :: aChirho -> String
  describeChirho _ = "described"

instance DescribeChirho Int where
  describeChirho _ = "int"

newtype WrapChirho = WrapChirho Int deriving (DescribeChirho)

main :: IO ()
main = putStrLn (describeChirho (WrapChirho 5))
"#;

#[test]
fn policy_d1_newtype_any_class_precedence_chirho() {
    assert_eq!(
        interpreted_chirho(
            "NewtypeAnyClassPrecedenceChirho.hs",
            NEWTYPE_ANY_CLASS_PRECEDENCE_CHIRHO
        ),
        "described\n"
    );
}

/// D2: alpha-renamed from root's measured source (sha256 29e544e7dab5aadbd508acc46d4f88190de23c3f9002f3a2e89f95f3a77a398f), bare `a` written
/// `aChirho`; these bytes are not the measured GHC input.
const NEWTYPE_GND_PRECEDENCE_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
class DescribeChirho aChirho where
  describeChirho :: aChirho -> String
  describeChirho _ = "described"

instance DescribeChirho Int where
  describeChirho _ = "int"

newtype WrapChirho = WrapChirho Int deriving (DescribeChirho)

main :: IO ()
main = putStrLn (describeChirho (WrapChirho 5))
"#;

#[test]
fn policy_d2_newtype_gnd_precedence_chirho() {
    assert_eq!(
        interpreted_chirho(
            "NewtypeGndPrecedenceChirho.hs",
            NEWTYPE_GND_PRECEDENCE_CHIRHO
        ),
        "int\n"
    );
}

/// O1: sha256 e93a0e515be528fcbc110d4ca2a92b6867b1177114a3e1328a04fe714414ae6e.
const EDITION_LATER_HASKELL_2_0_1_0_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GHC2021, Haskell2010, DerivingStrategies #-}
newtype AgeChirho = AgeChirho Int deriving newtype Eq

main :: IO ()
main = return ()
"#;

#[test]
fn policy_o1_edition_later_haskell_2_0_1_0_chirho() {
    assert_refused_chirho(
        "EditionLaterHaskell2010Chirho.hs",
        EDITION_LATER_HASKELL_2_0_1_0_CHIRHO,
        &["E0209", "`Eq AgeChirho` with the newtype strategy"],
    );
}

/// O2: sha256 2603825b5f364eb0eb8421fb677a29b33804476793e1e2bd948a63b3c866ce45.
const EDITION_LATER_GHC_2_0_2_1_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE Haskell2010, GHC2021, DerivingStrategies #-}
newtype AgeChirho = AgeChirho Int deriving newtype Eq

main :: IO ()
main = print (AgeChirho 3 == AgeChirho 3)
"#;

#[test]
fn policy_o2_edition_later_ghc_2_0_2_1_chirho() {
    assert_eq!(
        interpreted_chirho(
            "EditionLaterGhc2021Chirho.hs",
            EDITION_LATER_GHC_2_0_2_1_CHIRHO
        ),
        "True\n"
    );
}

/// O3: sha256 36f556f146a4edce5996aaa409117a5fa3cebadbeb15280edc674049981926da.
const EXPLICIT_OFF_BEFORE_EDITION_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE NoGeneralizedNewtypeDeriving, GHC2021 #-}
newtype AgeChirho = AgeChirho Int deriving (Show, Num)

main :: IO ()
main = print (AgeChirho 3 + AgeChirho 4)
"#;

#[test]
fn policy_o3_explicit_off_before_edition_chirho() {
    assert_refused_chirho(
        "ExplicitOffBeforeEditionChirho.hs",
        EXPLICIT_OFF_BEFORE_EDITION_CHIRHO,
        &["E0209", "`Num AgeChirho`", "not a stock derivable class"],
    );
}

/// O4: sha256 ada1048295c12e210df1e0a62f49a06ad925f40696de3f348b387a479cc80011.
const EXPLICIT_ON_AFTER_OFF_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE NoGeneralizedNewtypeDeriving, GeneralizedNewtypeDeriving #-}
newtype AgeChirho = AgeChirho Int deriving (Show, Num)

main :: IO ()
main = print (AgeChirho 3 + AgeChirho 4)
"#;

#[test]
fn policy_o4_explicit_on_after_off_chirho() {
    assert_eq!(
        interpreted_chirho("ExplicitOnAfterOffChirho.hs", EXPLICIT_ON_AFTER_OFF_CHIRHO),
        "AgeChirho 7\n"
    );
}

/// G1 (gpt #31759; GHC 9.14.1 prints 7): sha256 205ff4ae9886ef3c994cdd2dfed2709cd8348ee85aacd9478be2d3181734e934.
const DIRECT_NEWTYPE_SHOW_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DerivingStrategies, GeneralizedNewtypeDeriving #-}
newtype AgeChirho = AgeChirho Int deriving newtype Show
main :: IO ()
main = putStrLn (show (AgeChirho 7))
"#;

#[test]
fn direct_method_g1_direct_newtype_show_chirho() {
    assert_eq!(
        interpreted_chirho("DirectNewtypeShowChirho.hs", DIRECT_NEWTYPE_SHOW_CHIRHO),
        "7\n"
    );
}

/// G2 (gpt #31759; GHC 9.14.1 prints 7): sha256 06941d7f04432237746234b70331cb51c05736f31eb7c86c5fee7276a897160f.
const POLYMORPHIC_NEWTYPE_SHOW_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DerivingStrategies, GeneralizedNewtypeDeriving #-}
newtype AgeChirho = AgeChirho Int deriving newtype Show
renderChirho :: Show aChirho => aChirho -> IO ()
{-# NOINLINE renderChirho #-}
renderChirho valueChirho = putStrLn (show valueChirho)
main :: IO ()
main = renderChirho (AgeChirho 7)
"#;

#[test]
fn direct_method_g2_polymorphic_newtype_show_chirho() {
    assert_eq!(
        interpreted_chirho(
            "PolymorphicNewtypeShowChirho.hs",
            POLYMORPHIC_NEWTYPE_SHOW_CHIRHO
        ),
        "7\n"
    );
}

/// G3 (gpt #31759; GHC 9.14.1 prints 8): sha256 3495c70d90799a842fb599e6f969e61990f1782c2bdd9f90a041685e8497508a.
const USER_CLASS_NEWTYPE_METHOD_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DerivingStrategies, GeneralizedNewtypeDeriving #-}
class DescribeChirho aChirho where
  describeChirho :: aChirho -> String
  describeChirho _ = "default"
instance DescribeChirho Int where
  describeChirho valueChirho = show (valueChirho + 1)
newtype AgeChirho = AgeChirho Int deriving newtype DescribeChirho
main :: IO ()
main = putStrLn (describeChirho (AgeChirho 7))
"#;

#[test]
fn direct_method_g3_user_class_newtype_method_chirho() {
    assert_eq!(
        interpreted_chirho(
            "UserClassNewtypeMethodChirho.hs",
            USER_CLASS_NEWTYPE_METHOD_CHIRHO
        ),
        "8\n"
    );
}
