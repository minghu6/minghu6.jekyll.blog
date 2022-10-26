---
title: kernel crash dump
date: 2022-09-01
layout: post
mathjax: true
category:
- Kernel
---
## Ubuntu

[install linux-crashdump](https://ubuntu.com/server/docs/kernel-crash-dump)

## Dump Output

`alias crash="crash /usr/lib/debug/boot/vmlinux-$(uname -r)"`

`crash /var/crash/<yyyymmmmhhmm>/dump.<yyyymmmmhhmm>`

bt: backtrace
log: kernel log