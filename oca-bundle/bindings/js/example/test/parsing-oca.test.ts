import { expect } from 'chai'
import { OCABox } from 'oca.js'
const oca_bundle_json = require('./assets/oca.json')

describe('OCA is loaded', () => {
  const oca_box = new OCABox().load(oca_bundle_json)
    .addClassification('test_classification')

  describe('to AST', () => {
    it('has a valid AST', () => {
      const ast = oca_box.toAST()
      expect(ast.version).to.be.equal('1.0.0')
      expect(ast.commands.map(c => c.object_kind)).to.be.eql( [
        'CaptureBase',
        'Meta',
        'Meta',
        'Meta',
        'Meta',
        'CharacterEncoding',
        'EntryCode',
        'Entry',
        'Label',
        'Information',
        'Information',
        'Format',
        'Unit'
      ])
    })
  })

  it('has list of attributes', () => {
    const attributes = oca_box.attributes()

    expect(attributes).to.be.an('array')
    expect(attributes).to.have.lengthOf(21)
  })

  it('has meta', () => {
    const meta = oca_box.meta()

    expect(meta).to.be.an('object')
    expect(meta).to.deep.equal(
      {
        fra: {
          name: "VIZ pour passeport numérique",
          description: "Un formulaire à utiliser pour capturer les données de la zone d'inspection visuelle pour un passeport numérique"
        },
        eng: {
          description: "A form to be used for capturing Visual Inspection Zone data for a Digital Passport",
          name: "VIZ for Digital Passport"
        },
        epo: {
          name: "Cifereca pasporto",
          description: "Formo por kolekti ciferecajn pasportajn datumojn"
        },
        pol: {
          description: "Formularz służący do zebrania danych dotyczących paszportu cyfrowego",
          name: "Passport cyfrowy"
        }
      }
    )
  })

  it('has classification', () => {
    const classification = oca_box.classification()

    expect(classification).to.be.equal('test_classification')
  })
})
