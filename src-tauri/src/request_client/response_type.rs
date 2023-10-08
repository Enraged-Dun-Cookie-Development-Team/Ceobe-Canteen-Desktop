//! from https://docs.rs/tauri-runtime/0.14.1/src/tauri_runtime/http/response.rs.html#34
// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use serde_repr::{Serialize_repr, Deserialize_repr};

#[derive(Serialize_repr, Deserialize_repr, Clone, Debug)]
#[repr(u16)]
#[non_exhaustive]
/// The HTTP response type.
pub enum ResponseType {
  /// Read the response as JSON
  Json = 1,
  /// Read the response as text
  Text,
  /// Read the response as binary
  Binary,
}

