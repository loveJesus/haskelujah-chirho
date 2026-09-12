// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Open defaults do not erase source-specified polymorphism or closed inference.
//! Independent sources: kind-oracles-chirho/ascriptions-chirho/open-defaults-chirho.

use super::{assert_compile_success_chirho, assert_kind_error_chirho};

#[test]
fn open_result_defaults_chirho() {
    assert_kind_error_chirho(
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\n{-# LANGUAGE PolyKinds, TypeFamilies, DataKinds, KindSignatures, StandaloneKindSignatures, ExplicitForAll #-}\nmodule OpenDefaultsChirho where\nimport Data.Kind (Type)\nimport Data.Proxy (Proxy)\ntype family DefaultChirho aChirho\ntype instance DefaultChirho Int = 'True\n",
    );
}

#[test]
fn open_input_defaults_chirho() {
    assert_kind_error_chirho(
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\n{-# LANGUAGE PolyKinds, TypeFamilies, DataKinds, KindSignatures, StandaloneKindSignatures, ExplicitForAll #-}\nmodule OpenDefaultsChirho where\nimport Data.Kind (Type)\nimport Data.Proxy (Proxy)\ntype family DefaultChirho aChirho :: Type\ntype instance DefaultChirho 'True = Int\n",
    );
}

#[test]
fn open_named_kind_chirho() {
    assert_compile_success_chirho(
        "OpenNamedKindChirho.hs",
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\n{-# LANGUAGE PolyKinds, TypeFamilies, DataKinds, KindSignatures, StandaloneKindSignatures, ExplicitForAll #-}\nmodule OpenDefaultsChirho where\nimport Data.Kind (Type)\nimport Data.Proxy (Proxy)\ntype family MarkChirho (aChirho :: kChirho) :: Type\ntype instance MarkChirho (aChirho :: Bool) = Ordering\ntype UseChirho = Proxy ('LT :: MarkChirho 'True)\n",
    );
}

#[test]
fn open_complete_kind_chirho() {
    assert_compile_success_chirho(
        "OpenCompleteKindChirho.hs",
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\n{-# LANGUAGE PolyKinds, TypeFamilies, DataKinds, KindSignatures, StandaloneKindSignatures, ExplicitForAll #-}\nmodule OpenDefaultsChirho where\nimport Data.Kind (Type)\nimport Data.Proxy (Proxy)\ntype MarkChirho :: forall kChirho. kChirho -> Type\ntype family MarkChirho aChirho\ntype instance MarkChirho (aChirho :: Bool) = Ordering\ntype UseChirho = Proxy ('LT :: MarkChirho 'True)\n",
    );
}

#[test]
fn closed_inferred_kind_chirho() {
    assert_compile_success_chirho(
        "ClosedInferredKindChirho.hs",
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\n{-# LANGUAGE PolyKinds, TypeFamilies, DataKinds, KindSignatures, StandaloneKindSignatures, ExplicitForAll #-}\nmodule OpenDefaultsChirho where\nimport Data.Kind (Type)\nimport Data.Proxy (Proxy)\ntype family FlipChirho aChirho where\n  FlipChirho 'True = 'False\n  FlipChirho 'False = 'True\nwitnessChirho :: Proxy (FlipChirho 'True) -> Proxy 'False\nwitnessChirho xChirho = xChirho\n",
    );
}

#[test]
fn open_mixed_kinds_chirho() {
    assert_compile_success_chirho(
        "OpenMixedKindsChirho.hs",
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\n{-# LANGUAGE PolyKinds, TypeFamilies, DataKinds, KindSignatures, StandaloneKindSignatures, ExplicitForAll #-}\nmodule OpenDefaultsChirho where\nimport Data.Kind (Type)\nimport Data.Proxy (Proxy)\ntype family MixedChirho (aChirho :: kChirho) bChirho :: Type\ntype instance MixedChirho (aChirho :: Bool) Int = Ordering\ntype UseChirho = Proxy ('LT :: MixedChirho 'True Int)\n",
    );
}

#[test]
fn open_polymorphic_tail_chirho() {
    assert_compile_success_chirho(
        "OpenPolymorphicTailChirho.hs",
        "-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)\n{-# LANGUAGE PolyKinds, TypeFamilies, DataKinds, KindSignatures, StandaloneKindSignatures, ExplicitForAll #-}\nmodule OpenDefaultsChirho where\nimport Data.Kind (Type)\nimport Data.Proxy (Proxy)\ntype family TailChirho aChirho :: kChirho -> Type\ntype UseChirho = TailChirho Int 'True\n",
    );
}

#[test]
fn open_kind_inferred_chirho() {
    assert_compile_success_chirho(
        "OpenKindInferredChirho.hs",
        "{-# LANGUAGE PolyKinds #-}\n{-# LANGUAGE TypeFamilies #-}\n{-# LANGUAGE GADTs #-}\n{-# LANGUAGE DataKinds #-}\n\nmodule T11348 where\n\nimport Data.Kind\nimport Data.Proxy\n\ntype family TrivialFamily t :: Type\ntype instance TrivialFamily (t :: Type) = Bool\n\ndata R where\n    R :: Proxy Bool -> R\n\ntype ProblemType t = 'R ('Proxy :: Proxy (TrivialFamily t))\n",
    );
}
