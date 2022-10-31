/*
 * File: lib.rs
 * Project: src
 * Created Date: 31/10/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/11/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

#[cfg(not(target_os = "windows"))]
mod error;

#[cfg(not(target_os = "windows"))]
mod unix;
#[cfg(target_os = "windows")]
mod win32;

#[cfg(not(target_os = "windows"))]
pub use unix::SMem;
#[cfg(target_os = "windows")]
pub use win32::SMem;
