// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! GHC 9.14.1's `lines` boundary: `\n` terminates a line but does not create
//! one additional empty line at EOF. Carriage return remains ordinary content.
//! Workflow: compiler-pipeline-chirho/runtime-primitive-evaluation-chirho.md.

use haskelujah_span_chirho::SourceMapChirho;

fn run_output_chirho(source_chirho: &str) -> String {
    haskelujah_driver::eval_source_with_machine_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "LinesTrailingNewlineChirho.hs",
        None,
    )
    .expect("GHC runs the lines boundary program")
    .1
    .io_output_chirho
}

#[test]
fn lines_matches_ghc_at_empty_and_newline_boundaries_chirho() {
    let source_chirho = "module Main where\nmain :: IO ()\nmain = do\n  print (map length (lines \"\"))\n  print (map length (lines \"a\"))\n  print (map length (lines \"a\\n\"))\n  print (map length (lines \"a\\nb\\n\"))\n  print (map length (lines \"a\\nb\\n\\n\"))\n  print (map length (lines \"\\n\"))\n  print (map length (lines \"\\n\\n\"))\n  print (map length (lines \"\\na\\n\\nb\"))\n  print (map length (lines \"a\\r\\nb\\r\\n\"))\n";
    assert_eq!(
        run_output_chirho(source_chirho),
        "[]\n[1]\n[1]\n[1,1]\n[1,1,0]\n[0]\n[0,0]\n[0,1,0,1]\n[2,2]\n"
    );
}

#[test]
fn lines_after_unlines_round_trips_the_measured_lists_chirho() {
    let source_chirho = "module Main where\nmain :: IO ()\nmain = do\n  print (map length (lines (unlines [])))\n  print (map length (lines (unlines [\"a\", \"b\"])))\n  print (map length (lines (unlines [\"a\", \"\"])))\n  print (map length (lines (unlines [\"\"])))\n";
    assert_eq!(run_output_chirho(source_chirho), "[]\n[1,1]\n[1,0]\n[0]\n");
}
