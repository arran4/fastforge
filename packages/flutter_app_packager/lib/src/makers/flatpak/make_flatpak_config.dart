import 'dart:io';

import 'package:flutter_app_packager/src/api/app_package_maker.dart';

class MakeFlatpakConfig extends MakeLinuxPackageConfig {
  MakeFlatpakConfig({
    required this.appId,
    this.runtime = 'org.freedesktop.Platform',
    this.runtimeVersion = '23.08',
    this.sdk = 'org.freedesktop.Sdk',
    List<String>? finishArgs,
  }) : finishArgs =
           finishArgs ?? const ['--share=network', '--filesystem=home'];

  factory MakeFlatpakConfig.fromJson(Map<String, dynamic> map) {
    return MakeFlatpakConfig(
      appId: map['app_id'] as String,
      runtime: map['runtime'] as String? ?? 'org.freedesktop.Platform',
      runtimeVersion: map['runtime_version'] as String? ?? '23.08',
      sdk: map['sdk'] as String? ?? 'org.freedesktop.Sdk',
      finishArgs: (map['finish_args'] as List?)?.cast<String>(),
    );
  }

  final String appId;
  final String runtime;
  final String runtimeVersion;
  final String sdk;
  final List<String> finishArgs;

  String manifestContent(String binaryName) {
    final finish = finishArgs.map((e) => '  - $e').join('\n');
    return '''
app-id: $appId
runtime: $runtime
runtime-version: $runtimeVersion
sdk: $sdk
command: $binaryName
finish-args:
$finish
modules:
  - name: $appId
    buildsystem: simple
    build-commands:
      - cp -r * /app/
    sources:
      - type: dir
        path: app
''';
  }
}

class MakeFlatpakConfigLoader extends DefaultMakeConfigLoader {
  @override
  MakeConfig load(
    Map<String, dynamic>? arguments,
    Directory outputDirectory, {
    required Directory buildOutputDirectory,
    required List<File> buildOutputFiles,
  }) {
    final baseMakeConfig = super.load(
      arguments,
      outputDirectory,
      buildOutputDirectory: buildOutputDirectory,
      buildOutputFiles: buildOutputFiles,
    );
    final map = loadMakeConfigYaml(
      '$platform/packaging/$packageFormat/make_config.yaml',
    );
    return MakeFlatpakConfig.fromJson(map).copyWith(baseMakeConfig);
  }
}
