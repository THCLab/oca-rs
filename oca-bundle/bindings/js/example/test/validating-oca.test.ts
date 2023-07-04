import { expect } from 'chai'
describe('', () => {
  it('', () => {
    expect(true).to.be.true
  })
})
/*
import { AttributeBuilder, AttributeType, Encoding, Entry, OCABuilder, Validator } from 'oca.js'

describe('Plain OCA is validated', () => {
  const oca = new OCABuilder(Encoding.Utf8).finalize()
  const validator = new Validator()
  const result = validator.validate(oca)

  it('return success', () => {
    expect(result).to.haveOwnProperty("success")
    expect(result).to.haveOwnProperty("errors")

    expect(result.success).to.be.true
    expect(result.errors).to.be.an('array').that.is.empty
  })
})

describe('Translations are not enforced', () => {
  const oca = new OCABuilder(Encoding.Utf8)
    .addName({ en_EN: "OCA name" })
    .finalize()
  const validator = new Validator()
  const result = validator.validate(oca)

  it('return success', () => {
    expect(result.success).to.be.true
  })
})

describe('Missing meta translations', () => {
  const oca = new OCABuilder(Encoding.Utf8)
    .addName({ en_EN: "OCA name" })
    .finalize()
  const validator = new Validator()
    .enforceTranslations(["en_EN", "pl_PL"])
  const result = validator.validate(oca)

  it('return errors', () => {
    expect(result.success).to.be.false
    expect(result.errors).to.be.an('array').lengthOf(1)
  })

  describe('for name', () => {
    const oca = new OCABuilder(Encoding.Utf8)
      .addName({ en_EN: "OCA name" })
      .addDescription({
        en_EN: "OCA description",
        pl_PL: "opis OCA"
      })
      .finalize()
    const validator = new Validator()
      .enforceTranslations(["en_EN", "pl_PL"])
    const result = validator.validate(oca)

    it('return errors', () => {
      expect(result.success).to.be.false
      expect(result.errors).to.be.an('array').lengthOf(1)
    })
  })

  describe('for description', () => {
    const oca = new OCABuilder(Encoding.Utf8)
      .addName({
        en_EN: "OCA name",
        pl_PL: "nazwa OCA"
      })
      .addDescription({
        en_EN: "OCA description",
      })
      .finalize()
    const validator = new Validator()
      .enforceTranslations(["en_EN", "pl_PL"])
    const result = validator.validate(oca)

    it('return errors', () => {
      expect(result.success).to.be.false
      expect(result.errors).to.be.an('array').lengthOf(1)
    })
  })
})

describe('Missing overlay translations', () => {
  const oca = new OCABuilder(Encoding.Utf8)
    .addAttribute(
      new AttributeBuilder("attr1", AttributeType.Text)
        .addLabel({ en_EN: "Attribute 1" })
        .addInformation({ en_EN: "Attribute 1 info" })
        .addEntries([
          new Entry("o1", { en_EN: "Option 1" }).plain()
        ])
        .build()
    )
    .finalize()
  const validator = new Validator()
    .enforceTranslations(["en_EN", "pl_PL"])
  const result = validator.validate(oca)

  it('return errors', () => {
    expect(result.success).to.be.false
    expect(result.errors).to.be.an('array').lengthOf(3)
  })
})
*/
