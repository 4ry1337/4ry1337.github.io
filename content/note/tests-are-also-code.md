+++
title = "Tests are also code" 
date = 2025-01-11
description = "A reflection on how treating tests as first-class code can improve long-term maintainability, reliability, and developer experience."
[taxonomies]
tags = ["Testing", "TDD", "Software Engineering", "Code Quality", "Test Coverage", "Developer Experience"]
+++

# Introduction

Recently, I was writing some pet project by TDD (test driven development) and I noticed that I have  changed my way of thinking about tests. We all knew that tests are vital, they make our program safe and reliable, but do you write tests with full coverage? have you ever written with full covarage? why it is frustrating to write test? and why you might feel that tests are waste of time?

All tests start in the same way: an empty file, a world of possibilities.\
You go in, you add the first test. Easy, done.\
Then the second. Boom.\
The third. You just had to copy a few lines from the first, all good.\
The fourth…

After a while, test coverage starts to go down: new code is less thoroughly tested than the code you wrote at the very beginning of the project. Have you started to doubt the value of tests? Absolutely not, tests are great! Yet, you are writing fewer tests as the project moves forward.

It’s because of friction - it got progressively more cumbersome to write new tests as the codebase evolved.

# Tests are also code

Tests has to be modular, well-structured, sufficiently documented. They require maintenance. If we do not actively invest in the health of our test suite, it will rot over time. Coverage goes down and soon enough we will find critical paths in our application code that are never exercised by automated tests. You need to regularly step back to take a look at your test suite as a whole.
