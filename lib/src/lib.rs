//! This crate is a work in progress of implementation of an RDF and SPARQL software stack in Rust.
//!
//! Its goal is to provide a compliant, safe and fast implementation of W3C specifications.
//!
//! It currently provides:
//! * Basic RDF data structures in the `model` package
//! * Parsers for XML, Turtle and N-Triples syntaxes in the `rio` package
//! * A memory based and a disk based stores in the `store` package
//! * A work in progress SPARQL implementation in the `sparql` package

#![cfg_attr(
    feature = "cargo-clippy",
    warn(
        cast_possible_truncation,
        cast_possible_wrap,
        cast_precision_loss,
        cast_sign_loss,
        default_trait_access,
        empty_enum,
        enum_glob_use,
        expl_impl_clone_on_copy,
        explicit_into_iter_loop,
        filter_map,
        if_not_else,
        inline_always,
        invalid_upcast_comparisons,
        items_after_statements,
        linkedlist,
        //TODO match_same_arms,
        maybe_infinite_iter,
        mut_mut,
        needless_continue,
        option_map_unwrap_or,
        //TODO option_map_unwrap_or_else,
        pub_enum_variant_names,
        replace_consts,
        result_map_unwrap_or_else,
        //TODO single_match_else,
        string_add_assign,
        unicode_not_nfc
    )
)]

extern crate byteorder;
extern crate chrono;
#[macro_use]
extern crate failure;
extern crate language_tags;
#[macro_use]
extern crate lazy_static;
extern crate num_traits;
extern crate ordered_float;
extern crate quick_xml;
extern crate regex;
#[cfg(feature = "rocksdb")]
extern crate rocksdb;
extern crate rust_decimal;
extern crate url;
extern crate uuid;

pub mod model;
pub mod rio;
pub mod sparql;
pub mod store;
mod utils;

pub use failure::Error;
pub type Result<T> = ::std::result::Result<T, failure::Error>;
