---
title: Install php mods
date: 2018-02-09
layout: post
mathjax: true
category:
- PHP
---
## Ubuntu (16.04)

### install pecl && pear

`cd <php-executable-path>`
`curl -O http://pear.php.net/go-pear.phar`
`sudo php -d detect_unicode=0 go-pear.phar`

maybe need fix bug :
https://stackoverflow.com/questions/40999752/pear-error-xml-extension-not-found-on-ubuntu-14-04-after-installing-php-xml/43035546

`sudo sed -i "$ s|\-n||g" /usr/bin/pecl`

### install xdebug

1. Need php7.0-dev
   `sudo apt install php7.0-dev`

1. Install Xdebug by pear
   `pear install xdebug`