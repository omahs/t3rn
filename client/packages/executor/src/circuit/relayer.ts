import { EventEmitter } from "events"
import { ApiPromise, Keyring, WsProvider } from "@polkadot/api"
import { fetchNonce } from "../utils/"
import { SideEffect } from "../executionManager/sideEffect"
import createDebug from "debug"
import { BN } from "@polkadot/util"
import { Sdk } from "@t3rn/sdk"
import { SubmittableExtrinsic } from "@polkadot/api/promise/types"
const fs = require("fs")

/** Class responsible for submitting any type of transaction to the circuit.
 * All communication with the circuit is done through the circuit relayer.
 *
 */
export class CircuitRelayer extends EventEmitter {
    static debug = createDebug("circuit-relayer")

    api: ApiPromise
    sdk: Sdk
    id: string
    rpc: string
    signer: any

    constructor(sdk: Sdk) {
        super()
        // @ts-ignore
        this.api = sdk.client
        this.sdk = sdk
        const keyring = new Keyring({ type: "sr25519" })
        this.signer = keyring.addFromUri("//Executor//default")
    }

    /** Builds and submits a sfxBid to the circuit
     *
     * @param sfxId the bid is for
     * @param amount the bidding amount, as integer in the reward asset
     */
    async bidSfx(sfxId: string, amount: BN): Promise<string> {
        const encodedSfxId = this.api.createType("Hash", sfxId)
        const encodedAmount = this.api.createType("u128", amount)
        const tx = this.api.tx.circuit.bidSfx(encodedSfxId, encodedAmount)
        return this.sdk.circuit.tx.signAndSendSafe(tx)
    }

    /** Builds and submits a SFX confirmation tx to the circuit.
     * These confirmations are submitted as TX batch
     *
     * @param sfxs array of SideEffect objects that should be confirmed
     * @returns the block height of the included tx
     */
    async confirmSideEffects(sfxs: SideEffect[]): Promise<string> {
        const txs: SubmittableExtrinsic[] = sfxs.map((sfx) => this.createConfirmTx(sfx))
        const nonce = await fetchNonce(this.api, this.signer.address)
        if (txs.length > 1) {
            // only batch if more than one tx
            return this.sdk.circuit.tx.signAndSendSafe(this.sdk.circuit.tx.createBatch(txs))
        } else {
            return this.sdk.circuit.tx.signAndSendSafe(txs[0])
        }
    }

    /** Builds the actual confirm tx for a given SideEffect
     *
     * @param sfx the SideEffect to confirm
     */
    createConfirmTx(sfx: SideEffect): SubmittableExtrinsic {
        const inclusionData = this.api.createType("InclusionData", sfx.inclusionData)
        const receivedAt = this.api.createType("BlockNumber", 0) // ToDo figure out what to do here

        const confirmedSideEffect = this.api.createType("ConfirmedSideEffect", {
            err: null,
            output: null,
            inclusion_data: inclusionData.toHex(),
            executioner: sfx.executor,
            receivedAt: receivedAt,
            cost: null,
        })
        return this.api.tx.circuit.confirmSideEffect(sfx.id, confirmedSideEffect.toJSON())
    }
}

// in combination with transfer.ts
let indexes = [7, 8, 9, 10, 12, 13, 15, 16, 18, 21, 9999, 111111, 222222, 33333, 444444]
let counter = 0
export const exportData = (data: any, fileName: string, transactionType: string) => {
    let deepCopy
    // since its pass-by-reference
    if (Array.isArray(data)) {
        deepCopy = [...data]
    } else {
        deepCopy = { ...data }
    }
    let encoded = encodeExport(deepCopy, transactionType)
    fs.writeFile("exports/" + indexes[counter] + "-" + fileName, JSON.stringify(encoded, null, 4), (err) => {
        if (err) {
            console.log("Err", err)
        } else {
        }
    })

    counter += 1
}

// encodes data for exporting. We export in encoded and human format.
// Encoded: We use for seeding protal rust tests
// Human: Debugging those tests and viewing data
export const encodeExport = (data: any, transactionType: string) => {
    if (Array.isArray(data)) {
        return data.map((entry) => iterateEncode(entry, transactionType))
    } else {
        return iterateEncode(data, transactionType)
    }
}

const iterateEncode = (data: any, transactionType: string) => {
    let keys = Object.keys(data)
    let result = {}
    if (keys.includes("initialU8aLength")) {
        // this is a polkadot/apiPromise object
        return {
            data: data.toHuman(),
            transaction_type: transactionType,
            encoded_data: data.toHex().substring(2),
        }
    } else {
        for (let i = 0; i < keys.length; i++) {
            result["encoded_" + toSnakeCase(keys[i])] = data[keys[i]].toHex().substring(2)
            result[toSnakeCase(keys[i])] = data[keys[i]].toHuman()
        }
        result["transaction_type"] = transactionType
        result["submission_height"] = 0 // we ignore it here for now
        return result
    }
}

const toSnakeCase = (str) =>
    str &&
    str
        .match(/[A-Z]{2,}(?=[A-Z][a-z]+[0-9]*|\b)|[A-Z]?[a-z]+[0-9]*|[A-Z]|[0-9]+/g)
        .map((x) => x.toLowerCase())
        .join("_")
