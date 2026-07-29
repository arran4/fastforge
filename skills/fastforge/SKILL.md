# Fastforge CLI Skill

This skill teaches AI agents how to interact with the Fastforge CLI correctly and effectively.

## Core Directives for Agents

- **Non-Interactive Mode:** Always ensure Fastforge commands are run in a way that doesn't expect interactive TTY input. Where available, use structural output.
- **Authentication & Environment:** Do not store plain text secrets in the configuration files if possible. Use environment variables (e.g., `variables: API_KEY: ${PGYER_API_KEY}`) in the `distribute_options.yaml` file.
- **Project Structure:** Fastforge operates on a `distribute_options.yaml` configuration usually at the root of a Flutter project.
- **Packaging vs. Releasing:**
  - `fastforge package` merely builds the app and puts it in the output directory.
  - `fastforge publish` uploads an existing package to a distribution store.
  - `fastforge release` combines both for a complete job described in `distribute_options.yaml`.

## Common Pitfalls & Traps

- **Updating Configurations:** The `distribute_options.yaml` takes precedence for environment variables and tasks. If it does not exist, `fastforge release` will fail.
- **Paths:** When using `fastforge publish --path`, ensure the path exists and matches the expected format from previous packaging steps.
- **Platform Ambiguity:** When using `--platform`, specify `android`, `ios`, `macos`, `linux`, `windows`, or `web`.
- **Target Ambiguity:** Use valid targets for the platform (e.g., `apk`, `aab` for Android; `ipa` for iOS; `dmg`, `pkg`, `zip` for macOS).

## Command References

- **Package:** `fastforge package --platform <platform> --targets <target>`
- **Publish:** `fastforge publish --path <path> --targets <store_name>`
- **Release:** `fastforge release --name <job_name>`

By keeping these patterns in mind, AI agents can reliably build, package, and publish Flutter applications using Fastforge.
