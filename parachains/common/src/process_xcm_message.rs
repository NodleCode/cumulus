// Copyright 2020 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Implementation of `ProcessMessage` for an `ExecuteXcm` implementation.

use cumulus_primitives_core::{AggregateMessageOrigin, ParaId};
use sp_std::{marker::PhantomData};

// We use the default xcm processor, there is no need to adapt it.
pub use xcm_builder::ProcessXcmMessage;

/// Adapter types to help with specifying a `QueuePausedQuery` of the MessageQueue pallet.
pub mod message_queue {
	use super::*;
	use frame_support::traits::QueuePausedQuery;

	/// Narrow the scope of the `Inner` query from `AggregateMessageOrigin` to `ParaId`.
	///
	/// All non-paraIds will be treated as unpaused.
	pub struct NarrowToSiblings<Inner>(PhantomData<Inner>);

	impl<Inner: QueuePausedQuery<ParaId>> QueuePausedQuery<AggregateMessageOrigin>
		for NarrowToSiblings<Inner>
	{
		fn is_paused(origin: &AggregateMessageOrigin) -> bool {
			match origin {
				AggregateMessageOrigin::Sibling(id) => Inner::is_paused(id),
				_ => false,
			}
		}
	}
}

/// Convert a sibling `ParaId` to an `AggregateMessageOrigin`.
pub struct ParaIdToSibling;
impl sp_runtime::traits::Convert<ParaId, AggregateMessageOrigin> for ParaIdToSibling {
	fn convert(para_id: ParaId) -> AggregateMessageOrigin {
		AggregateMessageOrigin::Sibling(para_id)
	}
}
