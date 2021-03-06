// Copyright 2015-2018 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! Test rpc services.

mod dapps;
mod miner_service;
mod snapshot_service;
mod sync_provider;
mod update_service;

pub use self::dapps::TestDappsService;
pub use self::miner_service::TestMinerService;
pub use self::snapshot_service::TestSnapshotService;
pub use self::sync_provider::{Config, TestSyncProvider};
pub use self::update_service::TestUpdater;
