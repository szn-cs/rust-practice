#! /bin/bash

test() {
    RUSTFLAGS=-Awarnings cargo test tests::sort::quick_sort_test -- --nocapture
    cargo +nightly test -p concept iterator::test_zip_iterator
    cargo check
    cargo +nightly test -p package tests::tokio -- --nocapture
}

provision() { 
    rustup toolchain install nightly --component miri
    rustup override set nightly
}