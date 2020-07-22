@SETLOCAL
@CALL cargo fmt
@CALL cargo run --quiet --release -- %*
