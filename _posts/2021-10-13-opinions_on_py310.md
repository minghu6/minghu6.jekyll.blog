---
title: 吐嘈Python3.10
date: 2021-10-13
layout: post
mathjax: true
category:
- lang
---
Python这一版更新引入了对于舒服的函数式编程至关重要的结构化的模式匹配, 我把它看作是Python面对逐渐尴尬境地的一种自救。

实际上自从Rust持续发展以来, 从功能上好像是C++会受到冲击, 但由于Rust本身项目管理的科学性和工具链与文档的完善性, 和整个生态系统的蓬勃发展, 对于用户体验来讲, 我现在哪怕是脚本型的任务都会使用Rust而不是Python, Rust更甜,但是更有表现力(比如有我最喜欢的,也是生产力最高的模板宏的元编程方式), 而它的性能, 至少是C++级别的, 事实上从个人经验上看, 在一些基础性算法,比如求解后缀数组上, 其表现(O2优化)明显优于同等优化级别的C++。

扯远了, 回到py310,首先我很乐见这种改变, 但我必须吐嘈一点, 就是在[PEP635](https://www.python.org/dev/peps/pep-0635/)介绍这个想法的动机，里面讲:

*“(Structural) pattern matching syntax is found in many languages, from Haskell, Erlang and Scala to Elixir and Ruby.*

*(A proposal for JavaScript is also under consideration.)“*

这属于典型的搪塞型的理由, 因为这句话是对的, 可惜放在10年前也是对的, 其中Haskell, Erlang更是上个世纪发明的语言, 而模式匹配这种思想有比它们还要更早的历史, 那为什么十年前Python不想要加入这个特性, 而现在觉得想要加入呢? 是因为十年后Js也在考虑这个问题吗?

让我来揭开这层遮羞的薄布, 实际上就是因为最受欢迎的语言Rust把模式匹配带进了主流, 它证明了一门完全严肃的准工业级别的语言, 它居然可以比Python还要甜, 还要快, 还要强大, 还要更能利用已有的C/C++库, 这么一比,Python变得毫无优点, 这是真正的危机, 而这个PEP上还保持着某种身为老主流的傲慢, 它提了每一门与Rust有关系的语言(主要是Haskell和一点点儿Ruby), 它甚至都并列提了Erlang和它平台派生的Elixir两个, 而不是只用Erlang代表, 就仿佛在说, 按照举例逻辑, 我这里应该提一下Rust, 但是我的傲慢实在不允许……

最后一个结论, 这是个振奋人心的好消息, 但对于重振Python而言还远远不够, 但至少让它开始变得有了一点儿竞争力(在受用户欢迎的角度上).