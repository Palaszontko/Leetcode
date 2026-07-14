# LeetCode solution runner (leetgo per-problem layout)
#
# Usage:
#   just run rust 0001     # run problem 0001 in rust against all its testcases
#   just run go 1          # ids are zero-padded automatically (1 -> 0001)
#   just run python 0001
#   just debug rust 1      # same, but also print the full program output
#                          # (your println!/print debug lines) per testcase
#
# Each solution reads its input from stdin and prints "output: <result>".
# This runner feeds every testcase block from testcases.txt and compares the
# answer with the expected output.

# List available recipes.
default:
    @just --list

# Run a solution against every testcase in its testcases.txt.
run lang id: (_exec lang id "test")

# Same as run, but also show the full program output (debug prints) per case.
debug lang id: (_exec lang id "debug")

_exec lang id mode:
    #!/usr/bin/env bash
    set -euo pipefail

    lang='{{lang}}'
    raw_id='{{id}}'
    mode='{{mode}}'
    # Zero-pad numeric ids to 4 digits (10# avoids octal parsing of "0001").
    id=$(printf '%04d' "$((10#$raw_id))" 2>/dev/null || echo "$raw_id")

    case "$lang" in
        rust)        base='rust/src' ;;
        go)          base='go' ;;
        python|py)   base='python' ;;
        *) echo "Unknown language: $lang (use rust, go or python)" >&2; exit 1 ;;
    esac

    # Resolve the problem directory: <base>/<id>.<slug>
    prob=$(find "$base" -maxdepth 1 -type d -name "${id}.*" | head -1)
    if [ -z "$prob" ]; then
        echo "No $lang solution found for problem $id under $base/" >&2
        exit 1
    fi
    tc="$prob/testcases.txt"
    if [ ! -f "$tc" ]; then
        echo "No testcases.txt in $prob" >&2
        exit 1
    fi

    errf=$(mktemp)
    trap 'rm -f "$errf"' EXIT

    # Per-language run function: reads testcase input on stdin, prints program stdout.
    case "$lang" in
        rust)
            slug="${prob##*/}"; slug="${slug#*.}"
            # Build once up front so compiler output doesn't interleave with results.
            cargo build --quiet --manifest-path rust/Cargo.toml --bin "$slug"
            run_one() { printf '%s\n' "$1" | cargo run --quiet --manifest-path rust/Cargo.toml --bin "$slug" 2>"$errf"; }
            ;;
        go)
            reldir="${prob#go/}"
            run_one() { printf '%s\n' "$1" | ( cd go && go run "$reldir/solution.go" ) 2>"$errf"; }
            ;;
        python|py)
            py=python3
            [ -x python/.venv/bin/python ] && py=python/.venv/bin/python
            run_one() { printf '%s\n' "$1" | "$py" "$prob/solution.py" 2>"$errf"; }
            ;;
    esac

    # Colors (only when stdout is a terminal).
    if [ -t 1 ]; then
        GREEN=$'\e[32m'; RED=$'\e[31m'; DIM=$'\e[2m'; BOLD=$'\e[1m'; RST=$'\e[0m'
    else
        GREEN=''; RED=''; DIM=''; BOLD=''; RST=''
    fi

    norm() { printf '%s' "$1" | tr -d '[:space:]'; }

    echo "${BOLD}Running $lang $id ($prob)${RST}"
    echo

    # Parse testcases.txt: blocks of "input:" lines then "output:" lines,
    # separated by blank lines.
    section=''        # 'in' | 'out'
    input=''
    expected=''
    n=0
    pass=0
    fail=0

    run_case() {
        n=$((n+1))
        local got answer
        if got=$(run_one "$input"); then
            answer=$(printf '%s\n' "$got" | sed -n 's/^output:[[:space:]]*//p' | tail -1)
            if [ "$mode" = 'debug' ]; then
                echo "${BOLD}── case $n ──${RST} ${DIM}input: $(printf '%s' "$input" | tr '\n' ' ')${RST}"
                printf '%s\n' "$got" | sed 's/^/    /'
            fi
            if [ "$(norm "$answer")" = "$(norm "$expected")" ]; then
                echo "${GREEN}✓ case $n PASS${RST}  ${DIM}$(printf '%s' "$input" | tr '\n' ' ')${RST}"
                pass=$((pass+1))
                return
            fi
            echo "${RED}✗ case $n FAIL${RST}"
            echo "    input:    $(printf '%s' "$input" | tr '\n' ' ')"
            echo "    expected: $expected"
            echo "    got:      ${answer:-<no output: line>}"
        else
            echo "${RED}✗ case $n ERROR${RST}"
            echo "    input:    $(printf '%s' "$input" | tr '\n' ' ')"
            sed 's/^/    /' "$errf"
        fi
        fail=$((fail+1))
    }

    while IFS= read -r line || [ -n "$line" ]; do
        case "$line" in
            input:)
                # New case begins; flush the previous one if it had output.
                if [ -n "$expected" ]; then run_case; fi
                section='in'; input=''; expected=''
                ;;
            output:)
                section='out'
                ;;
            '')
                : # blank line — keep accumulating; case is flushed on next "input:" / EOF
                ;;
            *)
                if [ "$section" = 'in' ]; then
                    input="${input:+$input$'\n'}$line"
                elif [ "$section" = 'out' ]; then
                    expected="${expected:+$expected$'\n'}$line"
                fi
                ;;
        esac
    done < "$tc"
    # Flush the final case.
    if [ -n "$expected" ]; then run_case; fi

    echo
    if [ "$fail" -eq 0 ]; then
        echo "${GREEN}${BOLD}All $pass case(s) passed.${RST}"
    else
        echo "${BOLD}$pass passed, ${RED}$fail failed${RST}${BOLD} out of $n.${RST}"
        exit 1
    fi
