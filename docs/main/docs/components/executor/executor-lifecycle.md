---
sidebar_position: 3
---

# Execution Lifecycle

An execution goes through 3 different stages

### Bidding Stage:
When an execution is created the user sets a maximum reward it's willing to pay for each side effect of the execution. As it's difficult to estimate a fair reward for the executors, t3rn has implemented a simple bidding mechanism. Once a user has created an execution, executors can bid on the different side effects they're interested in executing. To bid on a side effect, executors must submit a lower reward amount compared to the latest bid. When submitting the bid, the insurance deposit is automatically deducted from their balance. If an executor bids on an optimistic side effect, the current reward sum of all other optimistic side effects in the execution is also deducted as a slashable bond. 

At the end of the bidding stage, the executor with the lowest bid wins the side effect. This means the executor is obliged to execute the side effect during the execution stage. Executors without a winning bid do not have any obligations and have received a refund for the funds they submitted.

If not all side effects have successfully found an executor, the execution is reverted and all involved parties are refunded. 

### Execution Stage:
Once the bidding stage is completed, the execution stage begins. The winning executors now need to execute the side effects they have bid on. This could for example be an escrowed transfer, in which case the executor would deposit the funds to transfer into the escrow contract. Next, the executor creates an inclusion proof for the side effect. Once the targets on-chain light client has reached the block height in which the side effect was executed, the executor can submit the proof, verifying that the execution was completed correctly. 

It must be noted that the execution stage can be separated into a number of execution phases. When the side effect is created, its side effects are grouped by the following rules:
1. All escrowed side effects are grouped into the first execution phase
2. All optimistic side effects are grouped into the last execution phase, only beginning execution after the first phase has all side effects confirmed

Each stage can only begin execution (or more the confirmation on circuit) after the previous stage has confirmed all side effects. Within an execution phase, the order of confirmation does not matter. 

There is always a risk that an executor doesn't confirm the execution it has bid on, even though the insurance deposit has been paid. For this situation, a re-execution is triggered. The non-confirming executor gets its deposit slashed, which is then added to the reward of the unconfirmed side effect. This increases the profit for executing the side effect, making it more attractive to be picked up by another executor. This reduces user frustration in case the situation arises and makes a revert more unlikely, which is beneficial for all executors. 

Once all side effects are confirmed, the execution stage terminates with `COMMIT`. This means the execution was completed, which triggers the payment for the executors. The reward for the side effect they executed is not claimable by them. 

If not all side effects are confirmed by the time the timeout limit has been reached, the executions stage terminates with `REVERT` , which triggers the refunding of the executors and user. For escrow side effects the step is simple, as only the user is refunded in the execution stage. The executor will receive its funds from the escrow contract in the finality stage. For optimistic side effects, the non-confirming executors are slashed, paying for the rewards of the executors that executed correctly. 

### Finalization Stage: 
The finalization stage is only relevant for escrowed side effects, as their funds need to be unlocked on the target. This could either be a refund for the executor or sending the funds to the receiver. 

To unlock funds from the escrow contract, we need to bridge the outcome of an execution from the circuit to the target in a trustless fashion. To achieve this, t3rn relies on a type of network participant called attesters. These require a high amount of stake on the t3rn blockchain, enabling them to sign the outcome of the escrow transaction of the current batch. In the beginning, we plan to activate around 30-35 attesters, selected by stake amount, requiring a >2/3 majority. Going forward we want to increase this set further and further eventually utilize zkSNARKS to activate hundreds of attesters, making the bridge resilient to collusion attacks. The details of the implementation are not yet finalized but will be provided once this is the case.

These unlock transactions are batched per target and then executed by executors as well. This reduces the fees required for unlocking significantly. When the user creates an execution, every escrow side effect is charged a finality fee, which is then used to refund the executor for submitting the unlock batches. The user pays this also in case of reverts, so the executors don't face any additional cost. 