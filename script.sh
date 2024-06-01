#! /bin/bash

test() {
    RUSTFLAGS=-Awarnings cargo test tests::sinle_linked_list_2::append -- --nocapture
    cargo check
}

provision() { 
    rustup toolchain install nightly --component miri
    rustup override set nightly
}