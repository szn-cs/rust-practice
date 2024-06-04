#! /bin/bash

test() {
    RUSTFLAGS=-Awarnings cargo test tests::sort::quick_sort_test -- --nocapture
    cargo test -p concept iterator::test_zip_iterator
    cargo check
}

provision() { 
    rustup toolchain install nightly --component miri
    rustup override set nightly
}