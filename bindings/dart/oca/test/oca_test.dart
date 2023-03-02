import 'dart:ffi';

import 'package:oca/oca.dart';
import 'package:test/test.dart';

void main() {
  test('it works', () async {
    final dylib = DynamicLibrary.open("../target/debug/libocadart.so");

    late final api = OcaDartImpl(dylib);

    final oca = OcaBox(
      bridge: api,
      meta_attrs: [
        OcaMetaAttr(bridge: api, name: "name", value: "Test"),
        OcaMetaAttr(bridge: api, name: "description", value: "Test case OCA"),
      ],
      attrs: [
        OcaAttr(
            bridge: api,
            name: "name",
            attr_type: OcaAttrType.Text,
            encoding: OcaAttrEncoding.Utf8,
            cardinality: "1"),
        OcaAttr(
            bridge: api,
            name: "age",
            attr_type: OcaAttrType.Numeric,
            encoding: OcaAttrEncoding.Utf8,
            flagged: true,
            conformance: "M",
            cardinality: "2"),
      ],
    );

    final ocaBundle = oca.generateBundle();

    print(ocaBundle.toJson());

    expect(ocaBundle.captureBase().attributes().length, 2);
    expect(ocaBundle.captureBase().flaggedAttributes().length, 1);
    expect(ocaBundle.overlays().length, 5);
  });
}
