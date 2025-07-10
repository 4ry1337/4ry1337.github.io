+++
title = "Huffman Coding"
date = 2024-06-19
description = "A breakdown of Huffman Coding, a lossless compression algorithm that assigns variable-length binary codes based on character frequency to reduce data size efficiently."
[taxonomies]
tags = ["Compression", "Huffman Coding", "Data Structures", "Binary Trees", "Rust", "Algorithms"]

[extra]
sources = [
   "https://en.wikipedia.org/wiki/Huffman_coding",
   "https://huffman-coding-online.vercel.app",
   "https://ben-tanen.com/adaptive-huffman"
]
+++
# Introduction

When we transmit information, need to convert the data (text, music, video, etc.) into binary code. To do this, we assign a code to each piece of data so we can distinguish them, and decode them later.

If we take a string as an example, for example 'ABACA', we could assign a same-length code to each one of the unique symbols (usually called naive coding).

Naive Coding:

```
A = 00, B = 01, C = 11
```

Encoded output:

```
00 01 00 11 00
```

With the obtained table, we could later translate the binary codes back to the text without loosing information on the process, but is this the best way to do this?

# Huffman Coding

Huffman coding improves this process, being a lossless data compression algorithm that assigns variable-length codes based on the frequencies of our input characters.

Let's say we have string `ABCABDA`, what Huffman does it first takes frequency of each character puts them into priority queue in descending order:

```
A — 3, B — 2, C — 1, D — 1
```

We have 4 values then we put them into *min-heap* (binary tree where data contained in each node is less than or equal to the data in that node's children).

Firstly we always take 2 values that are less frequent and combine them creating our first nodes:
```
      (2, 'CD')
      /         \
(1, 'C')     (1, 'D')
```

Then we add their frequencies and put it back into priority queue: 

```
A — 3, B — 2, CD — 2
```

Then we repeat previous steps
So 2 lowest are B and CD
```
      (4, 'BCD')
       /         \
(2, 'B')   (2, 'CD')
                /         \
         (1, 'C')     (1, 'D')
```

And putting back into priority queue: 

```
BCD — 4, A — 3
```

Notice that order changed since it is priority queue

Then we repeat until we get:

```
      (7, root)
    /           \
(3, 'A')   (4, 'BCD')
          /         \
   (2, 'B')   (2, 'CD')
                   /         \
            (1, 'C')     (1, 'D')
```

This tree is called Huffman Tree

Then we assign 0 for left hand side and 1 for right hand side:

```
      (7, root)
       /     \
     0        1
    /           \
(3, 'A')   (4, 'BCD')
                 /     \
               0       1
              /           \
       (2, 'B')      (2, 'CD')
                         /     \
                       0        1
                      /            \
                 (1, 'C')     (1, 'D')
```

But using this tree we can simply walk from the root to our character's leaf node:

```
A — 0
B — 10
C — 110
D — 111

// These values are called Huffman Codes
```

Hence, `ABCABDA` encoded is `0 10 110 0 10 111 0` which is `13 bits —> (upper ceiling) 13 / 8 bit/byte  = 2 bytes.` The main idea is that we assign longer bit sequence to a less frequent value.\
So originally, `ABCABDA` is stored in 56 bits or 7 bytes: 7 char * 1 byte * 8 bits/byte = 56 bits, then our compression rate is `13/56 = 0.23 = 23%`, on average Huffman Code *compresses data up to [39%](https://www.researchgate.net/publication/264848217_A_PERFORMANCE_ANALYSIS_OF_DELTA_AND_HUFFMAN_COMPRESSION_ALGORITHMS)*

[Check out my Implementation in Rust](https://github.com/4ry1337/mini-projects/tree/master/huffman)
