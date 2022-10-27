---
title: Zero Cost Exception Mechanism
date: 2022-03-30
layout: post
mathjax: true
category:
- lang
---
## Related Header Definition

依赖于 *unwinding library* 提供的至少如下的接口:

````txt
  _Unwind_RaiseException,
  _Unwind_Resume,
  _Unwind_DeleteException,
  _Unwind_GetGR,
  _Unwind_SetGR,
  _Unwind_GetIP,
  _Unwind_SetIP,
  _Unwind_GetRegionStart,
  _Unwind_GetLanguageSpecificData,
  _Unwind_ForcedUnwind
````

### Personality Routine

语言特定的函数(以下也称为routine, 名字用`__personality_routine`指代), 用于和**unwinding library** 配合做语言特定的异常处理

````cpp
_Unwind_Reason_Code (*__personality_routine)
  (int version,
    _Unwind_Action actions,
    uint64 exceptionClass,
    struct _Unwind_Exception *exceptionObject,
    struct _Unwind_Context *context);
````

* 返回值类型:

**\_Unwind_Reason_Code**:

````c
typedef enum {
    _URC_NO_REASON = 0,
    _URC_FOREIGN_EXCEPTION_CAUGHT = 1,
    _URC_FATAL_PHASE2_ERROR = 2,
    _URC_FATAL_PHASE1_ERROR = 3,
    _URC_NORMAL_STOP = 4,
    _URC_END_OF_STACK = 5,
    _URC_HANDLER_FOUND = 6,
    _URC_INSTALL_CONTEXT = 7,
    _URC_CONTINUE_UNWIND = 8
} _Unwind_Reason_Code;
````

* 参数1 `version`: 个性例程假设的 unwinding runtime 的版本号, 比如 1

* 参数2 `actions`: Personality Routine Actions:

**\_Unwind_Action**:

````c
  typedef int _Unwind_Action;
  static const _Unwind_Action _UA_SEARCH_PHASE = 1;  // 0b0001
  static const _Unwind_Action _UA_CLEANUP_PHASE = 2; // 0b0010
  static const _Unwind_Action _UA_HANDLER_FRAME = 4; // 0b0100
  static const _Unwind_Action _UA_FORCE_UNWIND = 8;  // 0b1000
````

action flag可以在不违反语义情况下(比如搜索和清除阶段不能被同时设置)叠加

* 参数3 `exception class`:
  
  By convention, the high 4 bytes indicate the vendor (for instance `HP\0\0`), and the low 4 bytes indicate the language. (for instance `C++\0`)

* 参数4 `exceptionObject`: `_Unwind_Exception`的指针类型

**\_Unwind_Exception**

````c
struct _Unwind_Exception {
    uint64    exception_class;  // 同上exception_class
    _Unwind_Exception_Cleanup_Fn exception_cleanup;
    uint64    private_1;
    uint64    private_2;
};
````

**\_Unwind_Exception_Cleanup_Fn**:

````c
typedef void (*_Unwind_Exception_Cleanup_Fn)
    (_Unwind_Reason_Code reason,
     struct _Unwind_Exception *exc);
````

`exception_cleanup` routine一定会在异常对象被不同运行时销毁时被调用, 比如Java异常被C++捕获.

这种情况下会返回reason code来表明异常对象被删除的原因:

1. `_URC_FOREIGN_EXCEPTION_CAUGHT`: 这表明不同的运行时捕获了异常, 嵌套的外部异常或者重抛外部异常会导致UB (undefined behaviour)
1. `_URC_FATAL_PHASE1_ERROR`: 个性例程在 *Phase-1* 遇到了未被特定错误码定义的错误
1. `_URC_FATAL_PHASE2_ERROR`: 个性例程在 *Phase-2* 遇到了错误,比如栈损坏

* 参数5 `context`: `_Unwind_Context`的指针

**\_Unwind_Context**

````c
struct _Unwind_Context
````

由 *unwinder library* 定义的不透明结构.  由其创建和销毁, 在unwinding的时候传给个性例程.

## The Stack Unwind Process

(begins with the raising of an exception)

**两阶段处理**:

*Phase-1*:  search,  action is set `_UA_SEARCH_PHASE`.

从当前PC和(其他)寄存器状态开始, 逐帧展开(函数不断向上退出), 直到个性例程报告成功(在某一帧找到handler) 然后进入`Phase-2`

或者失败(所有帧中找不到handler) 调用`terminate()`

*Phase-2*:  cleanup, action is set `_UA_CLEANUP_PHASE`.

框架重启,再次重复调用个性例程,找到被标志的帧, 然后把控制权转给landing pad代码 (`goto label xxx`)

两阶段处理的提供了一些好处, 比如可以在*Phase-1* dismiss 异常, 这允许通过修复异常的情况, 从而实现可恢复性的异常的处理.

对于一个异常只要多次抛出(by re-throwing), 就可以多次执行两阶段.

**\_Unwind_Action 解释**

1. `_UA_SEARCH_PHASE`:
   成功返回 `_URC_HANDLER_FOUND`, 失败返回 `_URC_CONTINUE_UNWIND` (上文所示, 个性例程返回值是`_Unwind_Reason_Code`).

1. `_UA_CLEANUP_PHASE`:
   个性例程可以通过调用嵌套过程来自己执行清理,然后返回`_URC_CONTINUE_UNWIND`;
   或者准备寄存器环境把控制转移给“landing pad“, 然后返回`_URC_INSTALL_CONTEXT`.

1. `_UA_HANDLER_FRAME`
   在 *Phase 2*, 表明当前帧就是要找的有被标记的handler的帧. The personality routine is not allowed to change its mind between phase 1 and phase 2, i.e. it must handle the exception in this frame in phase 2.

1. `_UA_FORCE_UNWIND`
   在 *Phase 2*, 表明, 表示异常不允许被捕获. This flag is set while unwinding the stack for `longjmp` or during thread cancellation. User-defined code in a catch clause may still be executed, but the catch clause must resume unwinding with a call to \_Unwind_Resume when finished.

转移控制权给landing pad,返回`_URC_INSTALL_CONTEXT`, 在这之前 **unwind library** 使用上下文管理例程和上下文记录`_Unwind_Context`来恢复寄存器环境.

### 上下文管理例程

**\_Unwind_GetGR**

````c
uint64 _Unwind_GetGR (struct _Unwind_Context *context, int index);
````

函数返回给定寄存器的64bit的值 (64位的系统). 寄存器按照它的索引编号进行标识.

对于Itanium实现:

0-31 是固定寄存器, 32-127 是栈寄存器. During the two phases of unwinding, only GR1 has a guaranteed value, which is the Global Pointer (GP) of the frame referenced by the unwind context. If the register has its NAT bit set, the behaviour is unspecified.

**\_Unwind_SetGR**

````c
void _Unwind_SetGR (struct _Unwind_Context *context, int index, uint64 new_value);
````

This function sets the 64-bit value of the given register, identified by its index as for `_Unwind_GetGR`. The NAT bit of the given register is reset.

The behaviour is guaranteed only if the function is called during phase 2 of unwinding, and applied to an unwind context representing a handler frame, for which the personality routine will return `_URC_INSTALL_CONTEXT`. In that case, only registers GR15, GR16, GR17, GR18 should be used. These scratch registers are reserved for passing arguments between the personality routine and the landing pads.

**\_Unwind_GetIP**

````c
uint64 _Unwind_GetIP (struct _Unwind_Context *context);
````

This function returns the 64-bit value of the instruction pointer (IP) 也就是PC.

During unwinding, the value is guaranteed to be the address of the bundle immediately following the call site in the function identified by the unwind context. This value may be outside of the procedure fragment for a function call that is known to not return (such as `_Unwind_Resume`).

**\_Unwind_SetIP**

````c
void _Unwind_SetIP (struct _Unwind_Context *context, uint64 new_value);
````

This function sets the value of the instruction pointer (IP) for the routine identified by the unwind context.

The behaviour is guaranteed only when this function is called for an unwind context representing a handler frame, for which the personality routine will return `_URC_INSTALL_CONTEXT`. In this case, control will be transferred to the given address, which should be the address of a landing pad.

**\_Unwind_GetLanguageSpecificData**

````c
uint64 _Unwind_GetLanguageSpecificData (struct _Unwind_Context *context);
````

This routine returns the address of the language-specific data area for the current stack frame.

![<b>NOTE</b>:](/assets/img/warning.gif) *This routine is not stricly required: it could be accessed through `_Unwind_GetIP` using the documented format of the `UnwindInfoBlock`, but since this work has been done for finding the personality routine in the first place, it makes sense to cache the result in the context. We could also pass it as an argument to the personality routine.*

**\_Unwind_GetRegionStart**

````c
uint64 _Unwind_GetRegionStart (struct _Unwind_Context *context);
````

This routine returns the address of the beginning of the procedure or code fragment described by the current unwind descriptor block.

This information is required to access any data stored relative to the beginning of the procedure fragment. For instance, a call site table might be stored relative to the beginning of the procedure fragment that contains the calls. During unwinding, the function returns the start of the procedure fragment containing the call site in the current stack frame.

### 抛出异常

**\_Unwind_RaiseException**

````c
_Unwind_Reason_Code _Unwind_RaiseException (struct _Unwind_Exception *exception_object );
````

`_Unwind_RaiseException` 实际并不返回, 除非发生了错误的情况 (such as no handler for the exception, bad stack format, etc.). 可能的返回值:

1. `_URC_END_OF_STACK`
   The unwinder encountered the end of the stack during phase 1, without finding a handler.
1. `_URC_FATAL_PHASE1_ERROR`: The unwinder encountered an unexpected error during phase 1, e.g. stack corruption.

*![<b>NOTE</b>:](/assets/img/warning.gif) The unwind runtime will likely have modified the stack (e.g. popped frames from it) or register context, or landing pad code may have corrupted them. As a result, the the caller of `_Unwind_RaiseException` can make no assumptions about the state of its stack or registers.*

**\_Unwind_ForcedUnwind**

````c
_Unwind_Reason_Code _Unwind_ForcedUnwind (
    struct _Unwind_Exception *exception_object,
    _Unwind_Stop_Fn stop,
    void *stop_parameter );
````

* 参数1 `stop`: `_Unwind_Stop_Fn` 特定函数指针类型

````c
typedef _Unwind_Reason_Code (*_Unwind_Stop_Fn) (
    int version,
    _Unwind_Action actions,
    uint64 exceptionClass,
    struct _Unwind_Exception *exceptionObject,
    struct _Unwind_Context *context,
    void *stop_parameter );
````

Forced unwinding 是 *Phase-2* 中的过程. 对每一个展开帧, 都调用`stop` 函数 and 加上一个额外的`stop parameter`.

如果`stop` 函数标识了目标帧, 它就会把控制权转给landing pad, 而不是返回(通常是在调用了 `_Unwind_DeleteException`之后).

反之, 没有找到目标帧时就会返回:

1. `_URC_NO_REASON`:
   这不是目标帧, unwind运行时会再次调用个性例程, 使用`_UA_FORCE_UNWIND` and `_UA_CLEANUP_PHASE` 的action参数, 展开下一帧,并再次调用`stop`例程

1. `_URC_END_OF_STACK`:
   In order to allow `_Unwind_ForcedUnwind`to perform special processing when it reaches the end of the stack, the unwind runtime will call it after the last frame is rejected, with a NULL stack pointer in the context, and the `stop` function must catch this condition (i.e. by noticing the NULL stack pointer). **It may return this reason code if it cannot handle end-of-stack.**

1. `_URC_FATAL_PHASE2_ERROR`: 这个`stop` 函数应该在其他致命错误的情况下返回, e.g. stack corruption.

如果`stop`函数返回了任何`_URC_NO_REASON`意外的reason code, 从 `_Unwind_ForcedUnwind`的调用者的角度讲, 栈的状态是不确定的. 因此, unwind library 应该返回 `_URC_FATAL_PHASE2_ERROR` 给它的调用者.

![<b>NOTE</b>:](/assets/img/warning.gif) *Example: `longjmp_unwind()`*

*期望的`longjmp_unwind()`的实现是这样的. `setjmp()` 保存了状态后 (包括帧的指针).  `longjmp_unwind()`将会调用`_Unwind_ForcedUnwind`,用context里记录的帧地址和当前保存的帧地址进行比较. 如果相等就调用 `setjmp()` 进行恢复, 否则返回 `_URC_NO_REASON` 或者 `_URC_END_OF_STACK`.*

![<b>NOTE</b>:](/assets/img/warning.gif) *如果未来对 两阶段的foced unwinding 有新的需求, 可以定义另外的例程和新的`actions` 参数类型来进行支持*

**\_Unwind_Resume**

````c
void _Unwind_Resume (struct _Unwind_Exception *exception_object);
````

恢复异常的传播 e.g. 在部分展开的栈中执行清理代码(clean-up code)后如果不能恢复程序的正常执行, 就会恢复该异常的传播. 具体地就是在执行清理任务的landing pad结尾调用它(`_Unwind_Resume`).

![<b>NOTE 1</b>:](/assets/img/warning.gif)*`_Unwind_Resume`不能用来实现重抛(re-throwing). 这是一个两阶段模型, 之前的unwind session会被关闭. 重抛需要使用`_Unwind_RaiseException`.*

![<b>NOTE 2</b>:](/assets/img/warning.gif) This is the only routine in the unwind library which is expected to be called directly by generated code: it will be called at the end of a landing pad in a “landing-pad” model.

### 异常对象的管理

**\_Unwind_DeleteException**

````c
void _Unwind_DeleteException (struct _Unwind_Exception *exception_object);
````

删除给定的异常对象. 当程序捕获了外部异常后仍可以恢复正常运行时, 由于并不清楚如何删除这个外部的异常对象, 这时就需要调用这个例程.

它实际上是个方便函数, 会调用异常对象头(header)里面的`exception_cleanup`字段所带的`_Unwind_Exception_Cleanup_Fn`类型的函数指针

## 互操作的约定规则

对于C++, 在forced unwinding的时候, 一个 catch-all 块也会执行. 比如, a longjmp may execute code in a catch(…) during stack unwinding. However, if this happens, unwinding will proceed at the end of the catch-all block, whether or not there is an explicit rethrow.

## Reference

1. <https://itanium-cxx-abi.github.io/cxx-abi/abi-eh.html>