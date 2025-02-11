// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import "@polkadot/api-base/types/events";

import type { ApiTypes, AugmentedEvent } from "@polkadot/api-base/types";
import type {
  Bytes,
  Null,
  Option,
  Result,
  U256,
  U8aFixed,
  Vec,
  bool,
  u128,
  u32,
  u64,
  u8,
} from "@polkadot/types-codec";
import type { ITuple } from "@polkadot/types-codec/types";
import type {
  AccountId32,
  H160,
  H256,
  Perbill,
} from "@polkadot/types/interfaces/runtime";
import type {
  EthereumLog,
  FrameSupportTokensMiscBalanceStatus,
  FrameSupportWeightsDispatchInfo,
  PalletXbiPortalXbiFormatXbiCheckOutStatus,
  PalletXbiPortalXbiFormatXbiNotificationKind,
  SpFinalityGrandpaAppPublic,
  SpRuntimeDispatchError,
  T3rnPrimitivesContractMetadataContractType,
  T3rnPrimitivesSideEffectFullSideEffect,
  T3rnSdkPrimitivesSignalSignalKind,
  T3rnTypesSideEffect,
} from "@polkadot/types/lookup";

export type __AugmentedEvent<ApiType extends ApiTypes> =
  AugmentedEvent<ApiType>;

declare module "@polkadot/api-base/types/events" {
  interface AugmentedEvents<ApiType extends ApiTypes> {
    accountManager: {
      ContractsRegistryExecutionFinalized: AugmentedEvent<
        ApiType,
        [executionId: u64],
        { executionId: u64 }
      >;
      DepositReceived: AugmentedEvent<
        ApiType,
        [
          chargeId: H256,
          payee: AccountId32,
          recipient: AccountId32,
          amount: u128
        ],
        {
          chargeId: H256;
          payee: AccountId32;
          recipient: AccountId32;
          amount: u128;
        }
      >;
      Issued: AugmentedEvent<
        ApiType,
        [recipient: AccountId32, amount: u128],
        { recipient: AccountId32; amount: u128 }
      >;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    assets: {
      /** An approval for account `delegate` was cancelled by `owner`. */
      ApprovalCancelled: AugmentedEvent<
        ApiType,
        [assetId: u32, owner: AccountId32, delegate: AccountId32],
        { assetId: u32; owner: AccountId32; delegate: AccountId32 }
      >;
      /** (Additional) funds have been approved for transfer to a destination account. */
      ApprovedTransfer: AugmentedEvent<
        ApiType,
        [
          assetId: u32,
          source: AccountId32,
          delegate: AccountId32,
          amount: u128
        ],
        {
          assetId: u32;
          source: AccountId32;
          delegate: AccountId32;
          amount: u128;
        }
      >;
      /** Some asset `asset_id` was frozen. */
      AssetFrozen: AugmentedEvent<ApiType, [assetId: u32], { assetId: u32 }>;
      /** An asset has had its attributes changed by the `Force` origin. */
      AssetStatusChanged: AugmentedEvent<
        ApiType,
        [assetId: u32],
        { assetId: u32 }
      >;
      /** Some asset `asset_id` was thawed. */
      AssetThawed: AugmentedEvent<ApiType, [assetId: u32], { assetId: u32 }>;
      /** Some assets were destroyed. */
      Burned: AugmentedEvent<
        ApiType,
        [assetId: u32, owner: AccountId32, balance: u128],
        { assetId: u32; owner: AccountId32; balance: u128 }
      >;
      /** Some asset class was created. */
      Created: AugmentedEvent<
        ApiType,
        [assetId: u32, creator: AccountId32, owner: AccountId32],
        { assetId: u32; creator: AccountId32; owner: AccountId32 }
      >;
      /** An asset class was destroyed. */
      Destroyed: AugmentedEvent<ApiType, [assetId: u32], { assetId: u32 }>;
      /** Some asset class was force-created. */
      ForceCreated: AugmentedEvent<
        ApiType,
        [assetId: u32, owner: AccountId32],
        { assetId: u32; owner: AccountId32 }
      >;
      /** Some account `who` was frozen. */
      Frozen: AugmentedEvent<
        ApiType,
        [assetId: u32, who: AccountId32],
        { assetId: u32; who: AccountId32 }
      >;
      /** Some assets were issued. */
      Issued: AugmentedEvent<
        ApiType,
        [assetId: u32, owner: AccountId32, totalSupply: u128],
        { assetId: u32; owner: AccountId32; totalSupply: u128 }
      >;
      /** Metadata has been cleared for an asset. */
      MetadataCleared: AugmentedEvent<
        ApiType,
        [assetId: u32],
        { assetId: u32 }
      >;
      /** New metadata has been set for an asset. */
      MetadataSet: AugmentedEvent<
        ApiType,
        [
          assetId: u32,
          name: Bytes,
          symbol_: Bytes,
          decimals: u8,
          isFrozen: bool
        ],
        {
          assetId: u32;
          name: Bytes;
          symbol: Bytes;
          decimals: u8;
          isFrozen: bool;
        }
      >;
      /** The owner changed. */
      OwnerChanged: AugmentedEvent<
        ApiType,
        [assetId: u32, owner: AccountId32],
        { assetId: u32; owner: AccountId32 }
      >;
      /** The management team changed. */
      TeamChanged: AugmentedEvent<
        ApiType,
        [
          assetId: u32,
          issuer: AccountId32,
          admin: AccountId32,
          freezer: AccountId32
        ],
        {
          assetId: u32;
          issuer: AccountId32;
          admin: AccountId32;
          freezer: AccountId32;
        }
      >;
      /** Some account `who` was thawed. */
      Thawed: AugmentedEvent<
        ApiType,
        [assetId: u32, who: AccountId32],
        { assetId: u32; who: AccountId32 }
      >;
      /** Some assets were transferred. */
      Transferred: AugmentedEvent<
        ApiType,
        [assetId: u32, from: AccountId32, to: AccountId32, amount: u128],
        { assetId: u32; from: AccountId32; to: AccountId32; amount: u128 }
      >;
      /**
       * An `amount` was transferred in its entirety from `owner` to
       * `destination` by the approved `delegate`.
       */
      TransferredApproved: AugmentedEvent<
        ApiType,
        [
          assetId: u32,
          owner: AccountId32,
          delegate: AccountId32,
          destination: AccountId32,
          amount: u128
        ],
        {
          assetId: u32;
          owner: AccountId32;
          delegate: AccountId32;
          destination: AccountId32;
          amount: u128;
        }
      >;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    balances: {
      /** A balance was set by root. */
      BalanceSet: AugmentedEvent<
        ApiType,
        [who: AccountId32, free: u128, reserved: u128],
        { who: AccountId32; free: u128; reserved: u128 }
      >;
      /** Some amount was deposited (e.g. for transaction fees). */
      Deposit: AugmentedEvent<
        ApiType,
        [who: AccountId32, amount: u128],
        { who: AccountId32; amount: u128 }
      >;
      /**
       * An account was removed whose balance was non-zero but below
       * ExistentialDeposit, resulting in an outright loss.
       */
      DustLost: AugmentedEvent<
        ApiType,
        [account: AccountId32, amount: u128],
        { account: AccountId32; amount: u128 }
      >;
      /** An account was created with some free balance. */
      Endowed: AugmentedEvent<
        ApiType,
        [account: AccountId32, freeBalance: u128],
        { account: AccountId32; freeBalance: u128 }
      >;
      /** Some balance was reserved (moved from free to reserved). */
      Reserved: AugmentedEvent<
        ApiType,
        [who: AccountId32, amount: u128],
        { who: AccountId32; amount: u128 }
      >;
      /**
       * Some balance was moved from the reserve of the first account to the
       * second account. Final argument indicates the destination balance type.
       */
      ReserveRepatriated: AugmentedEvent<
        ApiType,
        [
          from: AccountId32,
          to: AccountId32,
          amount: u128,
          destinationStatus: FrameSupportTokensMiscBalanceStatus
        ],
        {
          from: AccountId32;
          to: AccountId32;
          amount: u128;
          destinationStatus: FrameSupportTokensMiscBalanceStatus;
        }
      >;
      /** Some amount was removed from the account (e.g. for misbehavior). */
      Slashed: AugmentedEvent<
        ApiType,
        [who: AccountId32, amount: u128],
        { who: AccountId32; amount: u128 }
      >;
      /** Transfer succeeded. */
      Transfer: AugmentedEvent<
        ApiType,
        [from: AccountId32, to: AccountId32, amount: u128],
        { from: AccountId32; to: AccountId32; amount: u128 }
      >;
      /** Some balance was unreserved (moved from reserved to free). */
      Unreserved: AugmentedEvent<
        ApiType,
        [who: AccountId32, amount: u128],
        { who: AccountId32; amount: u128 }
      >;
      /** Some amount was withdrawn from the account (e.g. for transaction fees). */
      Withdraw: AugmentedEvent<
        ApiType,
        [who: AccountId32, amount: u128],
        { who: AccountId32; amount: u128 }
      >;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    circuit: {
      AddLiquidity: AugmentedEvent<
        ApiType,
        [AccountId32, u64, u64, u128, u128, u128]
      >;
      CallCustom: AugmentedEvent<
        ApiType,
        [AccountId32, AccountId32, AccountId32, u128, Bytes, u64, Bytes]
      >;
      CallEvm: AugmentedEvent<
        ApiType,
        [
          AccountId32,
          H160,
          H160,
          U256,
          Bytes,
          u64,
          U256,
          Option<U256>,
          Option<U256>,
          Vec<ITuple<[H160, Vec<H256>]>>
        ]
      >;
      CallNative: AugmentedEvent<ApiType, [AccountId32, Bytes]>;
      CallWasm: AugmentedEvent<
        ApiType,
        [AccountId32, AccountId32, u128, u64, Option<u128>, Bytes]
      >;
      CancelledSideEffects: AugmentedEvent<
        ApiType,
        [AccountId32, H256, Vec<T3rnTypesSideEffect>]
      >;
      EscrowTransfer: AugmentedEvent<ApiType, [AccountId32, AccountId32, u128]>;
      NewSideEffectsAvailable: AugmentedEvent<
        ApiType,
        [AccountId32, H256, Vec<T3rnTypesSideEffect>, Vec<H256>]
      >;
      Notification: AugmentedEvent<
        ApiType,
        [
          AccountId32,
          AccountId32,
          PalletXbiPortalXbiFormatXbiNotificationKind,
          Bytes,
          Bytes
        ]
      >;
      Result: AugmentedEvent<
        ApiType,
        [
          AccountId32,
          AccountId32,
          PalletXbiPortalXbiFormatXbiCheckOutStatus,
          Bytes,
          Bytes
        ]
      >;
      SFXNewBidReceived: AugmentedEvent<
        ApiType,
        [H256, H256, AccountId32, u128]
      >;
      SideEffectConfirmed: AugmentedEvent<ApiType, [H256]>;
      SideEffectInsuranceReceived: AugmentedEvent<ApiType, [H256, AccountId32]>;
      SideEffectsConfirmed: AugmentedEvent<
        ApiType,
        [H256, Vec<Vec<T3rnPrimitivesSideEffectFullSideEffect>>]
      >;
      Swap: AugmentedEvent<ApiType, [AccountId32, u64, u64, u128, u128, u128]>;
      Transfer: AugmentedEvent<
        ApiType,
        [AccountId32, AccountId32, AccountId32, u128]
      >;
      TransferAssets: AugmentedEvent<
        ApiType,
        [AccountId32, u64, AccountId32, AccountId32, u128]
      >;
      TransferORML: AugmentedEvent<
        ApiType,
        [AccountId32, u64, AccountId32, AccountId32, u128]
      >;
      XTransactionReadyForExec: AugmentedEvent<ApiType, [H256]>;
      XTransactionReceivedForExec: AugmentedEvent<ApiType, [H256]>;
      XTransactionStepFinishedExec: AugmentedEvent<ApiType, [H256]>;
      XTransactionXtxFinishedExecAllSteps: AugmentedEvent<ApiType, [H256]>;
      XTransactionXtxRevertedAfterTimeOut: AugmentedEvent<ApiType, [H256]>;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    clock: {
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    contracts: {
      /** A code with the specified hash was removed. */
      CodeRemoved: AugmentedEvent<
        ApiType,
        [codeHash: H256],
        { codeHash: H256 }
      >;
      /** Code with the specified hash has been stored. */
      CodeStored: AugmentedEvent<ApiType, [codeHash: H256], { codeHash: H256 }>;
      /** A contract's code was updated. */
      ContractCodeUpdated: AugmentedEvent<
        ApiType,
        [contract: AccountId32, newCodeHash: H256, oldCodeHash: H256],
        { contract: AccountId32; newCodeHash: H256; oldCodeHash: H256 }
      >;
      /** A custom event emitted by the contract. */
      ContractEmitted: AugmentedEvent<
        ApiType,
        [contract: AccountId32, data: Bytes],
        { contract: AccountId32; data: Bytes }
      >;
      /** Contract deployed by address at the specified address. */
      Instantiated: AugmentedEvent<
        ApiType,
        [deployer: AccountId32, contract: AccountId32],
        { deployer: AccountId32; contract: AccountId32 }
      >;
      /**
       * Contract has been removed.
       *
       * # Note
       *
       * The only way for a contract to be removed and emitting this event is by
       * calling `seal_terminate`.
       */
      Terminated: AugmentedEvent<
        ApiType,
        [contract: AccountId32, beneficiary: AccountId32],
        { contract: AccountId32; beneficiary: AccountId32 }
      >;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    contractsRegistry: {
      /** [requester, contract_id] */
      ContractPurged: AugmentedEvent<ApiType, [AccountId32, H256]>;
      /** [requester, contract_id] */
      ContractStored: AugmentedEvent<ApiType, [AccountId32, H256]>;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    evm: {
      /** A deposit has been made at a given address. [sender, address, value] */
      BalanceDeposit: AugmentedEvent<ApiType, [AccountId32, H160, U256]>;
      /** A withdrawal has been made from a given address. [sender, address, value] */
      BalanceWithdraw: AugmentedEvent<ApiType, [AccountId32, H160, U256]>;
      ClaimAccount: AugmentedEvent<
        ApiType,
        [accountId: AccountId32, evmAddress: H160],
        { accountId: AccountId32; evmAddress: H160 }
      >;
      /** A contract has been created at given [address]. */
      Created: AugmentedEvent<ApiType, [H160]>;
      /** A [contract] was attempted to be created, but the execution failed. */
      CreatedFailed: AugmentedEvent<ApiType, [H160]>;
      /** A [contract] has been executed successfully with states applied. */
      Executed: AugmentedEvent<ApiType, [H160]>;
      /**
       * A [contract] has been executed with errors. States are reverted with
       * only gas fees applied.
       */
      ExecutedFailed: AugmentedEvent<ApiType, [H160]>;
      /** Ethereum events from contracts. */
      Log: AugmentedEvent<ApiType, [EthereumLog]>;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    grandpa: {
      /** New authority set has been applied. */
      NewAuthorities: AugmentedEvent<
        ApiType,
        [authoritySet: Vec<ITuple<[SpFinalityGrandpaAppPublic, u64]>>],
        { authoritySet: Vec<ITuple<[SpFinalityGrandpaAppPublic, u64]>> }
      >;
      /** Current authority set has been paused. */
      Paused: AugmentedEvent<ApiType, []>;
      /** Current authority set has been resumed. */
      Resumed: AugmentedEvent<ApiType, []>;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    portal: {
      /**
       * Event documentation should end with an array that provides descriptive
       * names for event Gateway was registered successsfully. [ChainId]
       */
      GatewayRegistered: AugmentedEvent<ApiType, [U8aFixed]>;
      /** Header was successfully added */
      HeaderSubmitted: AugmentedEvent<ApiType, [U8aFixed, Bytes]>;
      /** Gateway was set operational. [ChainId, bool] */
      SetOperational: AugmentedEvent<ApiType, [U8aFixed, bool]>;
      /** Gateway owner was set successfully. [ChainId, Vec<u8>] */
      SetOwner: AugmentedEvent<ApiType, [U8aFixed, Bytes]>;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    sudo: {
      /** The [sudoer] just switched identity; the old key is supplied if one existed. */
      KeyChanged: AugmentedEvent<
        ApiType,
        [oldSudoer: Option<AccountId32>],
        { oldSudoer: Option<AccountId32> }
      >;
      /** A sudo just took place. [result] */
      Sudid: AugmentedEvent<
        ApiType,
        [sudoResult: Result<Null, SpRuntimeDispatchError>],
        { sudoResult: Result<Null, SpRuntimeDispatchError> }
      >;
      /** A sudo just took place. [result] */
      SudoAsDone: AugmentedEvent<
        ApiType,
        [sudoResult: Result<Null, SpRuntimeDispatchError>],
        { sudoResult: Result<Null, SpRuntimeDispatchError> }
      >;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    system: {
      /** `:code` was updated. */
      CodeUpdated: AugmentedEvent<ApiType, []>;
      /** An extrinsic failed. */
      ExtrinsicFailed: AugmentedEvent<
        ApiType,
        [
          dispatchError: SpRuntimeDispatchError,
          dispatchInfo: FrameSupportWeightsDispatchInfo
        ],
        {
          dispatchError: SpRuntimeDispatchError;
          dispatchInfo: FrameSupportWeightsDispatchInfo;
        }
      >;
      /** An extrinsic completed successfully. */
      ExtrinsicSuccess: AugmentedEvent<
        ApiType,
        [dispatchInfo: FrameSupportWeightsDispatchInfo],
        { dispatchInfo: FrameSupportWeightsDispatchInfo }
      >;
      /** An account was reaped. */
      KilledAccount: AugmentedEvent<
        ApiType,
        [account: AccountId32],
        { account: AccountId32 }
      >;
      /** A new account was created. */
      NewAccount: AugmentedEvent<
        ApiType,
        [account: AccountId32],
        { account: AccountId32 }
      >;
      /** On on-chain remark happened. */
      Remarked: AugmentedEvent<
        ApiType,
        [sender: AccountId32, hash_: H256],
        { sender: AccountId32; hash_: H256 }
      >;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    threeVm: {
      /** An author of a module was removed [contract] */
      AuthorRemoved: AugmentedEvent<ApiType, [AccountId32]>;
      /** An author of a module was stored [contract, author] */
      AuthorStored: AugmentedEvent<
        ApiType,
        [ITuple<[AccountId32, AccountId32]>]
      >;
      /** A signal event was bounced beyond the threshold. [step, kind, xtx_id] */
      ExceededBounceThrehold: AugmentedEvent<
        ApiType,
        [ITuple<[u32, T3rnSdkPrimitivesSignalSignalKind, H256]>]
      >;
      /**
       * A module was instantiated from the registry [id, module_author,
       * module_type, module_len]
       */
      ModuleInstantiated: AugmentedEvent<
        ApiType,
        [
          ITuple<
            [H256, AccountId32, T3rnPrimitivesContractMetadataContractType, u32]
          >
        ]
      >;
      /**
       * A signal event was bounced back, because a signal was already sent for
       * the current step. [step, kind, xtx_id]
       */
      SignalBounced: AugmentedEvent<
        ApiType,
        [ITuple<[u32, T3rnSdkPrimitivesSignalSignalKind, H256]>]
      >;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    transactionPayment: {
      /**
       * A transaction fee `actual_fee`, of which `tip` was added to the minimum
       * inclusion fee, has been paid by `who`.
       */
      TransactionFeePaid: AugmentedEvent<
        ApiType,
        [who: AccountId32, actualFee: u128, tip: u128],
        { who: AccountId32; actualFee: u128; tip: u128 }
      >;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    treasury: {
      BeneficiaryTokensIssued: AugmentedEvent<ApiType, [AccountId32, u128]>;
      InflationAllocationChanged: AugmentedEvent<
        ApiType,
        [developer: Perbill, executor: Perbill],
        { developer: Perbill; executor: Perbill }
      >;
      InflationConfigChanged: AugmentedEvent<
        ApiType,
        [
          annualMin: Perbill,
          annualIdeal: Perbill,
          annualMax: Perbill,
          roundMin: Perbill,
          roundIdeal: Perbill,
          roundMax: Perbill
        ],
        {
          annualMin: Perbill;
          annualIdeal: Perbill;
          annualMax: Perbill;
          roundMin: Perbill;
          roundIdeal: Perbill;
          roundMax: Perbill;
        }
      >;
      NewRound: AugmentedEvent<
        ApiType,
        [round: u32, head: u32],
        { round: u32; head: u32 }
      >;
      RewardsClaimed: AugmentedEvent<ApiType, [AccountId32, u128]>;
      RoundTermChanged: AugmentedEvent<
        ApiType,
        [
          old: u32,
          new_: u32,
          roundMin: Perbill,
          roundIdeal: Perbill,
          roundMax: Perbill
        ],
        {
          old: u32;
          new_: u32;
          roundMin: Perbill;
          roundIdeal: Perbill;
          roundMax: Perbill;
        }
      >;
      RoundTokensIssued: AugmentedEvent<ApiType, [u32, u128]>;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    utility: {
      /** Batch of dispatches completed fully with no error. */
      BatchCompleted: AugmentedEvent<ApiType, []>;
      /** Batch of dispatches completed but has errors. */
      BatchCompletedWithErrors: AugmentedEvent<ApiType, []>;
      /**
       * Batch of dispatches did not complete fully. Index of first failing
       * dispatch given, as well as the error.
       */
      BatchInterrupted: AugmentedEvent<
        ApiType,
        [index: u32, error: SpRuntimeDispatchError],
        { index: u32; error: SpRuntimeDispatchError }
      >;
      /** A call was dispatched. */
      DispatchedAs: AugmentedEvent<
        ApiType,
        [result: Result<Null, SpRuntimeDispatchError>],
        { result: Result<Null, SpRuntimeDispatchError> }
      >;
      /** A single item within a Batch of dispatches has completed with no error. */
      ItemCompleted: AugmentedEvent<ApiType, []>;
      /** A single item within a Batch of dispatches has completed with error. */
      ItemFailed: AugmentedEvent<
        ApiType,
        [error: SpRuntimeDispatchError],
        { error: SpRuntimeDispatchError }
      >;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    xbiPortal: {
      AbiInstructionExecuted: AugmentedEvent<ApiType, []>;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
    xdns: {
      /** [requester, xdns_record_id] */
      XdnsRecordPurged: AugmentedEvent<ApiType, [AccountId32, U8aFixed]>;
      /** [xdns_record_id] */
      XdnsRecordStored: AugmentedEvent<ApiType, [U8aFixed]>;
      /** [xdns_record_id] */
      XdnsRecordUpdated: AugmentedEvent<ApiType, [U8aFixed]>;
      /** Generic event */
      [key: string]: AugmentedEvent<ApiType>;
    };
  } // AugmentedEvents
} // declare module
