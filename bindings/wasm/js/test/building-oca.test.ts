import { expect } from 'chai'
import type {
  CharacterEncodingOverlay, EntryOverlay, EntryCodeOverlay, FormatOverlay,
  InformationOverlay, LabelOverlay, MetaOverlay, UnitOverlay
} from 'oca.js'
import { AttributeBuilder, AttributeType, Encoding, Entry, OCA, OCABuilder } from 'oca.js'

describe('Plain OCA is built', () => {
  const oca: OCA = new OCABuilder(Encoding.Utf8).finalize()

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

describe('OCA with attributes is built', () => {
  const oca = new OCABuilder(Encoding.Utf8)
    .addName({
      en_EN: "Driving Licence",
      pl_PL: "Prawo Jazdy"
    })
    .addDescription({
      en_EN: "DL desc",
      pl_PL: "PJ desc"
    })
    .addAttribute(
      new AttributeBuilder("attr_name", AttributeType.Number)
      .setPii()
      .addUnit("days")
      .addLabel({
        en_EN: "Name: ",
        pl_PL: "Imię: "
      })
      .addInformation({
        en_EN: "en info",
        pl_PL: "pl info"
      })
      .addEntries([
        new Entry("o1", {
          en_EN: "option 1",
          pl_PL: "opcja 1"
        }).build(),
        new Entry("o2", {
          en_EN: "option 2",
          pl_PL: "opcja 2"
        }).build()
      ])
      .build()
    )
    .addAttribute(
      new AttributeBuilder("attr2", AttributeType.Date)
      .addEncoding(Encoding.Iso8859_1)
      .addFormat("DD.MM.YYYY")
      .addLabel({
        en_EN: "Date: ",
        pl_PL: "Data: "
      })
      .build()
    )
    .finalize()

  describe("Capture Base", () => {
    const captureBase = oca.capture_base

    it('attributes properly added', () => {
      expect(captureBase.attributes).to.have.keys("attr_name", "attr2")
      expect(captureBase.attributes).to.have.property("attr_name", "Number")
      expect(captureBase.attributes).to.have.property("attr2", "Date")
      expect(captureBase.pii).to.eql(["attr_name"])
    })
  })

  describe("Overlays", () => {
    const allOverlays = oca.overlays

    describe("Meta", () => {
      const overlays = allOverlays.filter(o => o.type.includes("/meta/")) as MetaOverlay[]

      it('properly defined', () => {
        const expected: {
          [lang: string]: { name: string, description: string }
        } = {
          pl_PL: {
            name: "Prawo Jazdy",
            description: "PJ desc"
          },
          en_EN: {
            name: "Driving Licence",
            description: "DL desc"
          }
        }

        expect(overlays).to.be.lengthOf(Object.keys(expected).length)

        overlays.forEach(overlay => {
          const exp = expected[overlay.language]
          expect(exp).to.exist
          expect(overlay.name).to.be.eql(exp.name)
          expect(overlay.description).to.be.eql(exp.description)
        })
      })
    })

    describe("Character Encoding", () => {
      const overlays = allOverlays.filter(o => o.type.includes("/character_encoding/")) as CharacterEncodingOverlay[]

      it('properly defined', () => {
        expect(overlays).to.lengthOf(1)
        const overlay = overlays[0]

        expect(overlay.default_character_encoding).to.eql("utf-8")
        expect(overlay.attr_character_encoding).to.have.keys("attr2")
        expect(overlay).to.have.nested.property("attr_character_encoding.attr2", "iso-8859-1")
      })
    })

    describe("Unit", () => {
      const overlays = allOverlays.filter(o => o.type.includes("/unit/")) as UnitOverlay[]

      it('properly defined', () => {
        expect(overlays).to.lengthOf(1)
        const overlay = overlays[0]

        expect(overlay.attr_units).to.have.keys("attr_name")
        expect(overlay).to.have.nested.property("attr_units.attr_name", "days")
      })
    })

    describe("Format", () => {
      const overlays = allOverlays.filter(o => o.type.includes("/format/")) as FormatOverlay[]

      it('properly defined', () => {
        expect(overlays).to.lengthOf(1)
        const overlay = overlays[0]

        expect(overlay.attr_formats).to.have.keys("attr2")
        expect(overlay).to.have.nested.property("attr_formats.attr2", "DD.MM.YYYY")
      })
    })

    describe("Entry Code", () => {
      const overlays = allOverlays.filter(o => o.type.includes("/entry_code/")) as EntryCodeOverlay[]

      it('properly defined', () => {
        expect(overlays).to.lengthOf(1)
        const overlay = overlays[0]

        expect(overlay.attr_entry_codes).to.have.keys("attr_name")
        expect(overlay).to.have.nested.property("attr_entry_codes.attr_name").members(["o1", "o2"])
      })
    })

    describe("Label", () => {
      const overlays = allOverlays.filter(o => o.type.includes("/label/")) as LabelOverlay[]

      it('properly defined', () => {
        const expected: {
          [lang: string]: { [attr_name: string]: string }
        } = {
          pl_PL: {
            "attr_name": "Imię: ",
            "attr2": "Data: "
          },
          en_EN: {
            "attr_name": "Name: ",
            "attr2": "Date: "
          }
        }
        expect(overlays).to.lengthOf(2)

        overlays.forEach(overlay => {
          const exp = expected[overlay.language]
          expect(exp).to.exist
          expect(overlay.attr_labels).to.have.keys("attr_name", "attr2")
          expect(overlay.attr_labels).to.have.property("attr_name", exp["attr_name"])
          expect(overlay.attr_labels).to.have.property("attr2", exp["attr2"])
        })
      })
    })

    describe("Information", () => {
      const overlays = allOverlays.filter(o => o.type.includes("/information/")) as InformationOverlay[]

      it('properly defined', () => {
        const expected: {
          [lang: string]: { [attr_name: string]: string }
        } = {
          pl_PL: {
            "attr_name": "pl info",
          },
          en_EN: {
            "attr_name": "en info",
          }
        }
        expect(overlays).to.lengthOf(2)

        overlays.forEach(overlay => {
          const exp = expected[overlay.language]
          expect(exp).to.exist
          expect(overlay.attr_information).to.have.keys("attr_name")
          expect(overlay.attr_information).to.have.property("attr_name", exp["attr_name"])
        })
      })
    })

    describe("Entry", () => {
      const overlays = allOverlays.filter(o => o.type.includes("/entry/")) as EntryOverlay[]

      it('properly defined', () => {
        const expected: {
          [lang: string]: { [attr_name: string]: { [entry_code: string]: string } }
        } = {
          pl_PL: {
            "attr_name": { "o1": "opcja 1", "o2": "opcja 2" },
          },
          en_EN: {
            "attr_name": { "o1": "option 1", "o2": "option 2" },
          }
        }
        expect(overlays).to.lengthOf(2)

        overlays.forEach(overlay => {
          const exp = expected[overlay.language]
          expect(exp).to.exist
          expect(overlay.attr_entries).to.have.keys("attr_name")
          expect(overlay.attr_entries).to.have.property("attr_name")
            .that.have.property("o1", exp["attr_name"]["o1"])
          expect(overlay.attr_entries).to.have.property("attr_name")
            .that.have.property("o2", exp["attr_name"]["o2"])
        })
      })
    })
  })
})
