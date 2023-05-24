use super::*;
// got the "file not included in module tree" warning ?
// look at the comment in Cargo.toml (in the feature section), and the one at the top of the lib.rs
// file
//
use crate::Pallet as Weights;
use frame_benchmarking::{account as benchmark_account, benchmarks};
use frame_system::RawOrigin;

benchmarks! {
	/////////////////////// Part 2 - benchmarks ///////////////////////

	//TODO: change this generic benchmark to benchmark the duplicate_and_store extrinsic
	duplicate_and_store {
		//this variable is a range, meaning the benchmark will be run with the different values of
		//s, to evaluate the weight of this specific parameter
		let s in 1 .. 10000;
		// todo("change this range to something that makes sense for your benchmark");

		// let root = todo!("get the root origin, to sign our transactions");
		let root: T::AccountId = benchmark_account("Alice", 0, 0);


		// Now that we have all the parameters we need for our extrinsic's benchmark, we can call
		// it:
	}: duplicate_and_store(RawOrigin::Signed(root), 0, s.clone())
	verify {
		// Run some verifications here.
		// If something isn't right, the benchmark will throw an error and wont output values
		assert_eq!(<Weights<T> as Store>::VecDup::get(), Some([0].repeat(s as usize)));
	}

	/////////////////////// Part 3.A - conditional benchmarks ///////////////////////
	store_maybe_hashed_true {
		//TODO: prepare the datas for this benchmark (the account, the data, and the hash)
		// let root = todo!("get the root origin, to sign our transactions");
		let root: T::AccountId = benchmark_account("Alice", 0, 0);
		// let data = sp_std::vec![1,2,3,4,5];
		// let data = (1..100).collect::<Vec<u8>>();
		let data = sp_std::vec![0;100_000];
		let hash = true;
	}: store_maybe_hashed(RawOrigin::Signed(root), data.clone(), hash)
	verify {
		//TODO: do some verification that your extrinsic did what it was supposed to do
		assert_eq!(<Weights<T> as Store>::Data::get(), Some(blake2_256(&data).as_ref().to_vec()));
	}

	store_maybe_hashed_false {
		//TODO: prepare the datas for this benchmark (the account, the data, and the hash)
		// let root = todo!("get the root origin, to sign our transactions");
		let root: T::AccountId = benchmark_account("Alice", 0, 0);
		// let data = sp_std::vec![1,2,3,4,5];
		// let data = (1..100).collect::<Vec<u8>>();
		let data = sp_std::vec![0;100_000];
		let hash = false;
	}: store_maybe_hashed(RawOrigin::Signed(root), data.clone(), hash)
	verify {
		//TODO: do some verification that your extrinsic did what it was supposed to do
		// let hash = blake2_256(&data);
		assert_eq!(<Weights<T> as Store>::Data::get(), Some(data));
	}

	impl_benchmark_test_suite!(Weights, crate::mock::new_test_ext(), crate::mock::Test);
}
