import 'dart:ffi';
import 'dart:io';
import 'package:oca/oca.dart';

void main(List<String> arguments) async {
  final lib = DynamicLibrary.open("../target/debug/libocadart.so");

  late final api = OcaDartImpl(lib);

  print("Works");
}
