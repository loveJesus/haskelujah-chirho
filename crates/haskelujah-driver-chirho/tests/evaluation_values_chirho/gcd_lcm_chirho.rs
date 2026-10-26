// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! GHC 9.14.1's `gcd` and `lcm` results through their real Integral schemes.
//! Workflow: compiler-pipeline-chirho/builtin-type-environment-chirho.md.

use haskelujah_span_chirho::SourceMapChirho;

#[test]
fn gcd_and_lcm_results_keep_int_evidence_in_consuming_tuples_chirho() {
    let source_chirho = "module Main where\nmain :: IO ()\nmain = do\n  print (gcd 12 (18 :: Int))\n  print (gcd (-12) (18 :: Int))\n  print (gcd 12 (-18 :: Int))\n  print (gcd 0 (18 :: Int))\n  print (gcd 17 (19 :: Int))\n  print (gcd 21 (21 :: Int))\n  print (lcm 4 (6 :: Int))\n  print (lcm (-4) (6 :: Int))\n  print (lcm 0 (6 :: Int))\n  print (map (gcd 12) [18, 8, 5 :: Int])\n  print (gcd 12 (18 :: Int), lcm 4 (6 :: Int))\n";

    let (_, machine_chirho) = haskelujah_driver::eval_source_with_machine_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "GcdLcmEvidenceChirho.hs",
        None,
    )
    .expect("the measured gcd/lcm program evaluates");

    assert_eq!(
        machine_chirho.io_output_chirho,
        "6\n6\n6\n18\n1\n21\n12\n12\n0\n[6,4,1]\n(6,12)\n"
    );
}
