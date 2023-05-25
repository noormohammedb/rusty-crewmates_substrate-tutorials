#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod tests;

pub use pallet::*;

use sp_std::vec::Vec;

use frame_support::{
	// sp_runtime::traits::{CheckedConversion, CheckedMul},
	// traits::{Currency, Imbalance, TryDrop},
	traits::{Currency, Imbalance},
	transactional,
};

pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
pub type NegativeBalanceOf<T> = <<T as Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::NegativeImbalance;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config + scale_info::TypeInfo {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: Currency<Self::AccountId>;

		#[pallet::constant]
		type TreasuryAccount: Get<Self::AccountId>;
		#[pallet::constant]
		type TreasuryFlatCut: Get<BalanceOf<Self>>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::event]
	pub enum Event<T: Config> {}

	#[pallet::error]
	pub enum Error<T> {
		AccountDoesNotExist,
		ImbalanceOffsetFailed,
		WithdrawalFailed,
		Overflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn mint_to(
			origin: OriginFor<T>,
			amount: BalanceOf<T>,
			beneficiary: T::AccountId,
		) -> DispatchResult {
			ensure_root(origin)?;

			// Here we add some tokens to the chain total_issuance
			// If we do nothing more, those tokens will be removed when the `NegativeImbalance`
			// contained in the `amount_to_distribute` variable will be drop
			let amount_to_distribute = T::Currency::issue(amount);
			// TODO
			// We want to compensate this imbalance by increasing `benefeciary` balance by the
			// corresponding amount

			ensure!(
				T::Currency::resolve_into_existing(&beneficiary, amount_to_distribute).is_ok(),
				Error::<T>::AccountDoesNotExist
			);
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn slash(
			origin: OriginFor<T>,
			amount: BalanceOf<T>,
			target: T::AccountId,
		) -> DispatchResult {
			ensure_root(origin)?;

			// Todo: slash target
			// Todo: give 1/3 of the slashed amount to the treasury and burn the rest
			// Hint: use the `ration` method
			// Hint: TreasuryAccount is defined as on l35 as a Config constant

			let (neg_imbalance, _) = T::Currency::slash(&target, amount);
			let (one_by_three, _) = neg_imbalance.ration(1, 2);
			T::Currency::resolve_creating(&T::TreasuryAccount::get(), one_by_three);
			Ok(())
		}

		#[pallet::weight(0)]
		#[transactional]
		pub fn sack(
			origin: OriginFor<T>,
			sacked_accounts: Vec<T::AccountId>,
			beneficiary: T::AccountId,
		) -> DispatchResult {
			ensure_root(origin)?;

			// Todo:
			// Take as much as possible from each account in `sacked_accounts`,
			// without removing them from existence
			// and give it all to beneficiary
			// except for the TreasuryFlatCut amount, that goes to the treasury for each sacked
			// account Hint: there is a `split` method implemented on imbalances

			for acc in sacked_accounts {
				let free = T::Currency::free_balance(&acc) - T::Currency::minimum_balance();
				let (neg_imb, _) = T::Currency::slash(&acc, free);
				let (to_tre, to_ben) = neg_imb.split(T::TreasuryFlatCut::get());
				ensure!(
					T::Currency::resolve_into_existing(&beneficiary, to_ben).is_ok(),
					<Error<T>>::AccountDoesNotExist
				);
				T::Currency::resolve_creating(&T::TreasuryAccount::get(), to_tre);
			}
			Ok(())
		}
	}
}
