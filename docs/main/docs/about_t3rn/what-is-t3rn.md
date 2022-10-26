---
sidebar_position: 1
---

# What is t3rn

T3rn is a smart contract hosting platform for smart contracts that have the ability to perform multichain transactions.

Developers can write their smart contracts in Solidity, !ink or languages compiling to WASM, since t3rn supports WASM and EVM.

They also have the ability to store smart contracts in the contracts registry and make them available for everyone, so other developers can instantiate these contracts in their contracts and collaborate with them.
Creators of contracts can set a fixed fee for the use of the contract,so they will be remunerated each time someone executes it.

T3rn uses a special model to perform multichain transactions. Essential components for this are Circuit, Executor and Ranger.


## Interoperability
When a multichain smart contract is executed, it creates transactions on other blockchains, called side effects.

Side Effects are operations that contain parameters for a transaction on a target blockchain. The requester of a side effect specifies the fee he wants to pay for the execution and submits it to the Circuit. 

Side Effects are stored on Circuit and can then be picked up and executed by an Executor.
These side effects can be combined with others to execute a sequence of multichain transactions. This enables composition of multichain transactions.

In most cases, each individual transaction must be successful in order to retain the logic of the composite transaction.

That's why the Circuit keeps updating the state of each atomic transaction of the composite transactions and revert the entire composed transaction in case of failure of one or multiple transactions. 
That means that smart contracts executed on t3rn are fail-safe and reversible.


## Executors
Executors are individual accounts on different blockchains that have available liquidity. They check the Circuit for side effects and depending on the commission they execute the requested transaction on the target blockchain. 

Executors are software that is customizable. They can find multiple ways to perform transactions to increase their profit. As an example, they could aggregate multiple ways to execute a swap and find the cheapest one to execute. 

When the Executor has successfully executed the requested transaction, they submit an Inclusion Proof to the Circuit. 

With the help of the Ranger, the Circuit receives the current block headers of the blockchains plugged into t3rn and can verify the inclusion proof.

## Ranger
The ranger listens to all plugged in blockchains and sends the finalized blocks to the Circuit. 
Now the Circuit can check the submitted inclusion proof with the light clients customized for each blockchain.
Light clients are components of the Circuit that hold the logic of each plugged in Blockchain and help check the validity of and inclusion proof.



