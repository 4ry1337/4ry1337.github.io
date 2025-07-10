+++
title = "Understanding Binary Representation in Computers"
date = 2024-08-14
description = "A comprehensive explanation of how computers represent integers and real numbers using binary, two's complement, and IEEE 754 floating-point standards."
[taxonomies]
tags = ["Binary", "Two's Complement", "Floating Point", "IEEE 754", "Computer Architecture", "Bit Manipulation"]
+++
Computers store and process data using binary, a system of 0s and 1s. The smallest unit of data is a bit, which can be either 0 or 1. Bits are grouped into bytes (8 bits), which can represent a wider range of information. For instance, a 32-bit system processes data in chunks of 32 bits, allowing it to represent a wide range of values.

# Representing Positive Numbers

Positive numbers in binary are straightforward. Each bit represents a power of 2, and the number is the sum of these powers where there’s a 1. For example, in an 8-bit system: `5 -> 00000101`

In a 32-bit system, you can represent numbers from `0` to `2^32 - 1`.

# Representing Negative Numbers

Easiest way to represent negative number is to change notation of left most
digit, which is also called *most significant bit* (MSB), to make it represent
sign (-). For example, in an 8-bit system:
```
01110101
^
|
Most significant bit (MSB)
```

If bit is 0 it is positive and 1 it is negative. This solution is great but it has many problems, which we will discuss later. A more practical method for representing negative numbers is *two's complement*.

## Two's complement

In this system, the MSB not only indicates the sign but also contributes to the number's value. This method simplifies arithmetic operations like addition and subtraction.

### How Two's Complement Works

1. Inversion (Flip the bits): Change every `0` to `1` and every `1` to `0`.
   - For example, `6 = 0110 -> 1001`.
2. Add 1: Add 1 to the inverted number.
   - `1001 + 1 = 1010`, which represents `-6`.

Example in Two’s Complement
```
6  = 0110 = 4 + 2
-6 = 1010 = -8 + 2
```

In a 32-bit system, this allows you to represent numbers from `-2^31` to `2^31 - 1`.

### Why Two's Complement?

There is only one zero (`0000...0000`), unlike other methods that might have both `0` and `-0`.

Adding a number to its two's complement (i.e., its negative) results in zero (**additive inverse**), making calculations straightforward for computers. For example:

```
6  = 0110
-6 = 1010
6 + (-6) = 0110 + 1010 = 0000 (in a 4-bit system)
```

# Representing Real Numbers (Floating-Point Numbers)

Real numbers are more complex to represent because they include decimals. If
you used engineering calculator you might saw that it represent numbers like this: `2 * e^(-3) = 0.002`\
Computers use the same representation with a slight changes. One of the obvious one's is binary system so `e` will mean `2`, not `10`.\
Which is called **IEEE 754 standard**, which ensures consistent handling of real numbers across different systems and programming languages.

In a 32-bit system, a floating-point number is divided into three parts:
```
0  10000011  00111001100000000000000
^     ^                ^
|     |                Mantissa
|     Exponent
|
Sign Bit
```

## Sign

**Sign Bit (1 bit):** Indicates whether the number is positive (0) or negative (1).

*Why use sign bit? and not two's complement or any other method isn't that more effective use of storage?*

The sign bit allows for a consistent representation of numbers across a wide range of magnitudes, including subnormal numbers, normalized numbers, and special cases like infinity and NaN (Not a Number). A sign bit, +0 and -0 can be represented distinctly, which is important in certain mathematical contexts. 

## Exponent

**Exponent (8 bits):** Determines the scale of the number by indicating how many places to shift the decimal point.

The exponent is stored with a bias (usually 127) to allow for both positive and negative values.

*Why bias?* To store both negative and positive numbers as unsigned integers, easier for computation.

*Why 127?* in 32-bit system it is 127, since we can represent 2^256 values in 8 bit space, if we add negative into account we can represent from -127 to 128. And by adding 127 we normalize it.

## Mantissa

**Mantissa (23 bits) or Precision:** Represents the significant digits of the number.

It’s normalized to typically start with a 1 (which is implied and not stored).

*Why normalize?* Normalization of the mantissa in floating-point representation is done to maximize precision and ensure a unique representation of numbers.

*Why imply 1?* The implicit 1 in the mantissa (or significand) of a floating-point number is a clever optimization used to increase precision without requiring additional storage.

### Special case (0)

*If 1 is implicit how to represent 0?*  The implicit 1 in the mantissa only applies to non-zero. To represent zero in floating-point format, a special case is used, where both the exponent and the mantissa are set to specific values: 

Sign bit: 0 for positive zero, 1 for negative zero. All bits of Exponent and Mantissa are set to 0.

# Understanding

It is easier to understand it like this:
```
(sign bit) * 1.(mantissa) * 2^(exponent part)
```

Example: Converting a Number to IEEE 754

Let’s walk through converting the number 19.59375 into this format:

1. **Sign Bit:** Since `19.59375` is positive, the sign bit is `0`.
2. **Convert to Binary:**

```
19.59375 = 10011.10011 (binary)
```

This breakdown corresponds to:

```
16 8 4 2 1 . 0.5 0.25 0.125 0.0625 0.03125
 1 0 0 1 1 .  1   0     0      1       1
```

3. **Normalize the Binary Number:** Shift the decimal point to create a number that starts with `1.`:

```
10011.10011 = 1.001110011 * 2^4
```

4. **Calculate the Biased Exponent:** Add the bias (`127` for a 32-bit system):

```
4 + 127 = 131 = 10000011 (binary)
```

5. **Determine the Mantissa:** Remove the leading `1.` from the normalized number:

```
1.001110011 => 001110011...
```

**Putting it all together:**

```
0 | 10000011 | 00111001100000000000000
```

This is how `19.59375` is stored in a 32-bit floating-point format.
