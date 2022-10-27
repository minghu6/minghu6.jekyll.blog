---
title: Common Lisp 编写工程文件
date: 2020-07-21
layout: post
mathjax: true
category:
- lang
---
## 参考

​	https://xach.livejournal.com/278047.html

​	https://lisp-lang.org/learn/writing-libraries

## 流程

### Step-0 从安装quicklisp开始

https://www.quicklisp.org/beta/#installation

### Step-1 使用脚手架初始化项目

````commonlisp
(ql:quickload "quickproject")
(quickproject:make-project <package-path>)
````

比如项目路径为: `"/mnt/d/Coding/CL/minghu6"`，默认使用目录名作为项目名，可以使用 `:name`定义一个别的名字

### Step-n 安装本地项目 （使用前面脚手架的不需要手动添加）

有几种方案：

1. 将项目软连接到 `~/.local/share/common-lisp/source/` ，这是ASDF的扫描目录之一

1. 将项目软连接到`~/quicklisp/local-projects/`，这是QuickLisp的扫描目录（本地项目优先于Amazon上的同名云端版）

1. 注册ASDF的扫描目录：
   
   在`~/.config/common-lisp/source-registry.conf` 上添加:
   
   ````commonlisp
   (:source-registry
     (:tree <your-dir-path>)
     :inherit-configuration)
   ````
   
   `<your-dir-path>`:
   
   ​	exp1:(`"/mnt/d/Coding/CL/"`)
   
   ​	exp2:`(:home "code")` `:home`指`$HOME`
   
   或者建立子目录级别的配置文件: `~/.config/common-lisp/source-registry.conf.d/projects.conf`，添加的内容：
   
   ````commonlisp
   (:tree <your-dir-path>)
   ````

​		最后运行 `(asdf:initialize-source-registry)`进行更新。

### Step-n 编写测试

参考https://turtleware.eu/posts/Tutorial-Working-with-FiveAM.html

## Step-n. (SBCL) 导出镜像文件：

````commonlisp
;;启动repl后加载程序环境
(load "php-syntax.lisp")
;;环境保存到镜像
(save-lisp-and-die "core-php-synax") 
````

加载镜像 `rlwrap sbcl --core core-php-synax`

### Step-n 发布到QuickLisp项目

*找了许久，没有找到发布项目到quicklisp的方法，因为潜意识里我一直觉得会有一个类似pypi的网站，专门用来管理包*

*结果万万没想到quicklisp采用的是人肉管理的方法。。。。。。*

参考\[https://www.cliki.net/Quicklisp%20tutorial\](https://www.cliki.net/Quicklisp tutorial)，到[issue列表](https://github.com/quicklisp/quicklisp-projects/issues)里发请求！！！条件很宽泛，只需要必要的在ASDF下可运行和免费的证书 (ex: Public Domain, MIT, BSD or similar) license （反过来可见一段时间社区的活跃程度）。

## FAQ:

1. 编译宏的时候遇到诸如 “It’s defined earlier in the file but not available in compile-time”
   
   参考https://stackoverflow.com/questions/10674650/eval-when-uses
   
   有两种解决方法：
   
   1. 把依赖函数放到另一个编译顺序在该宏之前的文件里，在.asd文件里有一个默认被脚手架添加的配置项`:serial t`
      
      就是设置按顺序编译项目文件，我们只需要在`:coponents`中将依赖函数的文件放在宏的上面即可
   
   1. 使用`eval-when`将函数编译进compile-time的环境中，详见上面stackover的帖子。
      
      ````
       (eval-when (:compile-toplevel :execute :load-toplevel)
         (defun bar (form) ...)
       )
      ````

## Notes:

1. 遇事不决restart-slime
1. 首次加载使用`ql:quickload`，重载使用`asdf:load_system`
1. sbcl 本地更好的repl 
   1. install rlwrap，`rlwrap sbcl`