import { ApiPromise } from "@polkadot/api";
// @ts-ignore
import {T3rnPrimitivesXdnsXdnsRecord} from "@polkadot/types/lookup"
import {Gateway} from "./gateway";

export enum GatewayType {
	Substrate,
	Evm
}

export const initGateways = async (api: ApiPromise) => {
	// @ts-ignore
	const records = (await api.rpc.xdns.fetchRecords())['xdns_records'];

	let res = {}

	for(let i = 0; i < records.length; i++) {
		const gateway = new Gateway(records[i])
		res[gateway.id] = gateway
	}
	return res
}

export{Gateway}