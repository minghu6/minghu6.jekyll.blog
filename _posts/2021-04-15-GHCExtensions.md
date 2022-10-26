---
title: GHC extensions
date: 2021-04-15
layout: post
mathjax: true
category:
- Haskell
- GHC
---
[`AllowAmbiguousTypes`](https://downloads.haskell.org/~ghc/8.10.4/docs/html/users_guide/glasgow_exts.html#extension-AllowAmbiguousTypes)

since 7.8

The ambiguity check rejects functions that can never be called.

[`ScopedTypeVariables`](https://downloads.haskell.org/~ghc/8.10.4/docs/html/users_guide/glasgow_exts.html#extension-ScopedTypeVariables)

since 6.8.1

Enable lexical scoping of type variables explicitly introduced with `forall`.

[`TypeApplications`](https://downloads.haskell.org/~ghc/8.10.4/docs/html/users_guide/glasgow_exts.html#extension-TypeApplications)

since 8.0.1

Allow the use of type application syntax. (类型构造器)

本来只能：

````haskell
> show (read "5"::Int)
"5"
````

现在可以使用这种语法：

````haskell
> show (read @Int "5" )
"5"
````

[`TypeFamilies`](https://downloads.haskell.org/~ghc/8.10.4/docs/html/users_guide/glasgow_exts.html#extension-TypeFamilies)

facilitate type-level programming