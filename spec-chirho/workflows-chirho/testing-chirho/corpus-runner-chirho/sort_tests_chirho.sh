#!/bin/bash
# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
# Controls for the runner's ordering. The measurement artifacts hash a list of
# filenames, so the ordering must not depend on who runs the measurement.
# The permutation below is the real one: `TcSpecPragmas.hs` and `tc134.hs` are the
# two names whose relative order a case-folding locale changes, and they are the
# names that made an identical 53-file result hash differently for two agents.
# usage: sort_tests_chirho.sh
here_chirho="$(cd "$(dirname "$0")" && pwd)"
. "$here_chirho/sort_chirho.sh"
pass_chirho=0; fail_chirho=0
check_chirho() {
  local name_chirho="$1" want_chirho="$2" got_chirho="$3"
  if [ "$got_chirho" = "$want_chirho" ]; then pass_chirho=$((pass_chirho+1)); printf 'ok    %s\n' "$name_chirho"
  else fail_chirho=$((fail_chirho+1)); printf 'FAIL  %s\n  want %s\n  got  %s\n' "$name_chirho" "$want_chirho" "$got_chirho"; fi
}

names_chirho="tc134.hs
TcSpecPragmas.hs
UnliftedNewtypesGnd.hs
tc239.hs"
rows_chirho="REJECT tc134.hs 1
REJECT TcSpecPragmas.hs 1
REJECT UnliftedNewtypesGnd.hs 1
REJECT tc239.hs 1"

# C collation puts every uppercase initial before every lowercase one.
want_names_chirho="TcSpecPragmas.hs
UnliftedNewtypesGnd.hs
tc134.hs
tc239.hs"
want_rows_chirho="REJECT TcSpecPragmas.hs 1
REJECT UnliftedNewtypesGnd.hs 1
REJECT tc134.hs 1
REJECT tc239.hs 1"

# The same input under every locale a measurer might be running.
for locale_chirho in C en_US.UTF-8 en_GB.UTF-8 ''; do
  if [ -n "$locale_chirho" ]; then
    got_names_chirho="$(LC_ALL="$locale_chirho" LANG="$locale_chirho" bash -c ". '$here_chirho/sort_chirho.sh'; sort_names_chirho" <<< "$names_chirho")"
    got_rows_chirho="$(LC_ALL="$locale_chirho" LANG="$locale_chirho" bash -c ". '$here_chirho/sort_chirho.sh'; sort_rows_chirho" <<< "$rows_chirho")"
    label_chirho="$locale_chirho"
  else
    got_names_chirho="$(unset LC_ALL; LANG=en_US.UTF-8 bash -c ". '$here_chirho/sort_chirho.sh'; sort_names_chirho" <<< "$names_chirho")"
    got_rows_chirho="$(unset LC_ALL; LANG=en_US.UTF-8 bash -c ". '$here_chirho/sort_chirho.sh'; sort_rows_chirho" <<< "$rows_chirho")"
    label_chirho="LC_ALL unset, LANG=en_US.UTF-8"
  fi
  check_chirho "names under $label_chirho" "$want_names_chirho" "$got_names_chirho"
  check_chirho "rows under $label_chirho" "$want_rows_chirho" "$got_rows_chirho"
done

# A bare `sort` under a case-folding locale really does disagree, which is the
# defect these controls exist for. If this ever stops being true the controls
# above have stopped testing anything.
bare_chirho="$(LC_ALL=en_US.UTF-8 LANG=en_US.UTF-8 sort <<< "$names_chirho")"
if [ "$bare_chirho" = "$want_names_chirho" ]; then
  fail_chirho=$((fail_chirho+1)); printf 'FAIL  a bare sort no longer differs, so these controls prove nothing\n'
else
  pass_chirho=$((pass_chirho+1)); printf 'ok    a bare sort differs under a case-folding locale\n'
fi

echo "$pass_chirho/$((pass_chirho+fail_chirho)) ordering controls pass"
[ $fail_chirho -eq 0 ]
