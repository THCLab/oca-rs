const util = require('util')
const {
  Language, Encoding, OCA, Attribute, AttributeType, Entry
} = require('./pkg')

let oca = (new OCA(Encoding.Utf8))
  .enforceTranslations([Language.En, Language.Pl])
  .addName({
    [Language.En]: "Driving Licence",
    [Language.Pl]: "Prawo Jazdy"
  })
  .addDescription({
    [Language.En]: "DL desc",
    [Language.Pl]: "PJ desc"
  })
  .addAttribute(
    (new Attribute("attr_name", AttributeType.Number))
    .setPii()
    .addUnit("days")
    .addLabel({
      [Language.En]: "Name: ",
      [Language.Pl]: "Imię: "
    })
    .addInformation({
      [Language.En]: "en info",
      [Language.Pl]: "pl info"
    })
    .addEntries([
      new Entry("o1", {
        [Language.En]: "option 1",
        [Language.Pl]: "opcja 1"
      }),
      new Entry("o2", {
        [Language.En]: "option 2",
        [Language.Pl]: "opcja 2"
      })
    ])
    .build()
  )
  .addAttribute(
    (new Attribute("attr2", AttributeType.Date))
    .addEncoding(Encoding.Iso8859_1)
    .addFormat("DD.MM.YYYY")
    .addLabel({
      [Language.En]: "Date: ",
      [Language.Pl]: "Data: "
    })
    .build()
  )

try {
  oca = oca.finalize()
  console.log(
    util.inspect(
      oca,
      { showHidden: false, depth: null, colors: true }
    )
  )
} catch (e) {
  console.error(e)
}
