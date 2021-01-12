//! This crate serves as a parent for library crates in the DICOM-rs project.
//!
//! For an idiomatic API to reading and writing DICOM data, please see
//! the [`object`](crate::object) module.
pub use dicom_core as core;
pub use dicom_dictionary_std as dictionary_std;
pub use dicom_encoding as encoding;
pub use dicom_object as object;
pub use dicom_parser as parser;
pub use dicom_transfer_syntax_registry as transfer_syntax;

// re-export dicom_value macro
#[macro_export]
pub use dicom_core::dicom_value;
