import { expect } from 'chai'
// import util from 'util'
import { Attribute, AttributeType, OCABox, Encoding, Validator } from 'oca.js'

describe('Plain OCA is built', () => {
  const oca = new OCABox()
    .addClassification("GICS:35102020")
    .generateBundle()
  // console.log(oca)

  it('return OCA as JS object', () => {
    expect(oca).to.haveOwnProperty("d")
    expect(oca).to.haveOwnProperty("capture_base")
    expect(oca).to.have.nested.property("capture_base.type")
    expect(oca).to.have.nested.property("capture_base.d")
    expect(oca).to.have.nested.property("capture_base.classification")
    expect(oca).to.have.nested.property("capture_base.attributes")
    expect(oca).to.have.nested.property("capture_base.flagged_attributes")
    expect(oca).to.haveOwnProperty("overlays")

    expect(oca.capture_base.attributes).to.be.an('object').that.is.empty
    expect(oca.capture_base.classification).to.eq("GICS:35102020")
    expect(oca.capture_base.flagged_attributes).to.be.an('array').that.is.empty
  })
})

describe('OCA with attributes is built', () => {
  const oca = new OCABox()
    .addMeta("name", {
      eng: "Driving Licence",
      pol: "Prawo Jazdy"
    })
    .addMeta("description", {
      eng: "DL desc",
      pol: "PJ desc"
    })
    .addAttribute(
      new Attribute("attr_name")
      .setAttributeType(AttributeType.Numeric)
      .setFlagged()
      .setLabel({
        eng: "Name: ",
        pol: "Imię: "
      })
      .setInformation({
        eng: "en info",
        pol: "pl info"
      })
      .setEntries({
        o1: {
          eng: "option 1",
          pol: "opcja 1"
        },
        o2: {
          eng: "option 2",
          pol: "opcja 2"
        }
      })
    )
    .addAttribute(
      new Attribute("attr2")
      .setAttributeType(AttributeType.DateTime)
      .setLabel({
        eng: "Date: ",
        pol: "Data: "
      })
      .setEncoding(Encoding.Iso8859_1)
      .setFormat("DD.MM.YYYY")
    )
    .addAttribute(
      new Attribute("attr3")
      .setAttributeType(AttributeType.Reference)
      .setSai("sai")
      .setLabel({
        eng: "Reference: ",
        pol: "Referecja: "
      })
    )
    .generateBundle()


  const validator = new Validator().enforceTranslations(["eng", "pol"])
  const r = validator.validate(oca)

  // console.log(util.inspect(oca, false, null, true /* enable colors */))
  // console.log(r)

  describe("Capture Base", () => {
    const captureBase = oca.capture_base

    it('attributes properly added', () => {
      expect(captureBase.attributes).to.have.keys("attr_name", "attr2", "attr3")
      expect(captureBase.attributes).to.have.property("attr_name", "Numeric")
      expect(captureBase.attributes).to.have.property("attr2", "DateTime")
      expect(captureBase.attributes).to.have.property("attr3", "Reference:sai")
      expect(captureBase.flagged_attributes).to.eql(["attr_name"])
    })
  })
})
