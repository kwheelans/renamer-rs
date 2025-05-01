# Unreleased
## Changes
- Implement `Default` for `Delimiter`
- Remove `file()` and `files()` methods and replace with `input()` and `inputs()`
- Inputs must be a variant of `InputType`
- Made `FormatPattern` and `FormatType private as they are only needed internally

## Fixes
- Fix selectors not be substituted properly in the format selector

# v0.2.0
## Changes
- structs accessible from crate level
- add documentation
- filename_as_string function should not be public

# v0.1.0
Initial release

## Features
- `ProcessorBuilder` to support creation of `Renamed` trait structs
- use `Format` struct to rename files from segments
- use `Delimter` to split strings into segments
- use `Selector` to pattern match to a segment
- use `Trim` to remove specified values from segments(left, right, both)
- use `Replacer` to replace specified values with a substitute
- use `Extractor` to select values from the original string value before segmentation