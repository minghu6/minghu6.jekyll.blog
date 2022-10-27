---
title: 从Template Haskell（Haskell宏）来认识Haskell语法结构
date: 2021-04-17
layout: post
mathjax: true
category:
- lang
---
默认工作版本 `ghc 8.10.4`

从模板Haskell上看，主要有如下几种构造器：

Q Quotation

xxE属于 Exp，是xxExpression

xxP属于Pattern，是xxPatttern

````haskell
genId :: Q Exp
genId = do
  x <- newName "x"
  lamE [varP x] (varE x)
````

等价于

````haskell
genId = [| \x -> x |]  -- 也就是 genId = [| id |]
````

通常宏的返回值都是`Q Exp`意思是Quotation Expression

\[\|…\|\] 其实是\[e|…\|\]的简写

还有分别代表types、pattern，declaration的`[t|...|], [p|...|], [e|...|]`

实际上是

`$([| e |]) = e`

quote `'`

back-quote <code>\`</code>

quasi-quote `~@`