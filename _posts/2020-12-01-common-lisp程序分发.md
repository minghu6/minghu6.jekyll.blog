---
title: CL程序分发
date: 2020-12-01
layout: post
mathjax: true
category:
- CommonLisp
---
## 1. SBCL 导出镜像文件：

````commonlisp
;;启动repl后加载程序环境
(load "php-syntax.lisp")
;;环境保存到镜像
(save-lisp-and-die "core-php-synax") 
````

加载镜像 `rlwrap sbcl --core core-php-synax`