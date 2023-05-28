use crate as pallet_to_mock;
use frame_support::{parameter_types, traits::ConstU64};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;
type Block = frame_system::mocking::MockBlock<TestRuntime>;

type Balance = u64;
type AccountId = u64;

parameter_types! {
  pub const ValueToMint: u64 = 200;
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

frame_support::construct_runtime!(
	pub enum TestRuntime where
		UncheckedExtrinsic = UncheckedExtrinsic,
		Block = Block,
		NodeBlock = Block,
		{
			System: frame_system,
			Balances: pallet_balances,
			PalletToMock: pallet_to_mock,
		}
);

impl frame_system::Config for TestRuntime {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_balances::Config for TestRuntime {
	type Balance = Balance;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ConstU64<1>;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
}
impl pallet_to_mock::Config for TestRuntime {
	type Event = Event;
	type Currency = Balances;
	type ValueToMint = ValueToMint;
	type SomePriceOracle = MyMock;
}

use crate::PriceOracle;

pub struct MyMock {}

impl PriceOracle for MyMock {
	type Error = ();
	fn get_price() -> Result<u64, Self::Error> {
		Ok(1000)
	}
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::default()
		.build_storage::<TestRuntime>()
		.unwrap()
		.into()
}

pub const ALICE: AccountId = 1;
