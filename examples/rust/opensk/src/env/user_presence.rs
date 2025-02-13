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
// limitations under the License.use opensk_lib::api::clock::Clock;

use opensk_lib::api::user_presence::{UserPresence, UserPresenceWaitResult};

use crate::env::WasefireEnv;

impl UserPresence for WasefireEnv {
    fn check_init(&mut self) {
        todo!()
    }

    fn wait_with_timeout(
        &mut self, _packet: &mut [u8; 64], _timeout_ms: usize,
    ) -> UserPresenceWaitResult {
        todo!()
    }

    fn check_complete(&mut self) {
        todo!()
    }
}
