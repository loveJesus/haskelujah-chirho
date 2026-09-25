#!/bin/bash
# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
# The one ordering for everything this runner writes and hashes.
#
# Why this exists: the committed measurement artifacts hash a failure list, and that
# hash is only meaningful if the list's ORDER is fixed. A bare `sort` follows the
# caller's locale, and a case-folding locale puts `TcSpecPragmas.hs` after `tc134.hs`
# while C puts it before. The identical 53 accepted files hash 879cacad... under C and
# d90fc86c... under en_US.UTF-8, so a measurer in another locale would compute a
# different list hash from an IDENTICAL result and read it as movement. Both
# claude_chirho and claude2_chirho hit exactly that before establishing set equality.
#
# So: every sort here is C-collated, and the artifacts say so. Names are compared byte
# by byte, and each line is newline-terminated.
# usage (sourced): ... | sort_rows_chirho     # verdict rows, by the filename field
#                  ... | sort_names_chirho    # bare filenames

# Verdict rows ("VERDICT name rc [hash]"), ordered by the name field.
sort_rows_chirho() {
  LC_ALL=C sort -k2
}

# Bare filenames, one per line, as the artifacts' list hashes take them.
sort_names_chirho() {
  LC_ALL=C sort
}
