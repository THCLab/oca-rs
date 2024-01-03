import { expect } from 'chai'
import { Attribute, AttributeType, create_nested_attr_type_from_js, OCABox, OCABundle, Validator } from 'oca.js'

describe('Plain OCA', () => {
  const oca = new OCABox()
    .addClassification("GICS:35102020")
    .generateBundle()
  const validator = new Validator()
  const result = validator.validate(oca)

  it('is valid', () => {
    expect(result).to.haveOwnProperty("success")
    expect(result).to.haveOwnProperty("errors")

    expect(result.success).to.be.true
    expect(result.errors).to.be.an('array').that.is.empty
  })
})

describe('Validator without enforced translations', () => {
  const oca = new OCABox()
    .addClassification("GICS:35102020")
    .addMeta("name", {
      eng: "OCA name"
    })
    .generateBundle()
  const validator = new Validator()
  const result = validator.validate(oca)

  it('passes validation', () => {
    expect(result.success).to.be.true
  })
})

describe('Missing enforced translation', () => {
  const oca = new OCABox()
    .addClassification("GICS:35102020")
    .addMeta("name", {
      eng: "OCA name"
    })
    .generateBundle()

  const validator = new Validator()
    .enforceTranslations(["eng", "pol"])
  const result = validator.validate(oca)

  it('is not valid', () => {
    expect(result.success).to.be.false
    expect(result.errors).to.be.an('array').lengthOf(1)
  })
})

describe('Missing overlay translation', () => {
  const textType = create_nested_attr_type_from_js("Text");
  const oca = new OCABox()
    .addAttribute(
      new Attribute("attr1")
        .setAttributeType(textType)
        .setLabel({ eng: "Attribute 1" })
        .setInformation({ eng: "Attribute 1 info" })
        .setEntries({
          o1: { eng: "Option 1" }
        })
    )
    .generateBundle()

  const validator = new Validator()
    .enforceTranslations(["eng", "pol"])
  const result = validator.validate(oca)

  it('is not valid', () => {
    expect(result.success).to.be.false
    expect(result.errors).to.be.an('array').lengthOf(3)
  })
})

describe('Malformed OCA Bundle', () => {
  const said = 'EKwwHUyIW5NOuVi2zo6fyibkdJmFUoAoO-tJbKKeOuMb'
  const oca = {
    v: 'OCAB10JSON000106_',
    d: said,
    capture_base: {
      d: said,
      type: 'spec/capture_base/1.0',
      classification: 'GICS:35102020',
      attributes: {},
      flagged_attributes: []
    },
    overlays: {}
  } as OCABundle

  const validator = new Validator()
  const result = validator.validate(oca)

  it('is not valid', () => {
    expect(result.success).to.be.false
    expect(result.errors).to.be.an('array').lengthOf(1)
  })
})
