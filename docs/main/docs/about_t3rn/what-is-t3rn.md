---
sidebar_position: 1
---

# What is t3rn

t3rn is a hosting platform for smart contracts that have the ability to perform multichain transactions.
Developers can write their smart contracts in Solidity, !ink or languages compiling to WASM, since t3rn supports WASM and EVM.

They also have the ability to store smart contracts in the contracts registry and make them available for everyone, so other developers can instantiate these contracts in their contracts and collaborate with them.
Creators of contracts can set a fixed fee for the use of the contract,so they will be remunerated each time someone executes it.

T3rn uses a special model to perform multichain transactions. Essential components for this are Circuit, Executor and Ranger.


<p align="center">
    <img height="150" src="/img/t3rn-overview.png?raw=true"/>
</p>


## Interoperability
When a multichain smart contract is executed, it creates transactions on other blockchains, called [Sideeffects](components/sfx/sfx-overview).

Side Effects are operations that contain parameters for a transaction on a target blockchain. The requester of a side effect specifies the fee he wants to pay for the execution and submits it to the Circuit. 

They are stored on Circuit and can then be picked up and executed by an Executor.
These Sideeffects can be combined with others to execute a sequence of multichain transactions. This enables composition of multichain transactions.

In most cases, each individual transaction must be successful in order to retain the logic of the composite transaction.

That's why the Circuit keeps updating the state of each atomic transaction of the composite transactions and revert the entire composed transaction in case of failure of one or multiple transactions. 
That means that smart contracts executed on t3rn are fail-safe and reversible.




