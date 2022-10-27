---
title: C++ ABI 笔记 1
date: 2022-03-27
layout: post
mathjax: true
category:
- oth
---
Ref: [https://itanium-cxx-abi.github.io/cxx-abi/abi.html](1)

## Member Pointer

1. 数据成员指针类型的基本ABI属性是 `ptrdiff_t`, 表示数据成员到所属基类的字节偏移量.
   
   空数据成员指针表示为`-1` (但是通过显式派生到基类的转换,生成偏移量为`-1`的数据成员指针)

1. 一个非空的数据成员指针的基到派生和派生到基的转换, 可以通过分别加或减掉二者静态偏移量(C++标准保证可知这个静态偏移量)实现

1. 建议更好的空数据指针实现: (x \<\< 1) + 1 表示非空数据

### Member Function Pointers

````cpp
struct {
  fnptr_t ptr;
  ptrdiff_t adj;
};
````

1. `fnptr_t` : 1 + offset(function’s vtable entry offset in bytes)
   
   null 表示空的函数指针

调用过程:

````
adj_this = `this` + `adj`

if ptr != null { 从vtable中加载对应项 } else { 调用存储的函数指针 }
````

## Virtual Table Layout

Virtual call (vcall) offsets are used to perform pointer adjustment for virtual functions that are declared in a virtual base class or its subobjects and overridden in a class derived from it.

These entries are allocated in the virtual table for the virtual base class that is most immediately derived from the base class containing the overridden virtual function declaration.

They are used to find the necessary adjustment from the virtual base to the derived class containing the overrider, if any.

When a virtual function is invoked via a virtual base, but has been overridden in a derived class,

the overriding function first adds a fixed offset to adjust the this pointer to the virtual base,

and then adds the value contained at the vcall offset in the virtual base to its this pointer to get the address of the derived object where the function was overridden.

These values may be positive or negative.

Virtual Base (vbase) offsets are used to access the virtual bases of an object.

Such an entry is added to the derived class object address (i.e. the address of its virtual table pointer) to get the address of a virtual base class subobject.

Such an entry is required for each virtual base class. The values can be positive or negative.

However, in classes sharing a virtual table with a primary base class,

the vcall and vbase offsets added by the derived class all come before the vcall and vbase offsets required by the base class,

so that the latter may be laid out as required by the base class without regard to additions from the derived class(es).

The offset to top holds the displacement to the top of the object from the location within the object of the virtual table pointer that addresses this virtual table, as a  ptrdiff_t.

It is always present. The offset provides a way to find the top of the object from any base subobject with a virtual table pointer. This is necessary for dynamic_cast\<void\*\> in particular.

<b>NOTE</b>: In a complete object virtual table, and therefore in all of its primary base virtual tables, the value of this offset will be zero. For the secondary virtual tables of other non-virtual bases, and of many virtual bases, it will be negative. Only in some construction virtual tables will some virtual base virtual tables have positive offsets, due to a different ordering of the virtual bases in the full object than in the subobject’s standalone layout.
The typeinfo pointer points to the typeinfo object used for RTTI. It is always present. All entries in each of the virtual tables for a given class must point to the same typeinfo object. A correct implementation of typeinfo equality is to check pointer equality, except for pointers (directly or indirectly) to incomplete types. The typeinfo pointer is a valid pointer for polymorphic classes, i.e. those with virtual functions, and is zero for non-polymorphic classes.
The virtual table address point points here, i.e. this is the virtual table address contained in an object’s virtual pointer. This address must have the alignment required for pointers.
Virtual function pointers are used for virtual function dispatch. Each pointer holds either the address of a virtual function of the class, or the address of a secondary entry point that performs certain adjustments before transferring control to a virtual function.
The form of a virtual function pointer is specified by the processor-specific C++ ABI for the implementation. In the specific case of 64-bit Itanium shared library builds, a virtual function pointer entry contains a pair of components (each 64 bits): the value of the target GP value and the actual function address. That is, rather than being a normal function pointer, which points to such a two-component descriptor, a virtual function pointer entry is the descriptor.

The order of the virtual function pointers in a virtual table is the order of declaration of the corresponding member functions in the class. If an implicitly-declared copy assignment operator, move assignment operator, or destructor is virtual, it is treated as if it were declared at the end of the class, in that order. (Implicitly-declared assignment operators may be virtual if a base class declares a virtual assignment operator taking a reference to a derived class type.)

An entry is added for every virtual function in a class, including deleted functions, unless:

the function is consteval or
the function overrides a function from the primary base and that override does not require a return-type adjustment.
An override requires a return-type adjustment if the return types are different and have potentially incompatible representations. C++ permits an override to differ in return type from the overridden function only if both types are pointer-to-class or reference-to-class types and the class type B in the overridden function is an unambiguous base class of the class type D in the override. For the purposes of vtable layout, these types are considered to have potentially incompatible representations if:

B is a morally virtual base of D (even if D is final and the offset of B within D is known to be zero) or
the (static) offset of B within D is non-zero.
When a derived class and its primary base share a virtual table, the virtual function entries introduced by the derived class follow those for the primary base, so that the layout of the primary base’s embedded virtual table is the same as that of its standalone virtual table. In particular, if the derived class overrides a base class virtual function with a different (covariant) return type, the entry for the derived class comes after the primary base’s embedded virtual table in declaration order, and is the entry used for calls from the derived class without adjustment. The entry in the embedded primary virtual table points to a routine that adjusts the result pointer before returning.

The entries for virtual destructors are actually pairs of entries. The first destructor, called the complete object destructor, performs the destruction without calling delete() on the object. The second destructor, called the deleting destructor, calls delete() after destroying the object. Both destroy any virtual bases; a separate, non-virtual function, called the base object destructor, performs destruction of the object but not its virtual base subobjects, and does not call delete().

Following the primary virtual table of a derived class are secondary virtual tables for each of its proper base classes, except any primary base(s) with which it shares its primary virtual table. These are copies of the virtual tables for the respective base classes (copies in the sense that they have the same layout, though the fields may have different values). We call the collection consisting of a primary virtual table along with all of its secondary virtual tables a virtual table group. The order in which they occur is the same as the order in which the base class subobjects are considered for allocation in the derived object:

First are the virtual tables of direct non-primary, non-virtual proper bases, in the order declared, including their secondary virtual tables for non-virtual bases in the order they appear in the standalone virtual table group for the base. (Thus the effect is that these virtual tables occur in inheritance graph order, excluding primary bases and virtual bases.)
Then come the virtual base virtual tables, also in inheritance graph order, and again excluding primary bases (which share virtual tables with the classes for which they are primary).
This ABI does not make guarantees about the layout of other virtual tables in a virtual table group relative to a virtual table pointer in an object or a VTT. It guarantees only the layout of the global symbol for that virtual table group. It does not guarantee that the virtual table pointers actually installed in an object or a VTT will point into that global symbol.