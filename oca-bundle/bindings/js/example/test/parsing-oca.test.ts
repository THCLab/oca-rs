import { expect } from 'chai'
import { OCABox } from 'oca.js'
import * as util from 'util'
const oca_bundle_json = require('./assets/oca.json')

describe('OCA is loaded', () => {
  const oca_box = new OCABox().load(oca_bundle_json)
    .addClassification('test_classification')

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
