---
title: Common Lisp 语法
date: 2020-07-22
layout: post
mathjax: true
category:
- lang
---
1. 列表和点对的区别：
   
   点对是列表构成的基本单元，列表通过点对嵌套构成:
   
   `'(1 . (2 . nil))`
   
   反过来，点对`(1 . 2)`看起来像长度为2的列表，但如果它的cdr不是列表，那么它就不是列表