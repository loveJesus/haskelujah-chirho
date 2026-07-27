// For God so loved the world, that he gave his only begotten Son, that whosoever believeth
// in him should not perish, but have everlasting life. — John 3:16 (KJV)

use crate::eval_source_with_machine_chirho;
use haskelujah_span_chirho::SourceMapChirho;

fn eval_output_chirho(source_chirho: &str, file_name_chirho: &str) -> String {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let (_value_chirho, machine_chirho) = eval_source_with_machine_chirho(
        source_chirho,
        &mut source_map_chirho,
        file_name_chirho,
        None,
    )
    .expect("captured letrec program should evaluate");
    machine_chirho.io_output_chirho
}

#[test]
fn eval_captured_letrec_is_fresh_per_invocation_chirho() {
    let source_chirho = r#"module CapturedLetrecFreshChirho where
streamChirho pChirho = goChirho
  where goChirho = pChirho : goChirho
main =
  let { firstChirho = streamChirho 1; secondChirho = streamChirho 10 }
  in head firstChirho `seq`
     (head secondChirho `seq` print (head (tail firstChirho)))
"#;

    assert_eq!(
        eval_output_chirho(source_chirho, "CapturedLetrecFreshChirho.hs"),
        "1\n",
        "a later invocation must not retarget an earlier recursive closure"
    );
}

#[test]
fn eval_sieve_local_recursive_worker_chirho() {
    let source_chirho = r#"module SieveLocalWorkerChirho where
sieveChirho [] = []
sieveChirho (pChirho:xsChirho) = pChirho : sieveChirho (goChirho xsChirho)
  where
    goChirho [] = []
    goChirho (yChirho:ysChirho) =
      if yChirho `mod` pChirho /= 0
        then yChirho : goChirho ysChirho
        else goChirho ysChirho
main = print (take 6 (sieveChirho [2..]))
"#;

    assert_eq!(
        eval_output_chirho(source_chirho, "SieveLocalWorkerChirho.hs"),
        "[2,3,5,7,11,13]\n",
        "a captured recursive worker must keep each invocation's divisor"
    );
}

#[test]
fn eval_captured_mutual_letrec_group_is_fresh_chirho() {
    let source_chirho = r#"module CapturedMutualLetrecChirho where
streamChirho pChirho = leftChirho
  where
    leftChirho = pChirho : rightChirho
    rightChirho = (pChirho + 1) : leftChirho
main =
  let { firstChirho = streamChirho 1; secondChirho = streamChirho 10 }
  in head firstChirho `seq`
     (head secondChirho `seq` print (take 4 firstChirho))
"#;

    assert_eq!(
        eval_output_chirho(source_chirho, "CapturedMutualLetrecChirho.hs"),
        "[1,2,1,2]\n",
        "mutually recursive closures must point at their invocation's fresh group"
    );
}
