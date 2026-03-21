// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

use haskelujah_backend_cranelift_chirho::{
    TargetConfigChirho, compile_core_to_object_executable_chirho,
};
use haskelujah_core_chirho::{elide_dicts_and_filter_chirho, pretty_module_chirho};
use haskelujah_driver_chirho::compile_source_chirho;
use haskelujah_span_chirho::SourceMapChirho;

fn workspace_root_chirho() -> PathBuf {
    let crate_dir_chirho = Path::new(env!("CARGO_MANIFEST_DIR"));
    crate_dir_chirho
        .parent()
        .and_then(Path::parent)
        .expect("driver crate should live under workspace/crates")
        .to_path_buf()
}

fn ensure_rts_staticlib_for_cranelift_tests_chirho() -> PathBuf {
    static RTS_LIB_DIR_CHIRHO: OnceLock<PathBuf> = OnceLock::new();
    RTS_LIB_DIR_CHIRHO
        .get_or_init(|| {
            let workspace_root_chirho = workspace_root_chirho();
            let cargo_status_chirho = Command::new("cargo")
                .current_dir(&workspace_root_chirho)
                .args(["build", "-p", "haskelujah-rts-chirho", "--quiet"])
                .status()
                .expect("should invoke cargo to build RTS");
            assert!(
                cargo_status_chirho.success(),
                "cargo build -p haskelujah-rts-chirho failed with exit code {:?}",
                cargo_status_chirho.code()
            );
            workspace_root_chirho.join("target").join("debug")
        })
        .clone()
}

fn cranelift_round_trip_stdout_chirho(src_chirho: &str) -> String {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let compile_result_chirho =
        compile_source_chirho(src_chirho, &mut source_map_chirho, "Main.hs")
            .expect("source should compile");
    let config_chirho = TargetConfigChirho::default();
    let obj_chirho = compile_core_to_object_executable_chirho(
        &compile_result_chirho.core_chirho,
        &config_chirho,
    )
    .expect("Cranelift object generation should succeed");

    let temp_dir_chirho = tempfile::tempdir().expect("temp dir should be created");
    let object_path_chirho = temp_dir_chirho.path().join("main.o");
    let exe_path_chirho = temp_dir_chirho.path().join("main");
    std::fs::write(&object_path_chirho, &obj_chirho.object_bytes_chirho)
        .expect("object file should be written");

    let rts_lib_dir_chirho = ensure_rts_staticlib_for_cranelift_tests_chirho();
    let link_status_chirho = Command::new("cc")
        .arg("-o")
        .arg(&exe_path_chirho)
        .arg(&object_path_chirho)
        .arg("-Wl,-no_fixup_chains")
        .arg("-L")
        .arg(&rts_lib_dir_chirho)
        .arg("-lhaskelujah_rts_chirho")
        .status()
        .expect("linker should run");
    assert!(
        link_status_chirho.success(),
        "linker failed with exit code {:?}",
        link_status_chirho.code()
    );

    let output_chirho = Command::new(&exe_path_chirho)
        .output()
        .expect("compiled executable should run");
    assert!(
        output_chirho.status.success(),
        "compiled executable failed with exit code {:?}",
        output_chirho.status.code()
    );
    String::from_utf8(output_chirho.stdout).expect("stdout should be UTF-8")
}

fn cranelift_round_trip_stdout_with_input_chirho(src_chirho: &str, stdin_chirho: &str) -> String {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let compile_result_chirho =
        compile_source_chirho(src_chirho, &mut source_map_chirho, "Main.hs")
            .expect("source should compile");
    let config_chirho = TargetConfigChirho::default();
    let obj_chirho = compile_core_to_object_executable_chirho(
        &compile_result_chirho.core_chirho,
        &config_chirho,
    )
    .expect("Cranelift object generation should succeed");

    let temp_dir_chirho = tempfile::tempdir().expect("temp dir should be created");
    let object_path_chirho = temp_dir_chirho.path().join("main.o");
    let exe_path_chirho = temp_dir_chirho.path().join("main");
    std::fs::write(&object_path_chirho, &obj_chirho.object_bytes_chirho)
        .expect("object file should be written");

    let rts_lib_dir_chirho = ensure_rts_staticlib_for_cranelift_tests_chirho();
    let link_status_chirho = Command::new("cc")
        .arg("-o")
        .arg(&exe_path_chirho)
        .arg(&object_path_chirho)
        .arg("-Wl,-no_fixup_chains")
        .arg("-L")
        .arg(&rts_lib_dir_chirho)
        .arg("-lhaskelujah_rts_chirho")
        .status()
        .expect("linker should run");
    assert!(
        link_status_chirho.success(),
        "linker failed with exit code {:?}",
        link_status_chirho.code()
    );

    let mut child_chirho = Command::new(&exe_path_chirho)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("compiled executable should run");
    use std::io::Write;
    child_chirho
        .stdin
        .as_mut()
        .expect("stdin pipe should exist")
        .write_all(stdin_chirho.as_bytes())
        .expect("stdin should be writable");
    let output_chirho = child_chirho
        .wait_with_output()
        .expect("compiled executable should finish");
    assert!(
        output_chirho.status.success(),
        "compiled executable failed with exit code {:?}",
        output_chirho.status.code()
    );
    String::from_utf8(output_chirho.stdout).expect("stdout should be UTF-8")
}

fn haskell_string_literal_chirho(text_chirho: &str) -> String {
    let mut out_chirho = String::with_capacity(text_chirho.len() + 2);
    out_chirho.push('"');
    for ch_chirho in text_chirho.chars() {
        match ch_chirho {
            '\\' => out_chirho.push_str("\\\\"),
            '"' => out_chirho.push_str("\\\""),
            '\n' => out_chirho.push_str("\\n"),
            '\r' => out_chirho.push_str("\\r"),
            '\t' => out_chirho.push_str("\\t"),
            _ => out_chirho.push(ch_chirho),
        }
    }
    out_chirho.push('"');
    out_chirho
}

#[test]
fn cranelift_round_trip_sum_range_output_chirho() {
    let stdout_chirho =
        cranelift_round_trip_stdout_chirho("module Main where\nmain = print (sum [1..10])\n");
    assert_eq!(stdout_chirho, "55\n");
}

#[test]
fn cranelift_round_trip_user_range_large_output_chirho() {
    let stdout_chirho = cranelift_round_trip_stdout_chirho(
        "module Main where\nrange lo hi = if lo > hi then [] else lo : range (lo + 1) hi\nmain = print (sum (range 1 10000))\n",
    );
    assert_eq!(stdout_chirho, "50005000\n");
}

#[test]
fn cranelift_user_range_filtered_core_has_no_num_selectors_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let compile_result_chirho = compile_source_chirho(
        "module Main where\nrange lo hi = if lo > hi then [] else lo : range (lo + 1) hi\nmain = print (sum (range 1 10))\n",
        &mut source_map_chirho,
        "Main.hs",
    )
    .expect("source should compile");
    let filtered_core_chirho = elide_dicts_and_filter_chirho(&compile_result_chirho.core_chirho);
    let pretty_core_chirho = pretty_module_chirho(&filtered_core_chirho);
    assert!(
        !pretty_core_chirho.contains("$sel_Num_+"),
        "filtered core should not retain $sel_Num_+:\n{pretty_core_chirho}"
    );
    assert!(
        !pretty_core_chirho.contains("$sel_Num_fromInteger"),
        "filtered core should not retain $sel_Num_fromInteger:\n{pretty_core_chirho}"
    );
}

#[test]
fn cranelift_applyop_divide_filtered_core_has_no_integral_selectors_chirho() {
    let mut source_map_chirho = SourceMapChirho::new_chirho();
    let compile_result_chirho = compile_source_chirho(
        "module Main where\ndata Op = Plus | Minus | Times | Divide deriving (Eq, Show)\napplyOp Plus x y = x + y\napplyOp Minus x y = x - y\napplyOp Times x y = x * y\napplyOp Divide x y = div x y\nmain = print (applyOp Plus 3 4)\n",
        &mut source_map_chirho,
        "Main.hs",
    )
    .expect("source should compile");
    let filtered_core_chirho = elide_dicts_and_filter_chirho(&compile_result_chirho.core_chirho);
    let pretty_core_chirho = pretty_module_chirho(&filtered_core_chirho);
    assert!(
        !pretty_core_chirho.contains("$sel_Integral_super_Num"),
        "filtered core should not retain $sel_Integral_super_Num:\\n{pretty_core_chirho}"
    );
    assert!(
        !pretty_core_chirho.contains("$sel_Integral_div"),
        "filtered core should not retain $sel_Integral_div:\\n{pretty_core_chirho}"
    );
    assert!(
        pretty_core_chirho.contains("div#"),
        "filtered core should lower div to div#:\\n{pretty_core_chirho}"
    );
}

#[test]
fn cranelift_round_trip_applyop_divide_output_chirho() {
    let stdout_chirho = cranelift_round_trip_stdout_chirho(
        "module Main where\ndata Op = Plus | Minus | Times | Divide deriving (Eq, Show)\napplyOp Plus x y = x + y\napplyOp Minus x y = x - y\napplyOp Times x y = x * y\napplyOp Divide x y = div x y\nmain = print (applyOp Plus 3 4)\n",
    );
    assert_eq!(stdout_chirho, "7\n");
}

#[test]
fn cranelift_round_trip_show_bool_output_chirho() {
    let stdout_chirho =
        cranelift_round_trip_stdout_chirho("module Main where\nmain = putStrLn (show True)\n");
    assert_eq!(stdout_chirho, "True\n");
}

#[test]
fn cranelift_round_trip_show_negative_int_output_chirho() {
    let stdout_chirho =
        cranelift_round_trip_stdout_chirho("module Main where\nmain = putStrLn (show (-42))\n");
    assert_eq!(stdout_chirho, "-42\n");
}

#[test]
fn cranelift_round_trip_getline_read_int_output_chirho() {
    let stdout_chirho = cranelift_round_trip_stdout_with_input_chirho(
        "module Main where\nmain = do\n  line <- getLine\n  print (((read line) :: Int) + 1)\n",
        "41\n",
    );
    assert_eq!(stdout_chirho, "42\n");
}

#[test]
fn cranelift_round_trip_recursive_io_return_unit_output_chirho() {
    let stdout_chirho = cranelift_round_trip_stdout_chirho(
        "module Main where\nprintTodos [] = return ()\nprintTodos (x:xs) = do\n  putStrLn x\n  printTodos xs\nmain = printTodos [\"a\",\"b\"]\n",
    );
    assert_eq!(stdout_chirho, "a\nb\n");
}

#[test]
fn cranelift_round_trip_put_str_output_chirho() {
    let stdout_chirho = cranelift_round_trip_stdout_chirho(
        "module Main where\nmain = do\n  putStr \"Hello\"\n  putStr \" from\"\n  putStr \" Haskelujah!\"\n",
    );
    assert_eq!(stdout_chirho, "Hello from Haskelujah!");
}

#[test]
fn cranelift_round_trip_write_file_output_chirho() {
    let temp_dir_chirho = tempfile::tempdir().expect("temp dir should exist");
    let file_path_chirho = temp_dir_chirho.path().join("write-file-chirho.txt");
    let file_path_literal_chirho =
        haskell_string_literal_chirho(&file_path_chirho.display().to_string());
    let src_chirho = format!(
        "module Main where\nmain = do\n  writeFile {file_path_literal_chirho} \"hello from cranelift\"\n  putStr \"ok\"\n"
    );
    let stdout_chirho = cranelift_round_trip_stdout_chirho(&src_chirho);
    assert_eq!(stdout_chirho, "ok");
    let file_contents_chirho =
        std::fs::read_to_string(&file_path_chirho).expect("writeFile should create file");
    assert_eq!(file_contents_chirho, "hello from cranelift");
}

#[test]
fn cranelift_round_trip_read_file_output_chirho() {
    let temp_dir_chirho = tempfile::tempdir().expect("temp dir should exist");
    let file_path_chirho = temp_dir_chirho.path().join("read-file-chirho.txt");
    std::fs::write(&file_path_chirho, "hello from disk").expect("fixture file should be written");
    let file_path_literal_chirho =
        haskell_string_literal_chirho(&file_path_chirho.display().to_string());
    let src_chirho = format!(
        "module Main where\nmain = do\n  contentsChirho <- readFile {file_path_literal_chirho}\n  putStr contentsChirho\n"
    );
    let stdout_chirho = cranelift_round_trip_stdout_chirho(&src_chirho);
    assert_eq!(stdout_chirho, "hello from disk");
}

#[test]
fn cranelift_round_trip_write_then_read_file_output_chirho() {
    let temp_dir_chirho = tempfile::tempdir().expect("temp dir should exist");
    let file_path_chirho = temp_dir_chirho.path().join("write-read-file-chirho.txt");
    let file_path_literal_chirho =
        haskell_string_literal_chirho(&file_path_chirho.display().to_string());
    let src_chirho = format!(
        "module Main where\nmain = do\n  writeFile {file_path_literal_chirho} \"hello from combined cranelift\"\n  contentsChirho <- readFile {file_path_literal_chirho}\n  putStr contentsChirho\n"
    );
    let stdout_chirho = cranelift_round_trip_stdout_chirho(&src_chirho);
    assert_eq!(stdout_chirho, "hello from combined cranelift");
    let file_contents_chirho =
        std::fs::read_to_string(&file_path_chirho).expect("combined file IO should create file");
    assert_eq!(file_contents_chirho, "hello from combined cranelift");
}

#[test]
fn cranelift_round_trip_show_char_output_chirho() {
    let stdout_chirho =
        cranelift_round_trip_stdout_chirho("module Main where\nmain = putStrLn (show 'A')\n");
    assert_eq!(stdout_chirho, "'A'\n");
}

#[test]
fn cranelift_round_trip_show_float_output_chirho() {
    let stdout_chirho =
        cranelift_round_trip_stdout_chirho("module Main where\nmain = putStrLn (show 3.14)\n");
    assert_eq!(stdout_chirho, "3.14\n");
}

#[test]
fn cranelift_round_trip_show_derived_enum_output_chirho() {
    let stdout_chirho = cranelift_round_trip_stdout_chirho(
        "module Main where\ndata Color = Red | Green | Blue deriving (Show)\nmain = putStrLn (show Green)\n",
    );
    assert_eq!(stdout_chirho, "Green\n");
}

#[test]
fn cranelift_round_trip_print_derived_enum_output_chirho() {
    let stdout_chirho = cranelift_round_trip_stdout_chirho(
        "module Main where\ndata Color = Red | Green | Blue deriving (Show)\nmain = print Green\n",
    );
    assert_eq!(stdout_chirho, "Green\n");
}

#[test]
fn cranelift_round_trip_print_derived_field_constructor_output_chirho() {
    let stdout_chirho = cranelift_round_trip_stdout_chirho(
        "module Main where\ndata Pair = MkPair Int Int deriving (Show)\nmain = print (MkPair 3 4)\n",
    );
    assert_eq!(stdout_chirho, "MkPair 3 4\n");
}

#[test]
fn cranelift_round_trip_show_derived_field_constructor_output_chirho() {
    let stdout_chirho = cranelift_round_trip_stdout_chirho(
        "module Main where\ndata Pair = MkPair Int Int deriving (Show)\nmain = putStrLn (show (MkPair 3 4))\n",
    );
    assert_eq!(stdout_chirho, "MkPair 3 4\n");
}

#[test]
fn cranelift_round_trip_nested_adt_list_pattern_output_chirho() {
    let stdout_chirho = cranelift_round_trip_stdout_chirho(
        "module Main where\ndata Value = VInt Int | VList [Value]\nscore (VList [VInt n]) = n\nscore (VList (_:_)) = 1\nscore _ = 2\nmain = print (score (VList [VInt 7]))\n",
    );
    assert_eq!(stdout_chirho, "7\n");
}

#[test]
fn cranelift_round_trip_non_tail_recursive_list_fold_output_chirho() {
    let stdout_chirho = cranelift_round_trip_stdout_chirho(
        "module Main where\nmyMax [x] = x\nmyMax (x:xs) = if x > myMax xs then x else myMax xs\nmain = print (myMax [5,3,9,2])\n",
    );
    assert_eq!(stdout_chirho, "9\n");
}

#[test]
fn cranelift_round_trip_inner_column_list_binder_output_chirho() {
    let stdout_chirho = cranelift_round_trip_stdout_chirho(
        "module Main where\nf (x:_) 0 = x\nf (_:xs) n = f xs (n-1)\nmain = print (f [5,6,7] 1)\n",
    );
    assert_eq!(stdout_chirho, "6\n");
}

#[test]
fn cranelift_round_trip_print_int_list_output_chirho() {
    let stdout_chirho =
        cranelift_round_trip_stdout_chirho("module Main where\nmain = print [1,2,3]\n");
    assert_eq!(stdout_chirho, "[1,2,3]\n");
}

#[test]
fn cranelift_round_trip_print_bool_list_output_chirho() {
    let stdout_chirho =
        cranelift_round_trip_stdout_chirho("module Main where\nmain = print [True,False,True]\n");
    assert_eq!(stdout_chirho, "[True,False,True]\n");
}

#[test]
fn cranelift_round_trip_print_tuple_list_with_adt_field_output_chirho() {
    let stdout_chirho = cranelift_round_trip_stdout_chirho(
        "module Main where\ndata Op = Add | Mul deriving (Eq, Show)\nsteps = [(Add,2),(Mul,3)]\nmain = print steps\n",
    );
    assert_eq!(stdout_chirho, "[(Add,2),(Mul,3)]\n");
}

#[test]
fn cranelift_round_trip_show_tuple_list_with_adt_field_output_chirho() {
    let stdout_chirho = cranelift_round_trip_stdout_chirho(
        "module Main where\ndata Op = Add | Mul deriving (Eq, Show)\nsteps = [(Add,2),(Mul,3)]\nmain = putStrLn (show steps)\n",
    );
    assert_eq!(stdout_chirho, "[(Add,2),(Mul,3)]\n");
}

#[test]
fn cranelift_round_trip_otherwise_guard_output_chirho() {
    let stdout_chirho = cranelift_round_trip_stdout_chirho(
        "module Main where\nclassify n\n  | n < 0 = \"neg\"\n  | n == 0 = \"zero\"\n  | otherwise = \"pos\"\nmain = do\n  putStrLn (classify (-1))\n  putStrLn (classify 0)\n  putStrLn (classify 1)\n",
    );
    assert_eq!(stdout_chirho, "neg\nzero\npos\n");
}

#[test]
fn cranelift_round_trip_guard_where_binding_output_chirho() {
    let stdout_chirho = cranelift_round_trip_stdout_chirho(
        "module Main where\nf x\n  | a > 0 = a\n  | otherwise = 0\n  where\n    a = x - 5\nmain = do\n  print (f 7)\n  print (f 4)\n",
    );
    assert_eq!(stdout_chirho, "2\n0\n");
}

#[test]
fn cranelift_round_trip_case_inside_do_output_chirho() {
    let stdout_chirho = cranelift_round_trip_stdout_chirho(
        "module Main where\nmain = do\n  putStrLn \"start\"\n  case Just 3 of\n    Just n -> print (n + 1)\n    Nothing -> print 0\n  putStrLn \"done\"\n",
    );
    assert_eq!(stdout_chirho, "start\n4\ndone\n");
}

#[test]
fn cranelift_round_trip_nested_singleton_pattern_falls_through_chirho() {
    let stdout_chirho = cranelift_round_trip_stdout_chirho(
        "module Main where\nisSingle (x:[]) = 1\nisSingle _ = 2\nmain = print (isSingle [1,2])\n",
    );
    assert_eq!(stdout_chirho, "2\n");
}

#[test]
fn cranelift_round_trip_nested_singleton_pattern_var_fallback_chirho() {
    let stdout_chirho = cranelift_round_trip_stdout_chirho(
        "module Main where\nsingletonOrLength (x:[]) = 1\nsingletonOrLength ys = length ys\nmain = print (singletonOrLength [1,2])\n",
    );
    assert_eq!(stdout_chirho, "2\n");
}

#[test]
fn cranelift_round_trip_merge_sort_preserves_all_elements_chirho() {
    let stdout_chirho = cranelift_round_trip_stdout_chirho(
        "module Main where\nmerge [] ys = ys\nmerge xs [] = xs\nmerge (x:xs) (y:ys) = if x <= y then x : merge xs (y:ys) else y : merge (x:xs) ys\nmsort [] = []\nmsort (x:[]) = [x]\nmsort xs = merge (msort (take (div (length xs) 2) xs)) (msort (drop (div (length xs) 2) xs))\nprintList [] = putStrLn \"\"\nprintList (x:xs) = do print x; printList xs\nmain = printList (msort [5,3,8,1,9,2,7,4,6])\n",
    );
    assert_eq!(stdout_chirho, "1\n2\n3\n4\n5\n6\n7\n8\n9\n\n");
}

#[test]
fn cranelift_round_trip_middle_equation_list_fallback_chirho() {
    let stdout_chirho = cranelift_round_trip_stdout_chirho(
        "module Main where\ng [] ys = ys\ng xs [] = xs\ng (x:xs) (y:ys) = x : xs\nprintList [] = putStrLn \"\"\nprintList (x:xs) = do print x; printList xs\nmain = printList (g [1,2] [])\n",
    );
    assert_eq!(stdout_chirho, "1\n2\n\n");
}

#[test]
fn cranelift_round_trip_deep_tail_recursion_no_stack_overflow_chirho() {
    let stdout_chirho = cranelift_round_trip_stdout_chirho(
        "module Main where\nsumTo limit = go 0 0 where go acc n = if n >= limit then acc else go (acc + n) (n + 1)\nmain = print (sumTo 200000)\n",
    );
    assert_eq!(stdout_chirho, "19999900000\n");
}

#[test]
fn cranelift_round_trip_euler1_tail_recursion_no_stack_overflow_chirho() {
    let stdout_chirho = cranelift_round_trip_stdout_chirho(
        "module Main where\neuler1 limit = go 0 0 where\n  go acc n = if n >= limit then acc else if mod n 3 == 0 then go (acc + n) (n + 1) else if mod n 5 == 0 then go (acc + n) (n + 1) else go acc (n + 1)\nmain = print (euler1 100000)\n",
    );
    assert_eq!(stdout_chirho, "2333316668\n");
}

#[test]
fn cranelift_round_trip_ackermann_multi_equation_pattern_match_chirho() {
    // Regression test for multi-equation pattern match ordering.
    // ack 0 n = n+1 must take priority over ack m 0 when m=0, n=0.
    let stdout_chirho = cranelift_round_trip_stdout_chirho(
        "module Main where\nack :: Int -> Int -> Int\nack 0 n = n + 1\nack m 0 = ack (m - 1) 1\nack m n = ack (m - 1) (ack m (n - 1))\nmain = print (ack 3 7)\n",
    );
    assert_eq!(stdout_chirho, "1021\n");
}

#[test]
fn cranelift_round_trip_polymorphic_map_multi_use_output_chirho() {
    let stdout_chirho = cranelift_round_trip_stdout_chirho(
        "module Main where\nmyMap :: (a -> b) -> [a] -> [b]\nmyMap _ [] = []\nmyMap f (x:xs) = f x : myMap f xs\nsumList :: [Int] -> Int\nsumList [] = 0\nsumList (x:xs) = x + sumList xs\ncountTrue :: [Bool] -> Int\ncountTrue [] = 0\ncountTrue (x:xs) = if x then 1 + countTrue xs else countTrue xs\nisOddChirho :: Int -> Bool\nisOddChirho n = mod n 2 == 1\nmain = do\n  print (sumList (myMap (\\x -> x + 1) [1,2,3]))\n  print (countTrue (myMap isOddChirho [1,2,3,4,5]))\n",
    );
    assert_eq!(stdout_chirho, "9\n3\n");
}

#[test]
fn cranelift_round_trip_zip_tuple_list_output_chirho() {
    let stdout_chirho = cranelift_round_trip_stdout_chirho(
        "module Main where\nmyZip :: [a] -> [b] -> [(a, b)]\nmyZip [] _ = []\nmyZip _ [] = []\nmyZip (x:xs) (y:ys) = (x, y) : myZip xs ys\nsumPairs :: [(Int, Int)] -> Int\nsumPairs [] = 0\nsumPairs ((x, y):xys) = x + y + sumPairs xys\nmain = print (sumPairs (myZip [1,2,3,4,5] [10,11,12,13,14]))\n",
    );
    assert_eq!(stdout_chirho, "75\n");
}
