// Copyright 2016 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.1.  This, along with the
// Licenses can be found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

use crust::PeerId;
use lru_time_cache::LruCache;
use maidsafe_utilities;

use message_filter::MessageFilter;
use messages::SignedMessage;
use std::time::Duration;

const INCOMING_EXPIRY_DURATION_SECS: u64 = 60 * 20;
const OUTGOING_EXPIRY_DURATION_SECS: u64 = 60 * 10;

// Structure to filter (throttle) incoming and outgoing signed messages.
pub struct SignedMessageFilter {
    incoming: MessageFilter<SignedMessage>,
    outgoing: LruCache<(u64, PeerId, u8), ()>,
}

impl SignedMessageFilter {
    pub fn new() -> Self {
        let incoming_duration = Duration::from_secs(INCOMING_EXPIRY_DURATION_SECS);
        let outgoing_duration = Duration::from_secs(OUTGOING_EXPIRY_DURATION_SECS);

        SignedMessageFilter {
            incoming: MessageFilter::with_expiry_duration(incoming_duration),
            outgoing: LruCache::with_expiry_duration(outgoing_duration),
        }
    }

    // Filter incoming signed message. Return the number of times this specific
    // message has been seen, including this time.
    pub fn filter_incoming(&mut self, msg: &SignedMessage) -> usize {
        self.incoming.insert(msg)
    }

    // Filter outgoing signed message. Return whether this specific message has
    // been seen recently (and thus should not be sent, due to deduplication).
    pub fn filter_outgoing(&mut self, msg: &SignedMessage, peer_id: &PeerId, route: u8) -> bool {
        let hash = maidsafe_utilities::big_endian_sip_hash(msg);
        self.outgoing.insert((hash, *peer_id, route), ()).is_some()
    }

    #[cfg(feature = "use-mock-crust")]
    pub fn clear(&mut self) {
        self.incoming.clear();
        self.outgoing.clear();
    }
}
