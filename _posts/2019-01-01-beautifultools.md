---
title: 易用工具
date: 2019-01-01
layout: post
mathjax: true
category:
- Linux
---
## sh tool

`fish`
`sudo apt install fish`
直接在bash中运行 `fish`

## 查看当前进程

htop

## 查看ｉo

iotop

## 合并目录

AUFS
inpired by http://coolshell.cn/articles/17061.html
`sudo mount -t aufs -o dirs=~/Documents/tmp=rw:./tmp-dir=rw notify ./tmp-dir`

## 视频下载工具

### youtube-dl

porn site 专用:
`youtube-dl --proxy "socks5://127.0.0.1:1080/" <url>`