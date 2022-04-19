import { EventEmitter } from 'events'
import { ApiPromise, WsProvider } from '@polkadot/api'
import { Header } from '@polkadot/types/interfaces'
import { grandpaDecode } from './util'
import createDebug from 'debug'
import 'dotenv/config'

export default class Listener extends EventEmitter {
  static debug = createDebug('listener')

  kusama: ApiPromise
  kusamaEndpoint: string = process.env.KUSAMA_RPC as string
  rangeSize: number = Number(process.env.RANGE_SIZE)
  gatewayId: Buffer = Buffer.from(process.env.GATEWAY_ID as string, 'utf8')
  headers: Header[] = []
  // offset in this.headers for the current range batch
  offset: number = 0
  // last known grandpa set id
  grandpaSetId: number = 0
  // last emitted anchor
  anchor: number = 0
  // bike shed mutex
  busy: boolean = false
  unsubNewHeads: () => void

  async init() {
    this.kusama = await ApiPromise.create({
      provider: new WsProvider(this.kusamaEndpoint),
    })

    this.unsubNewHeads = await this.kusama.derive.chain.subscribeNewHeads(
      async header => {
        await this.handleGrandpaSet()

        await this.handleHeader(header)

        if (!this.busy && this.headers.length - this.offset >= this.rangeSize) {
          await this.concludeRange()
        }
      }
    )
  }

  async handleGrandpaSet() {
    const currentSetId = await this.kusama.query.grandpa
      .currentSetId()
      .then(id => Number(id.toJSON()))

    if (this.grandpaSetId !== 0 && currentSetId !== this.grandpaSetId) {
      Listener.debug('grandpa set change', this.grandpaSetId, currentSetId)
      await this.concludeRange()
    }

    this.grandpaSetId = currentSetId
  }

  async handleHeader(header: Header) {
    if (
      this.headers.length === 0 ||
      this.headers[this.headers.length - 1].number.toNumber() + 1 ===
        header.number.toNumber()
    ) {
      this.headers.push(header)
      Listener.debug(`#${header.number.toNumber()}`)
    }
  }

  async concludeRange() {
    this.busy = true
    Listener.debug('concluding range...')
    const unsubJustifications =
      await this.kusama.rpc.grandpa.subscribeJustifications(
        async justification => {
          unsubJustifications()

          const { blockNumber } = await grandpaDecode(justification)

          Listener.debug('decoded block number', blockNumber)

          const justifiedHeaderIndex = this.headers.findIndex(
            h => h.number.toNumber() === blockNumber
          )

          if (justifiedHeaderIndex + 1 > this.offset) {
            const reversedRange = this.headers
              .slice(this.offset, justifiedHeaderIndex + 1)
              .reverse()

            this.offset = justifiedHeaderIndex + 1

            const anchor = reversedRange.shift() as Header

            Listener.debug(
              'anchor',
              anchor.number.toNumber(),
              'reversed range',
              reversedRange.map(h => h.number.toNumber()),
              'range size',
              reversedRange.length
            )

            this.emit(
              'range',
              this.gatewayId,
              anchor,
              reversedRange,
              justification
            )
          }

          this.busy = false
        }
      )
  }

  kill() {
    Listener.debug('kill')
    this.unsubNewHeads()
  }
}
