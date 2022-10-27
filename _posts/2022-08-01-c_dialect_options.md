---
title: C dialect options
date: 2022-08-01
layout: post
mathjax: true
category:
- lang
---
Ref: [1](https://www.acrc.bris.ac.uk/acrc/RedHat/rhel-gcc-en-4/c-dialect-options.html), [2](https://stackoverflow.com/questions/17206568/what-is-the-difference-between-c-c99-ansi-c-and-gnu-c)

|ISO C90|ISO C99|
|-------|-------|
|-ansi, -std=c89, iso9899:1990, (ISO C90)|-std=c99, iso9899:1999|
|-std=iso9899:199409 (ISO C90 as modified in amendment 1)|-std=gnu99 (iso c99 + gnu extensions, gcc default)|
|-std=gnu89 (iso c90 + gnu extensions + some c99 features)||

|ISO C11|ISO C18|
|-------|-------|
|-std=c11|-std=c17, -std=c18|

1. iso9899 (C Lang Spec Number of ISO)
1. `-ansi` for C++ mode means that remove conflics gnu extensions with ISO C++
1. The word `ansi` means American Nation Standard Institute, c89 means ansi 89 (the first standard version), however c99 is iso 1999.The history is `ansi89 -> iso90(same with ansi89) -> iso99`, in other words, iso take the ownership of the C Lang standard from ansi.
1. c18 is created on 2017 and released on 2018, so it’s called c17 or c18. It contains no new features, just corrections

## Runtime Environments

|std|no_std|
|---|------|
|-fhosted (takes place in a hosted env)|-fno-hosted|
|-fno-freestanding|-ffreestanding|
|-fno-builtin, -fno-builtin-\<xxx\>||

1. fno-builtin: Don’t recognize built-in functions that do not begin with `__builtin_` as prefix, `__built_in_xxx` always existed.

1. fno-builtin-\<xxx\> such as `-fno-builtin-printf`.

1. on `no_std` env using `__builtin_xxx` instead