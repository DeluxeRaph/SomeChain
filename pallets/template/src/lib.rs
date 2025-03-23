//! A shell pallet built with [`frame`].
//!
//! To get started with this pallet, try implementing the guide in
//! <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html>

#![cfg_attr(not(feature = "std"), no_std)]

use frame::prelude::*;
use polkadot_sdk::polkadot_sdk_frame as frame;

// Re-export all pallet parts, this is needed to properly import the pallet into the runtime.
pub use pallet::*;

#[frame::pallet(dev_mode)]
pub mod pallet {
	use super::*;

	#[pallet::config]
	pub trait Config: polkadot_sdk::frame_system::Config {}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	pub type Balance = u128;

	#[pallet::storage]
	pub type TotalIssuance<T: Config> = StorageValue<_, Balance>;

	#[pallet::storage]
	pub type Balances<T: Config> = StorageMap<_, _, T::AccountId, Balance>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		pub fn unsafe_mint(
			origin: T::RuntimeOrigin,
			who: T::AccountId,
			amount: Balance,
		) -> DispatchResult {
			// Ensure the origin is a signed extrinsic
			let _anyone = ensure_signed(origin)?;
			// Update the balance of the destination account
			Balances::<T>::mutate(who, |balance| *balance = Some(balance.unwrap_or(0) + amount));
			// Update the total token supply
			TotalIssuance::<T>::mutate(|total| *total = Some(total.unwrap_or(0) + amount));

			Ok(())
		}
	}
}
