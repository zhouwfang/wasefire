// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use core::alloc::{GlobalAlloc, Layout};
use core::num::NonZeroUsize;
use core::ptr::{NonNull, null_mut};

use const_default::ConstDefault;
use rlsf::Tlsf;

use crate::sync::Mutex;

#[unsafe(no_mangle)]
extern "C" fn init() {
    assert!(!wasefire_sync::executed!());
    const SIZE: usize = 32768;
    #[repr(align(16))]
    struct Pool([u8; SIZE]);
    static mut POOL: Pool = Pool([0; SIZE]);
    // SAFETY: This function is called at most once and POOL is only accessed here.
    let pool_ptr = NonNull::new(unsafe { &raw mut POOL.0 }).unwrap();
    let mut allocator = ALLOCATOR.0.lock();
    // SAFETY: POOL is static and won't be used again.
    let size = unsafe { allocator.insert_free_block_ptr(pool_ptr) };
    assert!(size > NonZeroUsize::new(SIZE / 2));
}

#[unsafe(no_mangle)]
extern "C" fn alloc(size: u32, align: u32) -> u32 {
    let Ok(layout) = Layout::from_size_align(size as usize, align as usize) else { return 0 };
    if size == 0 {
        return 0; // this is not checked by Layout::from_size_align()
    }
    // SAFETY: Layout has non-zero size.
    unsafe { ALLOCATOR.alloc(layout) as u32 }
}

struct Allocator(Mutex<Tlsf<'static, u8, u8, 8, 8>>);

#[global_allocator]
static ALLOCATOR: Allocator = Allocator(Mutex::new(Tlsf::DEFAULT));

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        match self.0.lock().allocate(layout) {
            None => null_mut(),
            Some(x) => x.as_ptr(),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { self.0.lock().deallocate(NonNull::new(ptr).unwrap(), layout.align()) }
    }
}
