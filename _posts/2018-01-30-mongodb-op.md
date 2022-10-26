---
title: Mongo Repository 操作
date: 2018-01-30
layout: post
mathjax: true
category:
- MongoDB
---
## 批量添加一个字段

````js
// 更新全部, 如果没有，就添加这个字段
db.stay.updateMany({}, {$set: {"roomType": "At one wellness hotel"}}

// #unset 是删除字段
````