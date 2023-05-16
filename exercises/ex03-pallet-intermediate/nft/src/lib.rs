#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod tests;
pub mod types;

use frame_support::ensure;
use types::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config + scale_info::TypeInfo {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		#[pallet::constant]
		type MaxLength: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn unique_asset)]
	pub(super) type UniqueAsset<T: Config> =
		StorageMap<_, Blake2_128Concat, UniqueAssetId, UniqueAssetDetails<T, T::MaxLength>>;

	#[pallet::storage]
	#[pallet::getter(fn account)]
	/// The holdings of a specific account for a specific asset.
	pub(super) type Account<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		UniqueAssetId,
		Blake2_128Concat,
		T::AccountId,
		u128,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn nonce)]
	/// Nonce for id of the next created asset
	pub(super) type Nonce<T: Config> = StorageValue<_, UniqueAssetId, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New unique asset created
		Created {
			creator: T::AccountId,
			asset_id: UniqueAssetId,
		},
		/// Some assets have been burned
		Burned {
			asset_id: UniqueAssetId,
			owner: T::AccountId,
			total_supply: u128,
		},
		/// Some assets have been transferred
		Transferred {
			asset_id: UniqueAssetId,
			from: T::AccountId,
			to: T::AccountId,
			amount: u128,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The asset ID is unknown
		UnknownAssetId,
		/// The signing account does not own any amount of this asset
		NotOwned,
		/// Supply must be positive
		NoSupply,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn mint(
			origin: OriginFor<T>,
			metadata: BoundedVec<u8, T::MaxLength>,
			supply: u128,
		) -> DispatchResult {
			// create UniqueAssetDetails with those data
			// update UniqueAsset with the data
			// update nonce
			// mint in the account with inserting the data in Account
			// emit created event

			let origin = ensure_signed(origin)?;
			ensure!(supply > 0, Error::<T>::NoSupply);
			let asset_details =
				UniqueAssetDetails::<T, T::MaxLength>::new(origin.clone(), metadata, supply);

			let new_asset_id = Nonce::<T>::get();
			UniqueAsset::<T>::insert(new_asset_id, asset_details);

			let next_seet_id = new_asset_id + 1;
			Nonce::<T>::set(next_seet_id);

			Account::<T>::insert(new_asset_id, origin.clone(), supply);

			Self::deposit_event(Event::<T>::Created {
				asset_id: new_asset_id,
				creator: origin,
			});

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn burn(origin: OriginFor<T>, asset_id: UniqueAssetId, amount: u128) -> DispatchResult {
			// take asset count of the caller from Account, and burn with saturating_sub
			// saturating_sub the total supply from the UniqueAsset's supply
			// emit event

			let origin = ensure_signed(origin)?;

			ensure!(
				UniqueAsset::<T>::contains_key(asset_id),
				Error::<T>::UnknownAssetId
			);

			ensure!(
				Account::<T>::get(asset_id, origin.clone()) > 0,
				Error::<T>::NotOwned
			);

			let mut burned = 0;
			let mut total_supply_at_last = 0;

			Account::<T>::mutate(asset_id, origin.clone(), |asset_count| {
				let initial_asset = asset_count.clone();
				*asset_count = asset_count.saturating_sub(amount);
				// burned = *asset_count - initial_asset;
				burned = initial_asset.saturating_sub(*asset_count);
			});

			// pub(super) type UniqueAsset<T: Config> = StorageMap<_, Blake2_128Concat, UniqueAssetId, UniqueAssetDetails<T, T::MaxLength>>;

			UniqueAsset::<T>::try_mutate(asset_id, |asset_arg| -> DispatchResult {
				let asset_data = asset_arg.as_mut().ok_or(Error::<T>::UnknownAssetId)?;
				asset_data.supply = asset_data.supply.saturating_sub(burned);

				total_supply_at_last = asset_data.supply;
				Ok(())
			})?;

			Self::deposit_event(Event::<T>::Burned {
				asset_id,
				owner: origin,
				total_supply: total_supply_at_last,
			});

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn transfer(
			origin: OriginFor<T>,
			asset_id: UniqueAssetId,
			amount: u128,
			to: T::AccountId,
		) -> DispatchResult {
			// saturating_sub caller's Account
			// saturating_add to's Account with differents
			// emit event

			let origin = ensure_signed(origin)?;

			ensure!(
				UniqueAsset::<T>::contains_key(asset_id),
				Error::<T>::UnknownAssetId
			);

			ensure!(
				Account::<T>::get(asset_id, origin.clone()) > 0,
				Error::<T>::NotOwned
			);

			// ensure!(
			// 	Account::<T>::get(asset_id, origin.clone()) >= amount,
			// 	Error::<T>::NotOwned
			// );

			let mut transfer_amount = 0;

			Account::<T>::mutate(asset_id, origin.clone(), |asset_count| {
				let initial_asset = asset_count.clone();
				*asset_count = asset_count.saturating_sub(amount);
				transfer_amount = initial_asset.saturating_sub(*asset_count);
				dbg!(transfer_amount);
			});

			Account::<T>::mutate(asset_id, to.clone(), |asset_count| {
				*asset_count = asset_count.saturating_add(transfer_amount);
			});

			Self::deposit_event(Event::<T>::Transferred {
				asset_id,
				from: origin,
				to,
				amount: transfer_amount,
			});

			/*
			   Transferred {
				   asset_id: UniqueAssetId,
				   from: T::AccountId,
				   to: T::AccountId,
				   amount: u128,
			   },
			*/
			Ok(())
		}
	}
}
