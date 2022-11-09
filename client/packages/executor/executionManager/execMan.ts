import createDebug from "debug"
import {Execution} from "./execution";
import {SideEffect} from "./sideEffect";
import Estimator from "../gateways/substrate/estimator";
import SubstrateRelayer from "../gateways/substrate/relayer";
import {PriceEngine} from "../pricing";
import {BehaviorSubject} from "rxjs";


// A type used for storing the different SideEffects throughout their respective life-cycle.
// Please note that waitingForInsurance and readyToExecute are only used to track the progress. The actual logic is handeled in the execution
type Queue = {
    gateways: {
        blockHeight: number
		isBidding: string[],
		isExecuting: string[],
        // Executed sfx and their respective execution block.
        isConfirming: {
            [block: number]: string[]
        },
		complete: string[]
    }
}


export class ExecutionManager {
	static debug = createDebug("execution-manager")

	// we map the current state in the queue
	queue: Queue = <Queue>{}
    // maps xtxId to Execution instance
    xtx: {
        [id: string]: Execution
    } = {}
    // a lookup mapping, to find a sfx xtxId
    sfxToXtx: {
        [sfxId: string]: string
    } = {};

	targetEstimator: {
        [id: string]: Estimator
    } = {};

	priceEngine: PriceEngine;

	constructor(priceEngine: PriceEngine) {
		this.priceEngine = priceEngine;
	}


	// adds gateways on startup
    addGateway(id: string, estimator: Estimator) {
        this.queue[id] = {
            blockHeight: 0,
            waitingForInsurance: [],
            readyToExecute: [],
            readyToConfirm: {},
        }

		this.targetEstimator[id] = estimator;
    }

	async addXtx(xtx: Execution) {
		this.xtx[xtx.id] = xtx
		let sfxId = Object.keys(xtx.sideEffects)
		console.log("sfxIds", sfxId)
		for(let i = 0; i < sfxId.length; i++) {
			const sfx = xtx.sideEffects[sfxId[i]];
			this.sfxToXtx[sfxId[i]] = xtx.id
			await this.addRiskRewardParameters(sfx)
		}
	}

	async addRiskRewardParameters(sfx: SideEffect) {
		const txCostSubject = await this.targetEstimator[sfx.target].getNativeTxCostSubject(sfx);
		const nativeAssetPriceSubject = this.priceEngine.getAssetPrice(sfx.gateway.ticker);

		const txOutput = sfx.getTxOutputs()
		const txOutputPriceSubject = this.priceEngine.getAssetPrice(txOutput.asset);
		const rewardAssetPriceSubject = this.priceEngine.getAssetPrice("TRN");

		sfx.setRiskRewardParameters(txCostSubject, nativeAssetPriceSubject, txOutputPriceSubject, rewardAssetPriceSubject)
	}



}