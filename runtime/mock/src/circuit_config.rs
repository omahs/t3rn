use crate::*;

use frame_support::{parameter_types, traits::ConstU32, PalletId};
use sp_core::H256;
use t3rn_primitives::{bridges::runtime as bp_runtime, common::DEFAULT_ROUND_TERM};

use crate::xbi_config::XBIPortalRuntimeEntry;

use circuit_runtime_pallets::pallet_mfv;

impl t3rn_primitives::EscrowTrait<Runtime> for Runtime {
    type Currency = Balances;
    type Time = Timestamp;
}

impl pallet_clock::Config for Runtime {
    type AccountManager = AccountManager;
    type Event = Event;
    type Executors = t3rn_primitives::executors::ExecutorsMock<Self>;
    type RoundDuration = ConstU32<500u32>;
    type Treasury = Treasury;
}

impl pallet_xdns::Config for Runtime {
    type Balances = Balances;
    type Escrowed = Self;
    type Event = Event;
    type WeightInfo = pallet_xdns::weights::SubstrateWeight<Runtime>;
}

impl pallet_contracts_registry::Config for Runtime {
    type Balances = Balances;
    type Escrowed = AccountManager;
    type Event = Event;
    type WeightInfo = pallet_contracts_registry::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
    pub const PortalPalletId: PalletId = PalletId(*b"pal/port");
}
pub struct AccountId32Converter;
impl Convert<AccountId, [u8; 32]> for AccountId32Converter {
    fn convert(account_id: AccountId) -> [u8; 32] {
        account_id.into()
    }
}

impl pallet_circuit_portal::Config for Runtime {
    type AccountId32Converter = AccountId32Converter;
    type Balances = Balances;
    type Call = Call;
    type Escrowed = Self;
    // type EthVerifier = ethereum_light_client::Pallet<Runtime>;
    type EthVerifier = t3rn_protocol::side_effects::confirm::ethereum::EthereumMockVerifier;
    type Event = Event;
    type PalletId = PortalPalletId;
    type WeightInfo = pallet_circuit_portal::weights::SubstrateWeight<Runtime>;
    type Xdns = XDNS;
}

parameter_types! {
    pub const CircuitPalletId: PalletId = PalletId(*b"pal/circ");
    pub const SelfGatewayId: [u8; 4] = [3, 3, 3, 3];
}

impl pallet_circuit::Config for Runtime {
    type AccountManager = AccountManager;
    type Balances = Balances;
    type Call = Call;
    type CircuitPortal = CircuitPortal;
    type DeletionQueueLimit = ConstU32<100u32>;
    type Escrowed = Self;
    type Event = Event;
    type Executors = t3rn_primitives::executors::ExecutorsMock<Self>;
    type MultiCurrency = ORMLTokens;
    type PalletId = CircuitPalletId;
    type SelfGatewayId = SelfGatewayId;
    type SelfParaId = ConstU32<3333u32>;
    type SignalQueueDepth = ConstU32<5u32>;
    type WeightInfo = ();
    type XBIPortal = XBIPortalRuntimeEntry;
    type XBIPromise = XBIPortal;
    type Xdns = XDNS;
    type XtxTimeoutCheckInterval = ConstU32<10u32>;
    type XtxTimeoutDefault = ConstU32<400u32>;
}

parameter_types! {
    pub const MaxRequests: u32 = 2;
    pub const HeadersToKeep: u32 = 100;
}

type DefaultPolkadotBridgeInstance = ();
type Blake2ValU32BridgeInstance = pallet_mfv::Instance1;
type Blake2ValU64BridgeInstance = pallet_mfv::Instance2;
type Keccak256ValU64BridgeInstance = pallet_mfv::Instance3;
type Keccak256ValU32BridgeInstance = pallet_mfv::Instance4;

#[derive(Debug)]
pub struct Blake2ValU64Chain;
impl bp_runtime::Chain for Blake2ValU64Chain {
    type BlockNumber = <Runtime as frame_system::Config>::BlockNumber;
    type Hash = <Runtime as frame_system::Config>::Hash;
    type Hasher = <Runtime as frame_system::Config>::Hashing;
    type Header = <Runtime as frame_system::Config>::Header;
}

#[derive(Debug)]
pub struct Blake2ValU32Chain;
impl bp_runtime::Chain for Blake2ValU32Chain {
    type BlockNumber = u32;
    type Hash = H256;
    type Hasher = BlakeTwo256;
    type Header = sp_runtime::generic::Header<u32, BlakeTwo256>;
}

#[derive(Debug)]
pub struct Keccak256ValU64Chain;
impl bp_runtime::Chain for Keccak256ValU64Chain {
    type BlockNumber = u64;
    type Hash = H256;
    type Hasher = Keccak256;
    type Header = sp_runtime::generic::Header<u64, Keccak256>;
}

#[derive(Debug)]
pub struct Keccak256ValU32Chain;
impl bp_runtime::Chain for Keccak256ValU32Chain {
    type BlockNumber = u32;
    type Hash = H256;
    type Hasher = Keccak256;
    type Header = sp_runtime::generic::Header<u32, Keccak256>;
}

impl pallet_mfv::Config<Blake2ValU64BridgeInstance> for Runtime {
    type BridgedChain = Blake2ValU64Chain;
    type Escrowed = Self;
    type Event = Event;
    type HeadersToKeep = HeadersToKeep;
    type MaxRequests = MaxRequests;
    type WeightInfo = ();
    type Xdns = XDNS;
}

impl pallet_mfv::Config<Blake2ValU32BridgeInstance> for Runtime {
    type BridgedChain = Blake2ValU32Chain;
    type Escrowed = Self;
    type Event = Event;
    type HeadersToKeep = HeadersToKeep;
    type MaxRequests = MaxRequests;
    type WeightInfo = ();
    type Xdns = XDNS;
}

impl pallet_mfv::Config<Keccak256ValU64BridgeInstance> for Runtime {
    type BridgedChain = Keccak256ValU64Chain;
    type Escrowed = Self;
    type Event = Event;
    type HeadersToKeep = HeadersToKeep;
    type MaxRequests = MaxRequests;
    type WeightInfo = ();
    type Xdns = XDNS;
}

impl pallet_mfv::Config<Keccak256ValU32BridgeInstance> for Runtime {
    type BridgedChain = Keccak256ValU32Chain;
    type Escrowed = Self;
    type Event = Event;
    type HeadersToKeep = HeadersToKeep;
    type MaxRequests = MaxRequests;
    type WeightInfo = ();
    type Xdns = XDNS;
}

impl pallet_mfv::Config<DefaultPolkadotBridgeInstance> for Runtime {
    type BridgedChain = Blake2ValU32Chain;
    type Escrowed = Self;
    type Event = Event;
    type HeadersToKeep = HeadersToKeep;
    type MaxRequests = MaxRequests;
    type WeightInfo = ();
    type Xdns = XDNS;
}

// MinRoundTerm plays a crucial role:
//  + must at least be the size of the active collator set
//  + is applied as default round term during genesis
//  + codetermines staking delays as they are measured in rounds
parameter_types! {
    pub const TreasuryAccount: AccountId = AccountId::new([0u8; 32]); // TODO
    pub const ReserveAccount: AccountId = AccountId::new([1u8; 32]); // TODO
    pub const AuctionFund: AccountId = AccountId::new([2u8; 32]); // TODO
    pub const ContractFund: AccountId = AccountId::new([3u8; 32]); // TODO
    pub const MinRoundTerm: u32 = 20; // TODO
    pub const DefaultRoundTerm: u32 = DEFAULT_ROUND_TERM; // TODO
    pub const GenesisIssuance: u32 = 20_000_000; // TODO
    pub const IdealPerpetualInflation: Perbill =Perbill::from_percent(1);
    pub const InflationRegressionMonths: u32 = 72;
}

impl pallet_treasury::Config for Runtime {
    type AuctionFund = AuctionFund;
    type ContractFund = ContractFund;
    type Currency = Balances;
    type DefaultRoundTerm = DefaultRoundTerm;
    type Event = Event;
    type GenesisIssuance = GenesisIssuance;
    type IdealPerpetualInflation = IdealPerpetualInflation;
    type InflationRegressionMonths = InflationRegressionMonths;
    type MinRoundTerm = MinRoundTerm;
    type ReserveAccount = ReserveAccount;
    type TreasuryAccount = TreasuryAccount;
    type WeightInfo = pallet_treasury::weights::TreasuryWeight<Runtime>;
}