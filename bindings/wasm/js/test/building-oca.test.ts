import { expect } from 'chai'
import { Encoding, OCA } from 'oca-rust-wasm'

describe('Plain OCA is built', () => {
  const oca = (new OCA(Encoding.Utf8)).finalize()

  it('return OCA as JS object', () => {
    expect(oca).to.haveOwnProperty("capture_base")
    expect(oca).to.have.nested.property("capture_base.type")
    expect(oca).to.have.nested.property("capture_base.classification")
    expect(oca).to.have.nested.property("capture_base.attributes")
    expect(oca).to.have.nested.property("capture_base.pii")
    expect(oca).to.haveOwnProperty("overlays")

    expect(oca.capture_base.attributes).to.be.an('object').that.is.empty
    expect(oca.capture_base.pii).to.be.an('array').that.is.empty
    expect(oca.overlays).to.be.an('array').lengthOf(1)
  })
})
