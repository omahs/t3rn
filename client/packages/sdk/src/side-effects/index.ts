// @ts-ignore
import {T3rnPrimitivesXdnsXdnsRecord, T3rnTypesSideEffect, u128} from "@polkadot/types/lookup"
import * as BN from 'bn.js'

export const createSfx = (
	args: {
		target: number[],
		signature: string | undefined,
		nonce: number,
		enforceExecutioner: string | undefined,
		maxReward: BN,
		insurance: BN,
		encodedArgs: string[],
		encodedAction: string
	}
): T3rnTypesSideEffect => {
	const sfx: T3rnTypesSideEffect = {
		target: args.target,
		maxReward: args.maxReward,
		insurance: args.insurance,
		encodedAction: args.encodedAction,
		encodedArgs: args.encodedArgs,
		signature: args.signature,
		nonce: args.nonce,
		enforceExecutioner: args.enforceExecutioner,
	}
	return sfx
}