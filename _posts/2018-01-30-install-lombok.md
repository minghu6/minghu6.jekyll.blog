---
title: Install lombok
date: 2018-01-30
layout: post
mathjax: true
category:
- Java
- Eclipse
---
## IntelliJ IDEA

Just install the lombok plugin from plugin repository.

## Eclipse (or STS)

1. Dwonload the lombok jar file from [official site](https://projectlombok.org/downloads/lombok.jar) or from our chat group. And can also get it from local maven repository `~/.m2/repository/org/projectlombok/lombok/`

1. java -jar \<lombok-jar> , select the eclipse installation location in the popup window.

1. Make sure the follow line is in the `eclipse.ini` or `STS.ini`
   
   ````
    -javaagent:lombok.jar
    -Xbootclasspath/a:lombok.jar
   ````
   
   ps: */a : append class path*

1. elipse is buggy, maybe need exit and start (*Restart doesn’t take effect*) and `Maven -> Update Project`.