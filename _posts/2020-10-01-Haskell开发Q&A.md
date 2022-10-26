---
title: Haskell开发准备
date: 2020-10-01
layout: post
mathjax: true
category:
- Haskell
---
## 准备环境：

1. 安装系统级Haskell（或者直接跳过1-3，直接到4）

1. 安装Cabal-install

1. 安装Stack

1. 安装ghc/cabal多环境：ghcup
   
   `curl --proto '=https' --tlsv1.2 -sSf https://get-ghcup.haskell.org | sh`
   
   From https://www.haskell.org/ghcup/

### 安装包失败：

考虑使用更低一级的LTS版本：`stack --resolver <Ltsversion> install <Pkgname>`

* <Ltsversion>，exp: `lts-14.22`