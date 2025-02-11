import config from "./config/setup"
import { ApiPromise, Keyring } from'@polkadot/api';
import { register } from "./commands/register/register";
import { setOperational } from "./commands/operational";
import {onExtrinsicTrigger} from "./commands/onExtrinsicTrigger";
import * as fs from "fs";
import {submitHeader} from "./commands/submit_header/submit_header";
import { Sdk, converters } from "@t3rn/sdk/dist/src";

import { Command } from 'commander';
import {bid} from "./commands/bid";
const program = new Command();
import { cryptoWaitReady } from '@polkadot/util-crypto';

program
  .name('t3rn CLI')
  .description('CLI for interacting with the t3rn circuit')
  .version('0.1.0');

class CircuitCLI {
    circuit: ApiPromise;
    signer: any;
    sdk: Sdk;

    async setup() {
        await cryptoWaitReady();
        const keyring = new Keyring({ type: "sr25519" })
        this.signer = process.env.CIRCUIT_KEY === undefined
                ? keyring.addFromUri("//Alice")
                : keyring.addFromMnemonic(process.env.CIRCUIT_KEY)

        this.sdk = new Sdk("ws://127.0.0.1:9944", this.signer)
        // @ts-ignore suddenly this is not working
        this.circuit = await this.sdk.init();
    }

    async close() {
        await this.circuit.disconnect()
        process.exit();
    }

    async error() {
        await this.circuit.disconnect()
        process.exit(1);
    }

    async register(id: string, teleport: number, exportArgs: boolean, exportName: string) {
        let data: any = config.gateways.find(elem => elem.id === id)
        if(data) {
            if(data.registrationData?.parachain !== null) {
                // @ts-ignore
                data.relaychainRpc = config.gateways.find(elem => elem.id === data.registrationData.parachain.relayChainId).rpc
            }
            const registrationData: any = await register(this.circuit, data, teleport)

            const tx = this.circuit.tx.portal.registerGateway(
                registrationData[0].url,
                registrationData[0].gateway_id,
                registrationData[0].gateway_abi,
                registrationData[0].gateway_vendor,
                registrationData[0].gateway_type,
                registrationData[0].gateway_genesis,
                registrationData[0].gateway_sys_props,
                registrationData[0].allowed_side_effects,
                registrationData[0].registration_data.toHex()
            );
            let submissionHeight = await this.sdk.circuit.tx.signAndSendSafe(this.sdk.circuit.tx.createSudo(tx))
                .then(() => console.log("Success: Gateway registered!"))
                .catch(err => {
                    console.error("Error: Registration Failed! Err:", err)
                    this.error()
                })

            if (exportArgs) {
                const fileName = `./exports/` + exportName + '.json';
                // @ts-ignore
                await this.exportData(registrationData, fileName, "register", submissionHeight)
            } else {
                this.close()
            }

        } else {
            console.log(`Config for ${process.argv[3]} not found!`)
            this.error();
        }
    }

    async setOperational(id: string, operational: boolean, exportArgs: boolean, exportName: string) {
        const data = config.gateways.find(elem => elem.id === id)
        if (data) {
            const transactionArgs = await setOperational(this.circuit, data, operational)
            const tx = this.circuit.tx.portal.setOperational(transactionArgs?.gatewayId, transactionArgs?.operational);
            let submissionHeight = await this.sdk.circuit.tx.signAndSendSafe(this.sdk.circuit.tx.createSudo(tx))
                .then(() => console.log("Success: Operational status set!"))
                .catch(err => {
                    console.error("Error: setOperational Failed! Err:", err)
                    this.error()
                })

            if (exportArgs) {
                const fileName = `./exports/` + exportName + '.json';
                // @ts-ignore
                await this.exportData([transactionArgs], fileName, "set-operational", submissionHeight)
            } else {
                this.close()
            }

        } else {
            console.log(`Config or argument for ${process.argv[3]} not found!`)
            this.error();
        }
    }

    async submitHeaders(id: string, exportArgs: boolean, exportName: string) {
        const gatewayData = config.gateways.find(elem => elem.id === id)
        if(gatewayData) {
            if(gatewayData.registrationData?.parachain !== null) {
                // @ts-ignore
                gatewayData.relaychainRpc = config.gateways.find(elem => elem.id === gatewayData.registrationData.parachain.relayChainId).rpc
            }
            const transactionArgs: any[] = await submitHeader(this.circuit, gatewayData, id)
            let tx = this.sdk.circuit.tx.createBatch(transactionArgs.map(args => {
                return this.circuit.tx.portal.submitHeaders(
                    args.gatewayId,
                    args.data.toHex() // we submit in encoded form to portal
                )
            }))
            let submissionHeight = await this.sdk.circuit.tx.signAndSendSafe(tx)
                .then(height => {
                    console.log("Success: Header Range submitted!")
                    return height
                })
                .catch(err => {
                    console.error("Error: Header Submission Failed! Err:", err)
                    this.error()
                })

            if (exportArgs) {
                const fileName = `./exports/` + exportName + '.json';
                // @ts-ignore
                await this.exportData(transactionArgs, fileName, "submit-headers", submissionHeight)
            } else {
                this.close()
            }

        } else {
            console.log(`Config for ${process.argv[3]} not found!`)
            this.error();
        }
    }

    async transfer(data: any, sequential: boolean) {
        const gatewayData = config.gateways.find(elem => elem.id === data.target)

        if(gatewayData) {
            if(data.to === '') data.to = gatewayData.transferData.receiver;
            const transactionArgs: any = onExtrinsicTrigger(this.circuit, [data], sequential, this.signer.address, this.sdk)
            const tx = this.circuit.tx.circuit.onExtrinsicTrigger(transactionArgs.sideEffects, 0, false)
            // @ts-ignore
            let submissionHeight = await this.sdk.circuit.tx.signAndSendSafe(tx)
                .then(height => {
                    console.log("Success: Transfer submitted!")
                    return height
                })
                .catch(err => {
                    console.error("Error: Transfer submission failed! Err:", err);
                    this.error()
                })

            if (data.exportArgs) {
                const fileName = `./exports/` + data.exportName + '.json';
                this.exportData([transactionArgs], fileName, "transfer", submissionHeight as string)
            } else {
                this.close()
            }

        } else {
            console.log(`Config or argument for ${process.argv[3]} not found!`)
            this.error();
        }
    }

    async submitSideEffects(path: string, exportArgs: boolean, exportName: string) {
        if (!fs.existsSync(path)) {
             console.log("File doesn't exist!")
             this.error()
         }

        const data = (await import('./' + path)).default;

        // Check we have an config for each SideEffect
        data.sideEffects.forEach(effect => {
            if (!config.gateways.find(entry => entry.id === effect.target)) {
                console.error(`Error: Gateway for SideEffect ${effect.type} not found!`)
                this.error()
            }
        })

        const transactionArgs: any = onExtrinsicTrigger(this.circuit, data.sideEffects, data.sequential, this.signer.address, this.sdk)
        const tx = this.circuit.tx.circuit.onExtrinsicTrigger(transactionArgs.sideEffects, false)

        const submissionHeight = await this.sdk.circuit.tx.signAndSendSafe(tx)
            .then(height => {
                console.log("Success: SideEffects submitted!!")
                return height
            })
            .catch(err => {
                console.error("Error: SideEffects submission failed! Err:", err);
                this.error()
            })

        if (exportArgs) {
            const fileName = `./exports/` + exportName + '.json';
            this.exportData([transactionArgs], fileName, "transfer", submissionHeight as string)
        } else {
            this.close()
        }
    }

    async bid(data: any, exportArgs: boolean, exportName: string) {
        const transactionArgs: any = bid(this.circuit, data, this.sdk);

        const tx = this.circuit.tx.circuit.bidSfx(transactionArgs.sfxId, transactionArgs.bidAmount)
        // @ts-ignore
        let submissionHeight = await this.sdk.circuit.tx.signAndSendSafe(tx)
            .then(height => {
                console.log("Success: Bid submitted!")
                return height
            })
            .catch(err => {
                console.error("Error: Bidding Failed! Err:", err);
                this.error()
            })

        if (exportArgs) {
            const fileName = `./exports/` + exportName + '.json';
            this.exportData([transactionArgs], fileName, "transfer", submissionHeight as string)
        } else {
            this.close()
        }
    }

    exportData(data: any, fileName: string, transactionType: string, submissionHeight: string) {
        let deepCopy;
        // since its pass-by-reference
        if(Array.isArray(data)) {
            deepCopy = [...data];
        } else {
            deepCopy = {...data};
        }

        let encoded = converters.utils.encodeExport(deepCopy, transactionType, submissionHeight as string);
        fs.writeFile(fileName, JSON.stringify(encoded, null, 4), (err) => {
            if(err) {
                console.error("Error: Failed to export data! Err:", err);
                this.error();
            } else {
                console.log("JSON saved to " + fileName);
                this.close();
            }
        });
    }
}

program.command('register')
      .description('Register a gateway on the t3rn blockchain')
      .argument('gateway_id <string>', 'gateway_id as specified in setup.ts')
      .option('-t, --teleport <number>', 'how many epochs the registration should go back.', "0")
      .option('-e, --export', 'export the transaction arguments as JSON', false)
      .option('-o, --output <string>', 'specify the filename of the export', "export")
      .action(async (id, options) => {
          let cli = new CircuitCLI();
          await cli.setup()
          cli.register(id, parseInt(options.teleport), options.export, options.output)
      });

program.command('set-operational')
      .description('Activate/deactivate a gateway')
      .argument('gateway_id <string>', 'gateway_id as specified in setup.ts')
      .argument('operational <bool>', 'gateway_id as specified in setup.ts')
      .option('-e, --export', 'export the transaction arguments as JSON', false)
      .option('-o, --output <string>', 'specify the filename of the export', "export")
      .action(async (id, operational, options) => {
          let cli = new CircuitCLI();
          await cli.setup()
          operational = operational === "true" ? true : false;
          cli.setOperational(id, operational, options.export, options.output)
      });

program.command('submit-headers')
      .description('Submit the latest headers of a gateway to portal. All available finalized headers will be added.')
      .argument('gateway_id <string>', 'gateway_id as specified in setup.ts')
      .option('-e, --export', 'export the transaction arguments as JSON', false)
      .option('-o, --output <string>', 'specify the filename of the export', "export")
      .action(async (id, options) => {
          let cli = new CircuitCLI();
          await cli.setup()
          cli.submitHeaders(id, options.export, options.output)
      });

program.command('transfer')
      .description('Triggers a transfer SideEffect, sending the targets nativ asset')
      .argument('gateway_id <string>', 'gateway_id as specified in setup.ts')
      .option('-t --to <string>', 'receiver address', '')
      .option('-a --amount <flaot>', 'The Amount to send in target native asset', '0.01')
      .option('-r --reward <float>', 'Reward paid for execution', '1')
      .option('-i --insurance <float>', 'Insurance required for execution', '1')
      .option('-e, --export', 'export the transaction arguments as JSON', false)
      .option('-o, --output <string>', 'specify the filename of the export', "export")
      .action(async (id, options) => {
          let cli = new CircuitCLI();
          await cli.setup()
          options.target = id
          options.type = "tran"
          cli.transfer(options, false)
      });

program.command('submit-side-effects')
      .description('Submits SideEffects based on input file')
      .argument('path <string>', 'path to file')
      .option('-e, --export', 'export the transaction arguments as JSON', false)
      .option('-o, --output <string>', 'specify the filename of the export', "export")
      .action(async (path, options) => {
          let cli = new CircuitCLI();
          await cli.setup()
          cli.submitSideEffects(path, options.export, options.output)
      });

program.command('bid')
      .description('Bid on an execution as an Executor')
      .argument('sfxId <string>', 'sfxId of the side effect to bid on')
      .argument('amount <float>', 'bid amount')
      .option('-e, --export', 'export the transaction arguments as JSON', false)
      .option('-o, --output <string>', 'specify the filename of the export', "export")
      .action(async (sfxId, amount, options) => {
          let cli = new CircuitCLI();
          await cli.setup()
          cli.bid({sfxId, amount}, options.export, options.output)
      });

program.parse();