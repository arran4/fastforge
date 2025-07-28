import 'dart:convert';
import 'dart:io';

import 'package:flutter_app_packager/src/api/make_config.dart';
import 'package:flutter_app_packager/src/api/make_result.dart';
import 'package:shell_executor/shell_executor.dart';
import 'package:yaml/yaml.dart';

export 'make_config.dart';
export 'make_error.dart';
export 'make_result.dart';

Map<String, dynamic> _mergeYamlMap(
  Map<String, dynamic> parent,
  Map<String, dynamic> child,
) {
  final result = Map<String, dynamic>.from(parent);
  child.forEach((key, value) {
    if (value is Map && result[key] is Map) {
      result[key] = _mergeYamlMap(
        Map<String, dynamic>.from(result[key] as Map),
        Map<String, dynamic>.from(value as Map),
      );
    } else {
      result[key] = value;
    }
  });
  return result;
}

Map<String, dynamic> loadMakeConfigYaml(
  String path, {
  String? parentPath,
}) {
  Map<String, dynamic> map = {};
  if (parentPath != null && File(parentPath).existsSync()) {
    final parentDoc = loadYaml(File(parentPath).readAsStringSync());
    map = json.decode(json.encode(parentDoc));
  }
  final yamlDoc = loadYaml(File(path).readAsStringSync());
  final childMap = json.decode(json.encode(yamlDoc));
  return _mergeYamlMap(map, childMap);
}

abstract class AppPackageMaker {
  List<Command> get requirements => [];

  String get name => throw UnimplementedError();
  String get platform => throw UnimplementedError();
  bool get isSupportedOnCurrentPlatform => true;
  String get packageFormat => throw UnimplementedError();

  MakeConfigLoader get configLoader {
    return DefaultMakeConfigLoader()
      ..platform = platform
      ..packageFormat = packageFormat;
  }

  MakeResultResolver get resultResolver => DefaultMakeResultResolver();

  bool match(String platform, [String? target]) {
    return this.platform == platform && name == target;
  }

  Future<MakeResult> make(MakeConfig config) {
    throw UnimplementedError();
  }
}
