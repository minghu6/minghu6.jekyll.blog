---
title: Postgress sync to ElastricSearch
date: 2018-01-04
layout: post
mathjax: true
category:
- Postgress
- ElastricSearch
---
http://www.jianshu.com/p/629f698a7c58

https://yq.aliyun.com/articles/56824?do=login&accounttraceid=b7545736-e469-47c3-b880-16e82a7b9378

http://www.infoq.com/cn/news/2015/01/postgresql-elasticsearch

增量同步策略:
１．外部表，触发器
２．定时批量同步（比如一个小时内的数据，用一定是数据冗余解决问题）

全量同步策略：