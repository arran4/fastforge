import 'dart:io';

import 'package:flutter_app_packager/src/api/make_config.dart';
import 'package:pub_semver/pub_semver.dart';
import 'package:pubspec_parse/pubspec_parse.dart';
import 'package:test/test.dart';

void main() {
  group('MakeConfig', () {
    test('#1', () {
      final makeConfig = MakeConfig()
        ..buildMode = 'release'
        ..buildOutputDirectory = Directory('build')
        ..buildOutputFiles = []
        ..platform = 'android'
        ..packageFormat = 'apk'
        ..outputDirectory = Directory('dist/')
        ..pubspec = Pubspec(
          'test_app',
          version: Version.parse('1.0.0'),
        );
      expect(
        makeConfig.outputArtifactPath,
        'dist/1.0.0/test_app-1.0.0-android.apk',
      );
    });
    test('#2', () {
      final makeConfig = MakeConfig()
        ..buildMode = 'release'
        ..buildOutputDirectory = Directory('build')
        ..buildOutputFiles = []
        ..platform = 'android'
        ..packageFormat = 'apk'
        ..outputDirectory = Directory('dist/')
        ..pubspec = Pubspec(
          'test_app',
          version: Version.parse('1.0.0+1'),
        );
      expect(
        makeConfig.outputArtifactPath,
        'dist/1.0.0+1/test_app-1.0.0+1-android.apk',
      );
    });

    test('loadMakeConfigYaml merges parent', () {
      final tempDir = Directory.systemTemp.createTempSync();
      final parentFile = File('${tempDir.path}/parent.yaml')
        ..writeAsStringSync(
            'maintainer:\n  name: Foo\n  email: foo@ex.com\npackage_name: parent');
      final childFile = File('${tempDir.path}/child.yaml')
        ..writeAsStringSync('maintainer:\n  email: bar@ex.com');

      final map = loadMakeConfigYaml(
        childFile.path,
        parentPath: parentFile.path,
      );

      expect(map['package_name'], 'parent');
      expect((map['maintainer'] as Map)['name'], 'Foo');
      expect((map['maintainer'] as Map)['email'], 'bar@ex.com');
    });
  });
}
