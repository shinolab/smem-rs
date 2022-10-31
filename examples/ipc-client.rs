/*
 * File: ipc-client.rs
 * Project: examples
 * Created Date: 31/10/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 31/10/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use smem::*;

fn main() {
    println!("enter q to finish...");

    let mut smem = SMem::new();
    smem.create("test_smem", 65536).unwrap();
    let ptr = smem.map();
    unsafe {
        loop {
            while std::ptr::read_volatile(ptr) != 0x00 {}

            print!("enter something...: ");

            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();

            std::ptr::copy_nonoverlapping(s.as_ptr(), ptr.add(1), s.len());
            std::ptr::write(ptr, 1);

            if s == "q" {
                break;
            }
        }
    }

    smem.unmap();
}
