#![cfg_attr(not(feature = "export-abi"), no_main)]

#[cfg(feature = "export-abi")]
fn main() {
    stylus_coffee_vending_machine::print_abi("MIT-OR-APACHE-2.0", "pragma solidity ^0.8.23;");
}