---
sidebar_position: 1
---

# What is a Sideeffect

A Sideeffect is the description of a transaction that needs to be executed on an external consensus system. When initialised, the creator of the Sideeffect (can be a user or smart contract) sets all the neccessary attribtues and commits it to the Circuit, where Executors are able to lacate them.


## Sideeffect structure
```
pub struct SideEffect<AccountId, BalanceOf> {
    pub target: TargetId,
    pub max_reward: BalanceOf,
    pub insurance: BalanceOf,
    pub encoded_action: Bytes,
    pub encoded_args: Vec<Bytes>,
    pub signature: Bytes,
    pub enforce_executor: Option<AccountId>,
}
```

#### target:
The target describes the `destination` consensus system the Sideeffect should be executed on. 

#### max_reward 
The max_reward sets the reward for the Executor in `TRN`.

#### encoded_action
The encoded_action sets the action that should be done on the target chain e.g.: call `function x` on `contract y`.

#### encoded_args
The encoded_args sets the arguments that should be passed in `function_y`, when calling a function on a `contract_x` e.g.: contract_x.function_y(`args`).

#### signature
The signature attribute holds the `signature` of the creator of the Sideeffect.

#### enforce_executor
The enforce_executor attributer lets the user set the executor that should execute the Sideeffect.