# 0.5.0 (2019-06-23)

## New features

- Implement `salmon init` command. See https://github.com/mozamimy/salmon#initialize-salmon-project also.

## Bug fixes

- Fix slice range error when the number of article is less than 5.

## Improvements

- Improve log message more user friendly.
- Create build directory automatically if it is not exist.

# 0.4.1 (2019-06-16)

## Bug fixes

- Make `site_root` config handle URL terminated by `/`. Hence, both `https://example.com` and `https://example.com/` are acceptable.

## Improvements

- Improve help messages

# 0.4.0 (2019-06-02)

## New features

- New command `salmon new` which helps to start writing quickly. It creates an article template and some directories for you.

## Changes

- Set default log level to `info`.
