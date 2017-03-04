# ChangeLog #

## 0.2.1 (2017-03-04) ##
* Update *regex* dependency to 0.2

## 0.2.0 (2016-12-23) ##
* Fix use of `crowbook-intl-runtime` in macro generation. This might
  be a breaking change in some cases.
* Structs now derive `Debug` and `Clone`.

## 0.1.0 (2016-11-18) ##
* Renamed library `crowbook-localize` to `crowbook-intl`.
* Requires `rustc` >= 1.13.0.
* Split library between `crowbook-intl` and
  `crowbook-intl-runtime`. The latter defines some runtime functions
  used by the generated macros. This split (should) allow to have
  multiple libraries using `crowbook-intl` in the same program.
* `Localizer::write_macro_file` now takes an `AsRef<Path>` instead of
  an `&str`.

## 0.0.9 (2016-10-26) ##
* Make it possible to `include!` the generated macro files from
  `OUT_DIR`, and document this way of doing.

## 0.0.8 (2016-10-14) ##
* In order to correctly handle multiline strings, and to make it
  possible to use the same translation for two strings that are
  identical but don't use backslash the same way, the API had to be
  modified a bit. It should now work correctly-ish.
* Messages are now sorted (by file of apparition) before being written
  to `.pot` file.

## 0.0.7 (2016-10-13) ##
* `Extractor` now uses `escape_string` for its keys, allowing to use
  `lformat!` with multiline strings using the `\` escape at end of line. 

## 0.0.6 (2016-10-13) ##
* '\' followed by a newline is now escaped (well, suppressed along
  leading whitespace on next line) when generating pot file. 
* Now uses Travis for continuous integration.
	
## 0.0.5 (2016-10-10) ##
* Newlines characters are escaped when generating pot file so
  `msgmerge` doesn't complain

## 0.0.4 (2016-10-09) ##
`crowbook-localize` should now be able to generate `.pot` files that
are compatible with `msgmerge` and be able to read `.po` files that
have been updated with `msgmerge`.
* Fix printing and reading of strings which caused problems with
  escape characters.
* Add support for multiline strings ala gettext in translation files.

## 0.0.3 (2016-10-08) ##
* Added the `Extractor` struct, that generates a pot-like file looking
  at `lformat!` invocations in your source code.

## 0.0.2 (2016-10-07) ##
* Only export `Localizer` as public API.
* Rewrote `lformat!` macro generation.

## 0.0.1 (2016-10-07) ##
* Initial (pre-)release
