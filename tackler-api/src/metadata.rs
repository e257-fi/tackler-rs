/*
 * Copyright 2022 E257.FI
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */

pub trait MetadataItem {
    /// Get metadata item as text
    fn text(&self) -> String;
}

/// Generic checksum value
pub struct Checksum 
{
    /// used hash algorithm
    pub algorithm: String,
    /// hexadecimal hash value
    pub value: String,
}

/// Txn Set Checksum metadata item
#[allow(dead_code)]
pub struct TxnSetChecksum {
    /// size of transaction set
    pub size: u64,
    /// hash of Txn Set Checksum
    pub hash: Checksum,
}

impl MetadataItem for TxnSetChecksum {
    fn text(&self) -> String {
        let msg = format!("\
Txn Set Checksum (DEMO VERSION ONLY):
   {}: {}
   Set size: {}", self.hash.algorithm, &self.hash.value, self.size);

        return msg
    }
}

