// Copyright 2024 Google LLC
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

use core::slice;

use bytemuck::{bytes_of, pod_read_unaligned};

use crate::error::*;
use crate::module::Parser;

#[allow(dead_code)]
pub struct SideTable<'m> {
    indices: &'m [u16], // including 0 and the length of metadata_array
    metadata: &'m [u16],
}

#[allow(dead_code)]
impl<'m> SideTable<'m> {
    fn metadata(&self, func_idx: usize) -> Metadata<'m> {
        Metadata(
            &self.metadata[self.indices[func_idx] as usize .. self.indices[func_idx + 1] as usize],
        )
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
struct Metadata<'m>(&'m [u16]);

#[allow(dead_code)]
impl<'m> Metadata<'m> {
    pub fn type_idx(&self) -> usize {
        self.0[0] as usize
    }

    pub fn parser(&self, code: &'m [u8]) -> Parser<'m> {
        unsafe { Parser::new(&code[self.parser_pos(1, 3) .. self.parser_pos(3, 5)]) }
    }

    pub fn branch_table(&self) -> &[BranchTableEntry] {
        let bytes = &self.0[5 ..];
        unsafe {
            slice::from_raw_parts(bytes.as_ptr() as *const BranchTableEntry, self.0.len() - 5)
        }
    }

    fn parser_pos(&self, start: usize, end: usize) -> usize {
        debug_assert_eq!(start + 2, end);
        let pair: &[u16; 2] = &self.0[start .. end].try_into().unwrap();
        pod_read_unaligned::<u32>(bytes_of(pair)) as usize
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct BranchTableEntry([u16; 3]);

pub struct BranchTableEntryView {
    /// The amount to adjust the instruction pointer by if the branch is taken.
    pub delta_ip: i32,
    /// The amount to adjust the side-table pointer by if the branch is taken.
    pub delta_stp: i32,
    /// The number of values that will be copied if the branch is taken.
    pub val_cnt: u32,
    /// The number of values that will be popped if the branch is taken.
    pub pop_cnt: u32,
}

impl BranchTableEntry {
    pub fn new(view: BranchTableEntryView) -> Result<Self, Error> {
        Ok(BranchTableEntry([
            view.delta_ip as u16,
            view.delta_stp as u16,
            ((view.pop_cnt & 0xfff) << 4) as u16 | (view.val_cnt & 0xf) as u16,
        ]))
    }

    pub fn view(self) -> BranchTableEntryView {
        BranchTableEntryView {
            delta_ip: (self.0[0] as i16) as i32,
            delta_stp: (self.0[1] as i16) as i32,
            val_cnt: (self.0[2] & 0xf) as u32,
            pop_cnt: ((self.0[2] >> 4) & 0xfff) as u32,
        }
    }

    pub fn is_invalid(self) -> bool {
        self.0.iter().all(|&x| x == 0)
    }

    pub fn invalid() -> Self {
        BranchTableEntry([0; 3])
    }
}
