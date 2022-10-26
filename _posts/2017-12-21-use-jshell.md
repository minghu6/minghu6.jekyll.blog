---
title: JShell
date: 2017-12-21
layout: post
mathjax: true
category:
- jshell
---
## Requirement: JDK-9

## 辅助方法

`/imports`

````
查看导入的Java包
````

`/edit <var>`

````
<var>: class-name, var, snippet-id etc.
````

`/edit`

````
开启一个简单编辑器，输入整段代码
````

`/save <file>`

````
Save the commands and snippets currently active to the provided file path.
````

`/save -all <file>`

````
Save all commands and snippets, including overwritten, start-up, and failed commands and snippets to the provided file path.
````

`/save -history <file>`

````
Save all commands and snippets run to the provided file.
````

`/save -start <file>`

````
Save all commands and snippets that initialized the JShell session to the provided path.
````

默认是

````java
import java.io.*;
import java.math.*;
import java.net.*;
import java.nio.file.*;
import java.util.*;
import java.util.concurrent.*;
import java.util.function.*;
import java.util.prefs.*;
import java.util.regex.*;
import java.util.stream.*;
````

`/env <param> <param-value>`

````
设置环境变量, such as `/env -class-path .`
````

`/reset`

````
重置环境状态，会清空历史和list等
````

`reload`

````
重启 & 重置环境状态，但不会清空历史和list等
````

`open <file>`

````
打开外部的expression 文件
````

## 常用导入测试

参考 https://docs.oracle.com/javase/9/

````java
import java.util.concurrent.ExecutorService;
````

## 运行文件