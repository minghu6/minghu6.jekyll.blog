---
title: Compile Linux Kernel
date: 2022-08-26
layout: post
mathjax: true
category:
- os
---
编译实践1：
学习Linux内核模块开发，需要增加一个`FORCE_UNLOADING` feature 以避免安装的模块崩溃时可以强行卸载而不必重启。而好像只有OpenSuSE可以通过修改配置文件来动态修改内核配置，而其他发行版只能重新编译内核。

[**Linux上游源代码（git地址在其页面下方）**](https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux.git)

**默认环境**
当前目录： kernel源目录

## Ubuntu

注意源文件的下载，不要在系统目录比如`/usr/src`下进行，而要在用户空间内进行，而`apt source`，`apt-get source`, 不需要也不能使用`sudo`, 会错误得造成源文件的用户组变为root。对于>= 20.04,没有官方更新的wiki讲如何使用debian/rules，直接从linux上游源代码处获取。

= 18.04

参考[这篇文章](https://askubuntu.com/questions/1085411/unable-to-follow-kernel-buildyourownkernel)

= 19.04

参考[官方wiki](https://wiki.ubuntu.com/Kernel/BuildYourOwnKernel)

= 20.04 (or maybe upper than it)

参考[这篇文章](https://discourse.ubuntu.com/t/how-to-compile-kernel-in-ubuntu-20-04/20268/10)

1. 下载源代码

1. 安装依赖`sudo apt install asciidoc autoconf bc binutils-dev bison build-essential crash dkms fakeroot flex gawk gcc kernel-wedge kexec-tools libelf-dev libiberty-dev libncurses5-dev libncurses-dev libssl-dev libudev-dev makedumpfile openssl pciutils-dev`
   
   (`pciutils-dev` is replaced by `libpci-dev` in the later version)

1. 配置参考下面tips

1. `make -jn deb-pkg`

1. `cd .. && sudo apt ./linux-*.deb`

1. `sudo update-grub`

**Tips:**

1. Linux 68%的代码是驱动相关代码，当然大部分的编译时间也是在编译驱动上，而这一部分也是容易出问题，往往花了几个小时编译发现，后来发现在这里出了错误，这就很难受，需要提前仔细配置。个人经验是不使用 `` /boot/config-`uname -r`  `` 作为初始配置，而是使用`make localmodconfig` 创建初始配置:`.config`文件，使用默认的配置，确保驱动的编译不出问题。然后使用`make menuconfig`(shell text) or `make xconfig`(qt5 gui) 配置内核其他部分。

1. make 使用`-j`启用多核编译很重要，如果宿主机器在编译的同时需要运行其他任务，建议少用一个核心，避免机器down掉(然后比如当前用户直接被强制logout)。

1. 如果make过程中因为什么原因中途停掉了，可以再次启动，但是第一次会报错: `"dpkg-source: unrepresentable changes to source"`, 只要把相关报错文件删除掉重新运行即可。

1. \>= 20.04, 可能需要的额外依赖包:
   
   dwarves # tmp_vmlinux.btf: pahole (pahole) is not available

## Select Kernel

重启的时候按住`shift`进入grup界面，选择需要的版本内核启动。
修改默认启动内核，参考[华为云的这篇文章](https://support.huaweicloud.com/intl/en-us/trouble-ecs/ecs_trouble_0327.html)和[askubuntu的这个回答](https://askubuntu.com/questions/82140/how-can-i-boot-with-an-older-kernel-version/1393019#1393019)
修改配置文件`/etc/default/grub`的 `GRUB_DEFAULT` 的值为启动项的序号：

````
0，正常启动
1>y， 高级启动项，y start from 0
````

查看高级启动项的序号(Ubuntu为例)：
`sudo grub-mkconfig | grep -iE "menuentry 'Ubuntu, with Linux" | awk '{print i++ " : "$1, $2, $3, $4, $5, $6, $7}'`
不要忘记最后运行`sudo update-grup`来更新配置。