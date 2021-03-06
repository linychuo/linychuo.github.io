---
title: "微服务架构"
date_time: 2020-04-06 14:38:32
tags: microservice
---

微服务是一种架构模式，这种架构是一组服务集合的应用程序，它有以下特征：
- 易维护和测试
- 松散耦合
- 独立部署
- 围绕业务功能组织
- 每一个服务都可以由一个小团队来维护

微服务架构可以让团队快速，频繁可信的交付大型且复杂的应用程序，各服务可独立使用不同的技术栈来开发部署应用程序

![Microservice_Architecture.png](/imgs/Microservice_Architecture.png)

每个服务间可以通过同步（HTTP/REST）或异步（AMQP）来通信，能够独立部署和开发，有着自己的数据库，通过Saga模式来解决多服务之前数据一致性的问题

微服务架构带来了很多的优势，但也同时带来了很多问题
- 开发者必须处理分布式系统带来的额外的复杂性，例如：服务间通讯问题，测试困难
- 部署也会变得复杂
- 内存消耗也在不断的递增
- 开发工具的限制