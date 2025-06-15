import 'dart:io';

import 'package:flutter_app_packager/src/api/app_package_maker.dart';
import 'package:flutter_app_packager/src/makers/flatpak/make_flatpak_config.dart';
import 'package:path/path.dart' as path;
import 'package:shell_executor/shell_executor.dart';

class AppPackageMakerFlatpak extends AppPackageMaker {
  @override
  String get name => 'flatpak';
  @override
  String get platform => 'linux';
  @override
  bool get isSupportedOnCurrentPlatform => Platform.isLinux;
  @override
  String get packageFormat => 'flatpak';

  @override
  MakeConfigLoader get configLoader {
    return MakeFlatpakConfigLoader()
      ..platform = platform
      ..packageFormat = packageFormat;
  }

  @override
  Future<MakeResult> make(MakeConfig config) {
    return _make(
      config.buildOutputDirectory,
      outputDirectory: config.outputDirectory,
      makeConfig: config as MakeFlatpakConfig,
    );
  }

  Future<MakeResult> _make(
    Directory appDirectory, {
    required Directory outputDirectory,
    required MakeFlatpakConfig makeConfig,
  }) async {
    final packagingDirectory = makeConfig.packagingDirectory;
    final appDir = Directory(path.join(packagingDirectory.path, 'app'));
    await $('cp', ['-fr', '${appDirectory.path}/.', appDir.path]);

    final manifestFile = File(path.join(packagingDirectory.path, 'manifest.yml'))
      ..writeAsStringSync(
        makeConfig.manifestContent(makeConfig.appBinaryName),
      );

    final buildDir = Directory(path.join(packagingDirectory.path, 'build'));
    final repoDir = Directory(path.join(packagingDirectory.path, 'repo'));
    buildDir.createSync(recursive: true);
    repoDir.createSync(recursive: true);

    var result = await $('flatpak-builder', [
      '--force-clean',
      buildDir.path,
      manifestFile.path,
    ]);
    if (result.exitCode != 0) throw MakeError();

    result = await $('flatpak', [
      'build-bundle',
      repoDir.path,
      makeConfig.outputFile.path,
      makeConfig.appId,
      config.appVersion.toString(),
    ]);
    if (result.exitCode != 0) throw MakeError();

    packagingDirectory.deleteSync(recursive: true);
    return MakeResult(makeConfig);
  }
}
