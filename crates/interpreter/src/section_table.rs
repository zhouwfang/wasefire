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

pub struct SectionTable {
    /// Parser sizes in the Element section.
    pub elem_parser_sizes: Vec<32>,
    /// Parser sizes in the Code section.
    pub func_parser_sizes: Vec<u32>,
    /// Parser sizes in the Data section.
    pub data_parser_sizes: Vec<u32>,
}
