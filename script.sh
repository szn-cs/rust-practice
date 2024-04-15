#! /bin/bash

test() {
    cargo test tests::sinle_linked_list_2::append -- --nocapture
}