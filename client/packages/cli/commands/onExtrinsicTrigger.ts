import { ApiPromise } from '@polkadot/api';
import { Sdk } from "@t3rn/sdk";
import "@t3rn/types"
// @ts-ignore
import { Vec, T3rnTypesSideEffect } from "@polkadot/types/lookup"

export const onExtrinsicTrigger = (circuitApi: ApiPromise, sideEffects: any[], sequential: boolean, sender: any, sdk: Sdk) => {
    return {
        sideEffects: circuitApi.createType("Vec<T3rnTypesSideEffect>",
            sideEffects.map(data => {
                const obj: T3rnTypesSideEffect = sdk.gateways[data.target].createTransferSfx({
                    from: sender.toString(),
                    to: data.to,
                    value: sdk.gateways[data.target].floatToBn(data.amount),
                    maxReward: sdk.floatToBn(data.reward), // we need t3rn float encoding here
                    insurance: sdk.floatToBn(data.insurance), // and here
                    nonce: 1,
                })
                return obj
            })
        ),
        fee: circuitApi.createType("u128", 0),
        sequential: circuitApi.createType("bool", sequential),
    }
}