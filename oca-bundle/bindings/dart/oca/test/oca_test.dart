import 'dart:ffi';

import 'package:oca/oca.dart';
import 'package:test/test.dart';

void main() {
  test('it works', () async {
    final dylib = DynamicLibrary.open("../target/debug/libocadart.so");

    late final api = OcaDartImpl(dylib);

    final ocaBox = await OcaBox.newOcaBox(bridge: api)
      ..addMeta(name: "name", value: "value", lang: "en")
      ..addMeta(name: "description", value: "Test case OCA", lang: "en")
      ..addFormLayout(layout: """
        elements:
          - type: "test"
        """)
      ..addCredentialLayout(layout: """
        version: "1.0"
        pages:
            - config:
                name: "test"
              elements:
                - type: "test"
        """);

    final attr1 = await OcaAttr.newOcaAttr(bridge: api, name: "name")
      ..setAttributeType(attrType: OcaAttrType.Text)
      ..setFlagged()
      ..setEncoding(encoding: OcaEncoding.Utf8)
      ..setCardinality(cardinality: "1")
      ..setConformance(conformance: "0")
      ..setLabel(lang: "en", label: "Name")
      ..setInformation(lang: "en", information: "name information")
      ..setEntryCodes(entryCodes: ["a", "b"])
      ..setEntry(
          lang: "en",
          entries: await OcaMap.newOcaMap(bridge: api)
            ..insert(key: "a", value: "Option A")
            ..insert(key: "b", value: "Option B"))
      ..setUnitMetric(unit: OcaMetricUnit.Kilogram)
      ..setFormat(format: "^[a-zA-Z]*\$");
    await ocaBox.addAttribute(attr: attr1);

    final attr2 = await OcaAttr.newOcaAttr(bridge: api, name: "age")
      ..setAttributeType(attrType: OcaAttrType.Numeric)
      ..setFlagged()
      ..setEncoding(encoding: OcaEncoding.Utf8)
      ..setConformance(conformance: "M")
      ..setCardinality(cardinality: "2")
      ..setLabel(lang: "en", label: "Age")
      ..setInformation(lang: "en", information: "age information")
      ..setEntryCodes(entryCodes: ["a", "b"])
      ..setEntry(
          lang: "en",
          entries: await OcaMap.newOcaMap(bridge: api)
            ..insert(key: "a", value: "Option A")
            ..insert(key: "b", value: "Option B"))
      ..setUnitMetric(unit: OcaMetricUnit.Kilogram)
      ..setFormat(format: "^[a-zA-Z]*\$");
    await ocaBox.addAttribute(attr: attr2);

    final ocaBundle = await ocaBox.generateBundle();

    print(await ocaBundle.toJson());
    print(await ocaBundle.said());
    final capBase = await ocaBundle.captureBase();
    final attrs = await capBase.attributes();
    expect((await attrs.getKeys()).length, 2);
    expect((await capBase.flaggedAttributes()).length, 2);
    expect((await ocaBundle.overlays()).length, 12);

    final json = await ocaBundle.toJson();
    print(json);
    final ocaBundle2 = await api.loadOca(json: json);

    expect((await ocaBundle2.overlays()).length, 12);
  });
}
