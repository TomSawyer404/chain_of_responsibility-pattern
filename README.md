# Chain of Responsibility 模式

在责任链模式（Chain of Responsibility Pattern）中，包含了一些命令对象和一系列的处理对象。每一个处理对象决定它能处理哪些命令对象，它也知道如何将它不能处理的命令对象传递给该链中的下一个处理对象。该模式还描述了往该处理链的末尾添加新的处理对象的方法。

责任链模式也可以这么理解，它是很多对象由每一个对象对其下家的引用而连接起来形成的一条链。请求在这个链上传递，直到链上的某一个对象决定处理此请求。发出这个请求的客户端并不知道链上的哪一个对象最终处理这个请求，这使得系统可以在不影响客户端的情况下动态地重新组织和分配责任。

![Observer](https://refactoring.guru/images/patterns/content/chain-of-responsibility/chain-of-responsibility.png?id=56c10d0dc712546cc283)



责任链模式的结构图如下：

![Structure of Chain of Responsibility](https://refactoringguru.cn/images/patterns/diagrams/chain-of-responsibility/structure-indexed.png?id=e13a5bf44f9ca4729922)

它有以下几个角色：

- Handler：声明了所有具体处理者的通用接口。该接口通常仅包含单个方法用于处理请求，但有时其还会包含一个设置链上下处理者的方法。
- Base Handler：可选，我们可以将所有处理者共用的样本代码放置其中。
- Concrete Handlers：包含处理请求的实际代码。每个处理者接收到请求后，都必须决定是否处理，以及是否沿着链传递请求。

# 实战

我们来看一个应用情景。在医院中会有很多部门，如：

- 前台
- 医生
- 药房
- 收银

病人来访时，他们会先去前台，然后是看医生，取药，最后结帐。也就是说，病人需要通过一条部门链，每个部门都在完成其职能后将病人进一步沿着链条输送。

> 我的代码有一个问题， 责任链最多只有2层。到了药房那一层就出现了变量在同一时间被两次可变借用，造成内存安全问题。我暂时没想明白怎么处理这个情况。

# 参考资料

- [责任链模式-Wikipedia](https://zh.wikipedia.org/zh-cn/%E8%B4%A3%E4%BB%BB%E9%93%BE%E6%A8%A1%E5%BC%8F)
- [lpxxn的示例代码](https://github.com/lpxxn/rust-design-pattern/blob/master/behavioral/chain_of_responsibility.rs)
- [yukihir0的示例代码](https://github.com/yukihir0/rust_design_pattern/blob/master/chain_of_responsibility/src/main.rs)
- [Chain of Responsibility](https://refactoring.guru/design-patterns/chain-of-responsibility)
- [责任链模式 - 地鼠文档](https://www.topgoer.cn/docs/golang-design-pattern/ChainofResponsibility)

---
