# cargo-nextest handson

## [Install](https://nexte.st/book/pre-built-binaries.html)

```shell
curl -LsSF https://get.nexte.st/latest/linux | tar zxf - -C ${CARGO_HOME:-~/.cargo}/bin
```

## Running tests

```shell
cargo nextest run

# run specified test
cargo nextest run aaa::a01
# or
cargo nextest run aaa::a01 aaa::a02

# show stdout
cargo nextest run --no-capture
```

### Retry

```shell
# with retry, meaning three attempts
cargo nextest run --retries 1
```

```console
❯ cargo nextest run --retries 1
    Finished test [unoptimized + debuginfo] target(s) in 0.01s
    Starting 4 tests across 2 binaries
        PASS [   0.004s] nextest-handson aaa::a01::tests::aaa
        PASS [   0.004s] nextest-handson aaa::a02::tests::aaa
        PASS [   0.005s] nextest-handson tests::case_1
   1/2 RETRY [   0.006s] nextest-handson flaky::tests::rand

--- TRY 1 STDOUT:        nextest-handson flaky::tests::rand ---

running 1 test
test flaky::tests::rand ... FAILED

failures:

failures:
    flaky::tests::rand

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s


--- TRY 1 STDERR:        nextest-handson flaky::tests::rand ---
thread 'flaky::tests::rand' panicked at 'assertion failed: false', src/flaky.rs:8:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

  TRY 2 PASS [   0.003s] nextest-handson flaky::tests::rand
     Summary [   0.009s] 4 tests run: 4 passed (1 flaky), 0 skipped
```

### Report options

* There are 7 status levels(like logging fatal, error, info, debug,...): none, fail, retry, slow, pass, skip, all
```console
❯ cargo nextest run aaa::a01 --status-level skip
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
    Starting 1 tests across 2 binaries (3 skipped)
        SKIP [         ] nextest-handson aaa::a02::tests::aaa
        SKIP [         ] nextest-handson flaky::tests::rand
        SKIP [         ] nextest-handson tests::case_1
        PASS [   0.005s] nextest-handson aaa::a01::tests::aaa
     Summary [   0.005s] 1 tests run: 1 passed, 3 skipped
```

## List tests
```shell
cargo nextest list
```

## Partitioning test

```console
❯ cargo nextest run --partition count:1/2
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
    Starting 3 tests across 2 binaries (1 skipped)
        PASS [   0.004s] nextest-handson aaa::a01::tests::aaa
        PASS [   0.004s] nextest-handson tests::case_1
        PASS [   0.004s] nextest-handson flaky::tests::rand
     Summary [   0.006s] 3 tests run: 3 passed, 1 skipped

❯ cargo nextest run --partition count:2/2
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
    Starting 1 tests across 2 binaries (3 skipped)
        PASS [   0.003s] nextest-handson aaa::a02::tests::aaa
     Summary [   0.003s] 1 tests run: 1 passed, 3 skipped
```

## Config

create `.config/nextest.yaml`

### Switch profile

`.config/nextest.yaml`
```yaml
[profile.ci]
# Print out output for failing tests as soon as they fail, and also at the end
# of the run (for easy scrollability).
failure-output = "immediate-final"
# Do not cancel the test run on the first failure.
fail-fast = false
```

`cargo nextest run --profile ci`


