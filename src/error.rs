/*
 * File: error.rs
 * Project: src
 * Created Date: 31/10/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 31/10/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SMemError {
    #[error("Failed to get key: {}", errno)]
    GetKeyFailed { errno: i32 },
    #[error("Failed to create shared memory: {}", errno)]
    CreateSharedMemoryFailed { errno: i32 },
    #[error("Failed to get shared memory: {}", errno)]
    GetSharedMemoryFailed { errno: i32 },
}
