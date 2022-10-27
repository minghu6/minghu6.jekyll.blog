---
title: Mongo Repository 相关
date: 2018-02-01
layout: post
mathjax: true
category:
- oth
---
# mongo repository spring

findAll 批连查取只能
自定义在接口无效

key 字段缺失则无法过滤

## Model 的某个字段是某个(Model | Vo | id)的collection

* Model 完全的附属关系，没办法独立统计信息
* Vo 完全不会更改信息,除了添加　(比如权限)
* id 最常见，可以更改信息的

## 批量添加一个字段

````js
// 更新全部, 如果没有，就添加这个字段
db.stay.updateMany({}, {$set: {"roomType": "At one wellness hotel"}}

// #unset 是删除字段
````