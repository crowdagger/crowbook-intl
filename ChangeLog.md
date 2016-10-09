# ChangeLog #

## unreleased ##
* newlines characters are escaped when generating pot file so
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
