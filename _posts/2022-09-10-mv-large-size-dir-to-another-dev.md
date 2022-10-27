---
title: Linux上把大文件夹移动到其他存储设备上来释放空间
date: 2022-09-10
layout: post
mathjax: true
category:
- os
---
## 问题背景

一个常见的问题是发现某个文件夹比如`/var`, `/opt` 过大，使得所在系统磁盘（比如一块儿容量很稀缺的高速SSD）的空间非常紧张，这时我们想把大文件夹移动到其他存储设备上来释放空间。

## 约定

不妨假设根系统`/`挂载在磁盘分区`sda4`，要移动的大目录LargeDir是根文件系统下的一个子目录，目标存储设备为`sdb`。

## 思路1[^1]

一个体面的思路是在`sdb`上创建一个分区，假设为`sdb1`，创建LargeDir的同级根目录LargeDir2，把`sdb1`挂载到LargeDir2，把LargeDir的内容复制到LargeDir2

````bash
mkdir LargeDir2
mount /dev/sdb1 LargeDir2
rsync -a LargeDir/ LargeDir2
````

在文件系统表单`/etc/fstab`里添加对应项,比如
`/dev/sdb1    LargeDir    ext4    defaults      2 2`

重启后生效，但是这个思路是后一个文件系统隐藏了前一个文件系统的子目录，**如何删除原文件系统的子目录内容呢？**

由于**Linux允许同一文件系统有多个挂载点**，所以只需要把`sda4`再次挂载到一个新的目录，然后就可以找到原LargeDir，然后删除。

[^1]: https://askubuntu.com/questions/39536/how-can-i-store-var-on-a-separate-partition

## 思路2[^2]

重启进入单用户模式，安全移动内容，然后删除，然后重新命名回来

[^2]: https://www.suse.com/support/kb/doc/?id=000018399