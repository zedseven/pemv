# Changelog

All notable changes to this project will be documented in this file.

## [0.4.2] - 2022-10-25

### Miscellaneous Tasks

- Clean up the generated tag changelogs so that they render properly on GitHub. (hopefully) ([95e6104](https://github.com/zedseven/pemv/commit/95e6104))

## [0.4.1] - 2022-10-25

### Refactor

- Apply Clippy suggestions. ([18f8496](https://github.com/zedseven/pemv/commit/18f8496))

## [0.4.0] - 2022-10-25

### Bug Fixes

- Fix a panic during BER-TLV parsing. ([92da197](https://github.com/zedseven/pemv/commit/92da197))
- Fix several issues found after adding testing of Ingenico TLV parsing. ([9d2d3be](https://github.com/zedseven/pemv/commit/9d2d3be))
- Fix a small issue found after adding testing for automatic TLV parsing, where an empty value would match to any kind of TLV data. ([840311d](https://github.com/zedseven/pemv/commit/840311d))
- Make the Ingenico TLV parser handle leading zeroes. ([5c2ef9b](https://github.com/zedseven/pemv/commit/5c2ef9b))

### Continuous Integration

- Update `Swatinem/rust-cache` to `v2`. ([5b21fe6](https://github.com/zedseven/pemv/commit/5b21fe6))

### Features

- The CVM List breakdown now only shows the X and Y values if they're being used, since most of the time they go unused. ([5364e0c](https://github.com/zedseven/pemv/commit/5364e0c))
- Add explanations for a few initialisms that may not be known to everyone. ([d11f3a0](https://github.com/zedseven/pemv/commit/d11f3a0))
- Add sorting for parsed tags. Sorting is on by default. ([f9b74e7](https://github.com/zedseven/pemv/commit/f9b74e7))
- Add a new option to identify tags by name. ([7640012](https://github.com/zedseven/pemv/commit/7640012))
- Add the short alias `-?` to `--help`. It's not clear if anyone will ever use it, but I think it's neat. ([4dfb525](https://github.com/zedseven/pemv/commit/4dfb525))
- Add aliases for each individual EMV tag option for their actual tag names. ([e5e7f64](https://github.com/zedseven/pemv/commit/e5e7f64))

### Miscellaneous Tasks

- Add Tarpaulin-generated files to `.gitignore`. ([11ccdfa](https://github.com/zedseven/pemv/commit/11ccdfa))
- Update to Clap v4. ([985d27d](https://github.com/zedseven/pemv/commit/985d27d))
- Set up the repository for automated changelog & tag generation using git-cliff. ([415bcc8](https://github.com/zedseven/pemv/commit/415bcc8))

### Refactor

- Massively overhaul the way bitflag values are defined so there's now zero boilerplate. Also remove the stupid `Bytes` storage on every bitflag struct. ([844fe38](https://github.com/zedseven/pemv/commit/844fe38))
- Apply the new Clippy lint, `uninlined-format-args`. ([18d1bd0](https://github.com/zedseven/pemv/commit/18d1bd0))
- Run Rustfmt after running `cargo clippy --fix`. ([0fb9023](https://github.com/zedseven/pemv/commit/0fb9023))
- Optimise the CVM List display slightly. ([85024d2](https://github.com/zedseven/pemv/commit/85024d2))
- Update `rustfmt.toml` to add some new config options and remove a few deprecated ones, then reformat everything. ([df9b085](https://github.com/zedseven/pemv/commit/df9b085))

### Testing

- Add testing for BER-TLV parsing. ([9e33b67](https://github.com/zedseven/pemv/commit/9e33b67))
- Add testing for Ingenico TLV parsing. ([2ab7d33](https://github.com/zedseven/pemv/commit/2ab7d33))
- Add testing for automatic TLV parsing. ([7cc5786](https://github.com/zedseven/pemv/commit/7cc5786))
- Add more testing for EMV values. ([380a661](https://github.com/zedseven/pemv/commit/380a661))
- Add testing for the new `CvmCondition` function, `references_x_or_y_value`. ([07cfe98](https://github.com/zedseven/pemv/commit/07cfe98))

## [0.3.0] - 2022-10-25

### Bug Fixes

- Make CLI arguments only override the config file if their value was provided by the user directly. ([90c8c66](https://github.com/zedseven/pemv/commit/90c8c66))

### Continuous Integration

- Add a check for misplaced `use` statements. This should help combat a common issue with Rustfmt, where it associates the `// Uses` header with the `use` statement directly below. This means that after new `use` statements are added, the header is not at the top anymore. ([90d93be](https://github.com/zedseven/pemv/commit/90d93be))
- Add testing and code coverage calculation to CI, using Tarpaulin and Codecov. ([aeab01d](https://github.com/zedseven/pemv/commit/aeab01d))

### Documentation

- Put bare hyperlinks into `<>` so Rustdoc automatically makes them clickable. (Rustdoc `bare_urls` lint) ([903cb9c](https://github.com/zedseven/pemv/commit/903cb9c))
- Add a doc line I forgot to the new enum macros. ([645739f](https://github.com/zedseven/pemv/commit/645739f))
- Remove a redundant line in the CLI help text. ([6d809d6](https://github.com/zedseven/pemv/commit/6d809d6))
- Add visible command line argument aliases. ([2ab055c](https://github.com/zedseven/pemv/commit/2ab055c))
- Add a code coverage badge. ([27208d2](https://github.com/zedseven/pemv/commit/27208d2))
- Add a CI status badge. ([b85192d](https://github.com/zedseven/pemv/commit/b85192d))

### Features

- Add support for parsing Transaction Type values, which are in tag `9C`. ([00215fe](https://github.com/zedseven/pemv/commit/00215fe))
- Add support for parsing Authorisation Response Code values, which are in tag `8A`. ([b14db42](https://github.com/zedseven/pemv/commit/b14db42))
- Add support for parsing Terminal Type (tag `9F35`), Terminal Capabilities (tag `9F33`), and Additional Terminal Capabilities (tag `9F40`) values. ([9b8affc](https://github.com/zedseven/pemv/commit/9b8affc))
- Add Figment configuration with CLI argument overrides. This allows configuration to be done via config files, environment variables, and CLI arguments - in that order. ([599cfd0](https://github.com/zedseven/pemv/commit/599cfd0))
- Add a CLI argument for changing the character that indicates masked data when parsing TLV data. This currently goes unused. Also make a few small improvements to CLI-parsing in general. ([54103a5](https://github.com/zedseven/pemv/commit/54103a5))
- Add support for handling masked data without failing during TLV parsing. This is useful because EMV data can contain sensitive information, and if using this tool on data pulled from log files, some info may be masked. ([34ab63f](https://github.com/zedseven/pemv/commit/34ab63f))
- Add support for parsing Ingenico's TLV format. ([7599d7c](https://github.com/zedseven/pemv/commit/7599d7c))
- Revamp how constructed data objects are handled. This removes the `ber-tlv-simple` CLI argument and makes it so that if a tag's data cannot be parsed as a constructed data object, it is assumed to not be a constructed tag and no error occurs. ([882a8ff](https://github.com/zedseven/pemv/commit/882a8ff))
- Make it so that IAC values don't show flags that normally indicate errors or warnings in colour, since an IAC value is effectively a checklist, not a result. ([7c48890](https://github.com/zedseven/pemv/commit/7c48890))
- Add support for parsing POS Entry Mode values, which are in tag `9F39`. ([cf47933](https://github.com/zedseven/pemv/commit/cf47933))
- Add an automatic TLV format parser that attempts to parse the data however possible. ([9f38a94](https://github.com/zedseven/pemv/commit/9f38a94))
- Make CLI arguments require a value, or otherwise return an error. ([d7eb0ef](https://github.com/zedseven/pemv/commit/d7eb0ef))

### Miscellaneous Tasks

- Update dependencies. ([c8533e0](https://github.com/zedseven/pemv/commit/c8533e0))

### Refactor

- Refactor a lot of repetitive enum definition into macros. This makes the code significantly easier to maintain. ([2ebcf66](https://github.com/zedseven/pemv/commit/2ebcf66))
- Improve the macros used to create the enum values used ubiquitously throughout the program. ([9c7e167](https://github.com/zedseven/pemv/commit/9c7e167))

### Testing

- Add a single demo test. ([ee395bc](https://github.com/zedseven/pemv/commit/ee395bc))
- Add some starting tests. ([0fff464](https://github.com/zedseven/pemv/commit/0fff464))
- Add testing for pretty much all bitflag and enum values. This covers a large percentage of the code, but still leaves the TLV parsing and configuration code for testing. ([c6eb339](https://github.com/zedseven/pemv/commit/c6eb339))

## [0.2.0] - 2022-10-25

### Bug Fixes

- Fix a potential 'index out of bounds' panic when parsing BER-TLV data. ([98b8eee](https://github.com/zedseven/pemv/commit/98b8eee))

### Documentation

- Fix a tiny issue with the help text. ([f120e3c](https://github.com/zedseven/pemv/commit/f120e3c))
- Update `README.md` to change the wording slightly. ([efc8eb4](https://github.com/zedseven/pemv/commit/efc8eb4))
- Fix an incorrect comment. ([1e43e3b](https://github.com/zedseven/pemv/commit/1e43e3b))
- Add an explanation for what `BER-TLV` data is. ([ae3ad6d](https://github.com/zedseven/pemv/commit/ae3ad6d))
- Change `a genuine value` to `genuine data`. ([9196fff](https://github.com/zedseven/pemv/commit/9196fff))

### Features

- Implement the basics of EMV TLV parsing (currently only supporting BER-TLV) without a user display. ([f4c7a68](https://github.com/zedseven/pemv/commit/f4c7a68))
- Display the complete breakdown for parsed EMV TLV data. ([e91b332](https://github.com/zedseven/pemv/commit/e91b332))
- Add a `ber-tlv-simple` option to support parsing BER-TLV data that doesn't use constructed data objects. ([7d37c0f](https://github.com/zedseven/pemv/commit/7d37c0f))
- Remove the redundant output in `display_breakdown`. ([ddb0534](https://github.com/zedseven/pemv/commit/ddb0534))
- Make it so the tag `5F30` is parsed as a service code when parsing TLV data. ([e30251a](https://github.com/zedseven/pemv/commit/e30251a))
- Add tag length display to TLV-parsing output. ([0265419](https://github.com/zedseven/pemv/commit/0265419))
- Add support for parsing IAC (Issuer Action Code) values (tags `9F0D`, `9F0E`, and `9F0F`), which basically uses the TVR parsing under the hood. ([baadca1](https://github.com/zedseven/pemv/commit/baadca1))
- Move the `EMV UTILITIES` CLI section to the top, above the `INDIVIDUAL EMV TAGS` section. ([eea569a](https://github.com/zedseven/pemv/commit/eea569a))

### Refactor

- Remove an unused `use` statement. ([a60b668](https://github.com/zedseven/pemv/commit/a60b668))
- Move the EMV tag processing into a dedicated module. ([becbcee](https://github.com/zedseven/pemv/commit/becbcee))
- Move the processing of BCD (Binary-Coded Decimal) data into a dedicated function. ([4dae539](https://github.com/zedseven/pemv/commit/4dae539))
- Rename `ProcessedEmvTag::Parsed`'s `value` field to `raw_tag`, to more accurately indicate what it represents. ([cb3859a](https://github.com/zedseven/pemv/commit/cb3859a))
- Move the BCD parsing for service codes into a `TryFrom<&[u8]>` implementation on `ServiceCode`. ([82a29e8](https://github.com/zedseven/pemv/commit/82a29e8))

## [0.1.4] - 2022-10-25

### Bug Fixes

- Fix a panic when no actionable arguments are provided, but an option like `colour` *is* provided. ([062aed8](https://github.com/zedseven/pemv/commit/062aed8))

### Documentation

- Remove `TODO.md` from version control. ([c0a2e8b](https://github.com/zedseven/pemv/commit/c0a2e8b))
- Add a lot more information to the help documentation. ([6658249](https://github.com/zedseven/pemv/commit/6658249))
- Add documentation about where to find the specifications for the CCD module. ([83e0b7a](https://github.com/zedseven/pemv/commit/83e0b7a))

### Features

- Add compile-time generation of a man page and shell completion scripts, which should make the program more convenient to use. ([6d9faa7](https://github.com/zedseven/pemv/commit/6d9faa7))
- Make the value display a little prettier for the CVM List breakdown. ([1d026fc](https://github.com/zedseven/pemv/commit/1d026fc))
- Add support for parsing CCD-compliant (Common Core Definitions) IAD (Issuer Application Data) values, which are in tag `9F10`. ([a85b28c](https://github.com/zedseven/pemv/commit/a85b28c))

### Refactor

- Apply Clippy suggestions. ([48ced22](https://github.com/zedseven/pemv/commit/48ced22))
- Rename `UnitValue` to `BitflagValue` because the latter better represents what the values are. ([c44fcc0](https://github.com/zedseven/pemv/commit/c44fcc0))
- Improve the code style and add support for displaying nested values with indentation. ([d3d0031](https://github.com/zedseven/pemv/commit/d3d0031))
- Rework the project structure and split out CCD-compliant structures to their own module. ([1585efd](https://github.com/zedseven/pemv/commit/1585efd))
- Improve the way values that are represented by a single numeric value are parsed. ([fa54c2e](https://github.com/zedseven/pemv/commit/fa54c2e))

## [0.1.3] - 2022-10-25

### Documentation

- Add documentation about where to find the specifications for each value. ([be99a6d](https://github.com/zedseven/pemv/commit/be99a6d))
- Add a note to `README.md` about reporting genuine `RFU` values, to handle the case where an unsupported value is found during use. ([841eb58](https://github.com/zedseven/pemv/commit/841eb58))

### Features

- Add support for parsing CVM List (Cardholder Verification Method List) values, which are in tag `8E`. Currently they are not displayed. ([46b34e4](https://github.com/zedseven/pemv/commit/46b34e4))

### Refactor

- Fix a misplaced `use` statement. ([303243c](https://github.com/zedseven/pemv/commit/303243c))
- Move the error display to a proper `Display` implementation on the `ParseError` struct. ([cf0ecfc](https://github.com/zedseven/pemv/commit/cf0ecfc))
- Rework the way CV Rule-related values are displayed internally, and complete the implementation for CVM List parsing. ([f2adf0f](https://github.com/zedseven/pemv/commit/f2adf0f))

## [0.1.2] - 2022-10-25

### Continuous Integration

- Add artifact generation using GitHub Actions. ([a5ca029](https://github.com/zedseven/pemv/commit/a5ca029))

### Documentation

- Add documentation to the `ParseError` enum. ([1c82902](https://github.com/zedseven/pemv/commit/1c82902))

### Features

- Add a help heading to separate the EMV tag options, and add annotations showing the tag associated with each option. ([0650b7e](https://github.com/zedseven/pemv/commit/0650b7e))
- Add support for parsing Service Code values, which come from the track 2 data or EMV tag `5F30`. ([d6dde8d](https://github.com/zedseven/pemv/commit/d6dde8d))

### Miscellaneous Tasks

- Update dependencies. ([5833e53](https://github.com/zedseven/pemv/commit/5833e53))

### Refactor

- Refactor the majority of the codebase, paving the way for more sophisticated parsing and non-EMV values soon. ([1a0836a](https://github.com/zedseven/pemv/commit/1a0836a))
- Rename `status_values` to `unit_values` to better reflect the nature of the data. ([a81a8f6](https://github.com/zedseven/pemv/commit/a81a8f6))
- Refactor the way values are parsed a bit. ([ae06564](https://github.com/zedseven/pemv/commit/ae06564))

## [0.1.1] - 2022-10-25

### Bug Fixes

- Make the CVM Results parser properly parse the result byte. ([5b08c05](https://github.com/zedseven/pemv/commit/5b08c05))

### Continuous Integration

- Add some simple CI (Continuous Integration) that simply checks for unformatted code and failing Clippy lints. ([1a3857e](https://github.com/zedseven/pemv/commit/1a3857e))

### Documentation

- Update `TODO.md` with some additional kinds of values that may be nice to support in the future. ([b848e6d](https://github.com/zedseven/pemv/commit/b848e6d))
- Update `TODO.md` with some additional thoughts. ([708c647](https://github.com/zedseven/pemv/commit/708c647))

### Features

- Implement support for parsing the TVR in a pretty format. There is room for improvement - it does so in a messy and stupid way. ([bb49894](https://github.com/zedseven/pemv/commit/bb49894))
- Add support for parsing CVR (Card Verification Results) values, which are a part of tag `9F10`. ([5a06bf5](https://github.com/zedseven/pemv/commit/5a06bf5))
- Add support for parsing TSI (Transaction Status Information) values, which are in tag `9B`. ([a5b51c5](https://github.com/zedseven/pemv/commit/a5b51c5))
- Add a hex value display above the binary value breakdown. ([4ca8932](https://github.com/zedseven/pemv/commit/4ca8932))
- Add support for parsing CV Rule (Cardholder Verification Rule) values, though for now, this support goes unused. ([e3b9877](https://github.com/zedseven/pemv/commit/e3b9877))
- Add support for parsing CVM Results (Cardholder Verification Method Results) values, which are in tag `9F34`. ([69ed380](https://github.com/zedseven/pemv/commit/69ed380))
- Add a new CV Method that indicates `No CVM Performed`. This isn't explicitly marked in the EMV Books. ([14e9746](https://github.com/zedseven/pemv/commit/14e9746))
- Tweak the display for a few flags in the CVR. ([b695a00](https://github.com/zedseven/pemv/commit/b695a00))
- Add output colouring, showing flags that indicate warnings in yellow and flags that indicate errors in red. ([96fa319](https://github.com/zedseven/pemv/commit/96fa319))
- Adjust the severity of the PIN bypass bit (byte 3, bit 4) in the TVR and add an annotation labelling it as PIN bypass. ([aa18b4c](https://github.com/zedseven/pemv/commit/aa18b4c))

### Miscellaneous Tasks

- Update dependencies. ([3c8c9ee](https://github.com/zedseven/pemv/commit/3c8c9ee))

### Refactor

- Move away from the bitflag-style parsing and add support for parsing multi-bit values. ([f11e579](https://github.com/zedseven/pemv/commit/f11e579))
- Refactor much of the display code to allow display of values with nested information. ([77e2a09](https://github.com/zedseven/pemv/commit/77e2a09))
- Apply Clippy suggestions. ([f18e649](https://github.com/zedseven/pemv/commit/f18e649))

<!-- generated by git-cliff -->
