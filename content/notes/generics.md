+++
title = "Generics"
date = 2024-11-08
+++
# Generics

Think of generics as a set of universal tools designed to handle various tasks efficiently. 

**In programming**, before generics existed, every function was like a tool designed for one specific job.

Want to sort numbers? *Write a special function*.
Need to sort words? *Write another*.

This repetition was **inefficient**. Generics emerged to simplify things. Instead of rewriting code for every data type, developers thought: 
> What if we could design functions and data structures that adapt to any type of data?

This led to the development of different techniques, each reflecting the needs and limitations of its time.

One of the earliest techniques was **templates**, introduced in C++. 

## Templates

***Templates** let you write a function or class once, and the compiler would generate a version for each data type used.*

This process, called **monomorphization**, ensures optimized code for every type. It’s *efficient but error-prone*.

As programming needs evolved, developers sought more flexibility.

## Type erasure

Languages like Java introduced **type erasure**, which *keeps generics during compilation but removes type information at runtime.*

Think of it as writing a recipe with placeholders like "some ingredient" (`Object`) and letting the cook figure it out later. This approach allows dynamic handling of types but *loses efficiency and type specificity* (`Integer` instead of `int`).

To overcome these limitations, languages like Kotlin implemented **reified types**.

## Reified types

***Reified types** retain type information as metadata at runtime, giving more control and the ability to check types when needed.*

Imagine a recipe that initially says "some ingredient" but can later reveal if it’s flour or sugar, depending on the context.

## As a result

Each of these techniques serves to make code more flexible and powerful.

- Templates generate efficient, type-specific code, but can lead to code bloat and compile time complexity.
- Type erasure provides runtime flexibility, albeit with some trade-offs.
- Reified types offer a middle ground, maintaining type details when needed. Might lead to runtime overhead.

Generics have revolutionized coding, allowing developers to write adaptable and reusable code.
