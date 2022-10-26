---
title: Regex Syntax
date: 2017-12-14
layout: post
mathjax: true
category:
- Regex
---
Reference http://blog.didispace.com/regular-expression-2/

`?=pattern`:positive lookahead　  pattern 前面的位置

````
exp:
    re.sub('(?=l)', '#', 'hello') => 'he#l#lo'
````

`?!pattern`:negative lookahead 除去`?=`匹配以外的位置

````
exp:
    re.sub('(?！l)', '#', 'hello')　=>　'#h#ell#o#'
usage:
    re.sub("(?=(\d{3})+$)", ",", "123456789") => ',123,456,789'
    re.sub("(?!^)(?=(\d{3})+$)", ",", "123456789") => '123,456,789'  # 前面不能是空位置, 类似有 \B (\b 以外的位置)
````

`?<=pattern`: pattern 后面的位置

````
exp:
    re.sub('(?<=l)', '#', 'hello') => 'hel#l#o'
````

`?<!pattern`: pattern 后面以外的位置

````
exp:
    re.sub('(?<!l)', '#', 'hello') =>  '#h#e#llo#'
````

Refrence: [http://blog.didispace.com/regular-expression-6/](http://blog.didispace.com/regular-expression-6/)

lazy match: `?`
exp:
re.match(‘\\d+’, ‘1234’) => ‘1234’
re.match(‘\\d+?’, ‘1234’) => ‘1’

uncached group: `?:`
exp:
re.match(‘(ab)+’, ‘abab’) => re.match(‘(?:ab)+’, ‘abba’)