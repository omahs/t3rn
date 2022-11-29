import "@t3rn/types"
// @ts-ignore
import {T3rnTypesSideEffect} from '@polkadot/types/lookup';
import {TextDecoder} from "util"
import {SecurityLevel, SfxStatus, SfxType} from "@t3rn/sdk/dist/src/side-effects/types";
import {Sdk} from "@t3rn/sdk";
import {BehaviorSubject} from "rxjs";
import {Gateway} from "@t3rn/sdk/dist/src/gateways";
import {StrategyEngine} from "../strategy";
import {BiddingEngine} from "../bidding";
import {EventEmitter} from "events";
import {floatToBn} from "@t3rn/sdk/dist/src/circuit";

const BN = require('bn.js')
// maps event names to SfxType enum;
export const EventMapper = ["Transfer", "MultiTransfer"]

export type TxOutput = {
    amount: BigInt,
    amountHuman: number,
    asset: string,
}

export enum NotificationType {
    SubmitBid,
}

export enum TxStatus {
    Pending,
    Ready,
}
export type Notification = {
    type: NotificationType,
    payload: any,
}

export class SideEffect extends EventEmitter {
    step: number;
    status: SfxStatus = SfxStatus.Bidding;
    action: SfxType;
    txStatus: TxStatus = TxStatus.Ready; // used as mutex to prevent concurrent bids
    target: string;
    gateway: Gateway

    securityLevel: SecurityLevel;

    wantToBid: boolean = false;
    circuitSignerAddress: string;
    isBidder: boolean = false;
    minProfitUsd: number = 0;

    relayer: string;

    // SideEffect data
    id: string;
    humanId: string;
    xtxId: string;
    arguments: string[];
    insurance: number;
    reward: BehaviorSubject<number>;
    raw: T3rnTypesSideEffect;

    // TargetConfirmation
    inclusionData: any; // contains encoded payload, inclusionProof, and blockHash
    targetInclusionHeight: number = 0;
    executor: string;

    // Risk/Reward Parameters:
    // this is the tx cost in the native currency of the target
    txCostNative: BehaviorSubject<number>;
    // cost of the native asset in USD. Used for tx cost calculation
    nativeAssetPrice: BehaviorSubject<number>;
    // current cost of the assets that are used for the sfx execution
    txOutputAssetPrice: BehaviorSubject<number>;
    // profit that can be generated by executing this sfx
    maxProfitUsd: BehaviorSubject<number> = new BehaviorSubject<number>(0);
    // price for reward assert in USD
    rewardAssetPrice: BehaviorSubject<number> = new BehaviorSubject<number>(0);

    subscriptions: any[] = [];

    txCostUsd: number = 0;
    txOutputCostUsd: number = 0;
    rewardUsd: number = 0;

    txReceipt: any; // store tx receipt

    strategyEngine: StrategyEngine;
    biddingEngine: BiddingEngine;


    constructor(sideEffect: T3rnTypesSideEffect, id: string, xtxId: string, sdk: Sdk, strategyEngine: StrategyEngine, biddingEngine: BiddingEngine, circuitSignerAddress: string) {
        super();
        if(this.knownTransactionInterface(sideEffect.encodedAction)) {
            this.raw = sideEffect;
            this.id = id;
            this.humanId = id.substring(0, 8)
            this.xtxId = xtxId
            this.arguments = sideEffect.encodedArgs.map(entry => entry.toString());
            this.target =  new TextDecoder().decode(sideEffect.target.toU8a())
            this.gateway = sdk.gateways[this.target]
            this.securityLevel = this.evalSecurityLevel(this.gateway.gatewayType)
            this.reward = new BehaviorSubject(sdk.circuit.toFloat(sideEffect.maxReward)) // this is always in TRN (native asset)
            this.insurance = sdk.circuit.toFloat(sideEffect.insurance) // this is always in TRN (native asset)
            this.strategyEngine = strategyEngine;
            this.biddingEngine = biddingEngine;
            this.circuitSignerAddress = circuitSignerAddress;
        } else {
            console.log("SideEffect interface unknown!!")
        }
    }

    evalSecurityLevel(gatewayType: any): SecurityLevel {
        if (gatewayType.ProgrammableExternal === '0' || gatewayType.OnCircuit === '0') {
            return SecurityLevel.Escrow
        } else {
            return SecurityLevel.Optimistic
        }
    }

    // sets the step of the sideEffect in its execution
    setStep(step: number) {
        this.step = step
    }

    setRiskRewardParameters(
        txCostNative: BehaviorSubject<number>,
        nativeAssetPrice: BehaviorSubject<number>,
        txOutputAssetPrice: BehaviorSubject<number>,
        rewardAssetPrice: BehaviorSubject<number>
    ){
        this.txCostNative = txCostNative;
        this.nativeAssetPrice = nativeAssetPrice;
        this.txOutputAssetPrice = txOutputAssetPrice;
        this.rewardAssetPrice = rewardAssetPrice;

        const txCostNativeSubscription = this.txCostNative.subscribe(() => {
            this.recomputeMaxProfit()
        })

        this.subscriptions.push(txCostNativeSubscription)

        const nativeAssetPriceSubscription = this.nativeAssetPrice.subscribe(() => {
            this.recomputeMaxProfit()
        })

        this.subscriptions.push(nativeAssetPriceSubscription)

        const txOutputAssetPriceSubscription = this.txOutputAssetPrice.subscribe(() => {
            this.recomputeMaxProfit()
        })

        this.subscriptions.push(txOutputAssetPriceSubscription)

        const rewardAssetPriceSubscription = this.rewardAssetPrice.subscribe(() => {
            this.recomputeMaxProfit()
        })

        this.subscriptions.push(rewardAssetPriceSubscription)

        const rewardSubscription = this.reward.subscribe(() => {
            this.recomputeMaxProfit()
        })

        this.subscriptions.push(rewardSubscription)

        this.recomputeMaxProfit();

    }

    // computes the max profit that can be generated by executing this sfx and updates the maxProfitUsd subject
    recomputeMaxProfit() {
        // recomputing profit
        const txCostUsd = this.gateway.toFloat(this.txCostNative.getValue()) * this.nativeAssetPrice.getValue()
        this.txCostUsd = txCostUsd;
        const txOutputCostUsd = this.txOutputAssetPrice.getValue() * this.getTxOutputs().amountHuman
        this.txOutputCostUsd = txOutputCostUsd;
        const rewardValueUsd = this.rewardAssetPrice.getValue() * this.reward.getValue()
        this.rewardUsd = rewardValueUsd;
        const maxProfitUsd = rewardValueUsd - txCostUsd - txOutputCostUsd
        if(maxProfitUsd !== this.maxProfitUsd.getValue()) {
            this.maxProfitUsd.next(maxProfitUsd);
            // for most cases it should be fine to only reevaluate the strategy when the maxProfit changes
            try {
                this.minProfitUsd = this.strategyEngine.evaluateSfx(this) as number;
                this.wantToBid = true;

                if(!this.isBidder && this.wantToBid && this.txStatus === TxStatus.Ready && this.status === SfxStatus.Bidding) {
                    this.txStatus = TxStatus.Pending; //
                    const bidUsd = this.biddingEngine.computeBid(this)
                    const bidRewardAsset = (bidUsd / this.rewardAssetPrice.getValue())
                    this.emit("Notification", {
                        type: NotificationType.SubmitBid,
                        payload: {
                            sfxId: this.id,
                            bidAmount: floatToBn(bidRewardAsset) // converts human to native
                        }
                    })
                } else {
                    console.log("I am already a bidder or I don't want to bid")
                }
            } catch(e) {
                console.log(`Sfx ${this.humanId} eval failed: `, e.toString())
                this.wantToBid = false;
            }
        }
    }

    updateStatus(status: SfxStatus) {
        this.status = status;
    }

    // return an array of arguments to execute on target.
    execute(): any[] | void {
        switch(this.action) {
            case SfxType.Transfer: {
                return this.getTransferArguments()
            }
        }
    }

    // returns the amount that needs to be spent to execute (without fees)
    getTxOutputs(): TxOutput  {
        switch(this.action) {
            case SfxType.Transfer: {
                const amount = this.getTransferArguments()[1];
                return {
                    amount: amount,
                    amountHuman: this.gateway.toFloat(amount), // converts to human format
                    asset: this.gateway.ticker
                }
            }
        }
    }

    // sfx was successfully executed on target and has the inclusion proof data
    executedOnTarget(inclusionData: any, executor: any, targetInclusionHeight: any) {
        this.inclusionData = inclusionData;
        this.executor = executor;
        this.targetInclusionHeight = targetInclusionHeight;
        this.status = SfxStatus.ExecutedOnTarget;
    }

    confirmedOnCircuit() {
        this.status = SfxStatus.Confirmed;

        // unsubscribing from all subjects, as no longer needed
        this.unsubscribe()
    }

    bidAccepted(bidAmount: number) {
        this.isBidder = true;
        this.txStatus = TxStatus.Ready; // open mutex lock
        this.reward.next(this.gateway.toFloat(bidAmount)); // not sure if we want to do this tbh. Reacting to other bids should be sufficient
    }

    bidRejected() {
        this.isBidder = false;
        this.txStatus = TxStatus.Ready;
    }

    processBid(signer: string, bidAmount: number) {
        // if this is not own bid, update reward and isBidder
        if(signer !== this.circuitSignerAddress) {
            this.isBidder = false;
            this.reward.next(this.gateway.toFloat(bidAmount));
            console.log(`Sfx ${this.humanId} received bid from ${signer} for ${bidAmount} ${this.gateway.ticker}`)
        }

        console.log("own bid detected!")
    }

    readyToExecute() {
        this.status = SfxStatus.PendingExecution;
    }

    droppedAtBidding() {
        this.status = SfxStatus.Dropped;
        this.unsubscribe()
    }

    reverted() {
        this.status = SfxStatus.Reverted;
        this.unsubscribe()
    }

    // ensure we can deal with the sfx action and set SfxType
    private knownTransactionInterface(encodedAction: any): boolean {
        switch(encodedAction.toHuman()) {
            case "tran": {
                this.action = SfxType.Transfer
                return true
                break;
            }
            default: {
                return false
            }
        }
    }

    // returns the arguments
    private getTransferArguments(): any[] {
        return [
            this.arguments[1],
            this.gateway.parseLe(this.arguments[2]).toNumber(),
        ]
    }

    private unsubscribe() {
        this.subscriptions.forEach(subscription => {
            subscription.unsubscribe()
        })
    }
}