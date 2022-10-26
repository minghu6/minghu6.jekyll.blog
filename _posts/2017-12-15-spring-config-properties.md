---
title: Spring Config
date: 2017-12-15
layout: post
mathjax: true
category:
- Spring
---
# Spring

## Basic

````yml
spring:
    application:
        name: 应用名称
            exp:
                spring.application.name=client-dc

server:
    port: 启动的端口
        exp:
            server.port=8123
````

## Spring Cloud

````yml
spring:
    cloud:
        config:
            profile:  # 配置文件profile
                # exp:
                #     spring.cloud.config.profile=dev
                #     spring.cloud.config.profile=default (默认)
                #     spring.cloud.config.profile=staging
            label:  # 配置文件label
                # exp:
                #     spring.cloud.config.label=master (默认)
                #     spring.cloud.config.label=develop
            uri:  # 配置文件server地址
                # exp:
                #     spring.cloud.config.uri=http://localhost:7001/

management:
    security:
        enabled: true | false (default)  # 开启 spring-boot-starter-actuator 详细信息以及操作端点 /pause 等

endpoints:
  shutdown:
    enabled: true | false (default)  # 开启 /shutdown 端点
    sensitive: true (default) | false  # 开启 /shutdown 端点的验证保护, 与下面的security配合使用


security:
    user:
        name:  # 验证用户名
            # exp:
            #     security.user.name=admin
        password:  # 验证密码
            # exp:
            #     security.user.password=secret
        role:  # 角色
            # exp:
            #     management.security.role=SUPERUSER
````

## 统一设置版本:

### method 1, set `<parent>`

````xml
<parent>
    <groupId>org.springframework.boot</groupId>
    <artifactId>spring-boot-starter-parent</artifactId>
    <version>1.4.2.RELEASE</version>
</parent>

<dependencies>
    <dependency>
      <groupId>org.springframework.boot</groupId>
      <artifactId>spring-boot-starter-data-elasticsearch</artifactId>
    </dependency>
    ...
</dependencies>
````

### method 2 set `<property>`

````xml
<properties>
    <springboot.version>1.5.3.RELEASE</springboot.version>
</properties>

<dependencies>
    <dependency>
        <groupId>org.springframework.boot</groupId>
        <artifactId>spring-boot-starter-data-elasticsearch</artifactId>
        <version>${springboot.version}</version>
    </dependency>
    ...
</dependencies>
````

**要注意区分springframework, springframework.boot, 以及原生的java package**
**特别是springframework和springframework.boot，分别有两套不同的版本**
**就比如spring-data-elasticsearch, spring-boot-starter-data-elasticsearch**

## 设置scope [转载](http://peak.iteye.com/blog/299225)

maven依赖关系中Scope的作用 

Dependency Scope 

在POM 4中，<dependency>中还引入了<scope>，它主要管理依赖的部署。目前<scope>可以使用5个值：

* compile，缺省值，适用于所有阶段，会随着项目一起发布。 
* provided，类似compile，期望JDK、容器或使用者会提供这个依赖。如servlet.jar。 
* runtime，只在运行时使用，如JDBC驱动，适用运行和测试阶段。 
* test，只在测试时使用，用于编译和运行测试代码。不会随项目发布。 
* system，类似provided，需要显式提供包含依赖的jar，Maven不会在Repository中查找它。

## elastricsearch

````xml
<dependency>
    <groupId>net.java.dev.jna</groupId>
    <artifactId>jna</artifactId>
    <version>4.1.0</version>
</dependency>
````

## spring boot using mongo

````xml
<dependency>
    <groupId>org.springframework.boot</groupId>
    <artifactId>spring-boot-starter-data-mongodb</artifactId>
</dependency>
````

`spring-boot-starter-data-mongodb` 与 `spring-data-jpa` 不要同时配置，否则无法使用自动配置．