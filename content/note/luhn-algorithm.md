+++
title = "Luhn Algorithm"
date = 2024-10-15
description = "Explore how the Luhn Algorithm validates payment card numbers by detecting input errors using a simple yet effective checksum formula."
[taxonomies]
tags = ["Validation", "Checksum", "Luhn Algorithm", "Finance", "Security", "Credit Cards"]
+++
# Introduction

Ever wondered what the numbers on the front of your bank card actually mean? They aren’t random!\
These numbers, known as the Payment Account Number (PAN), play a crucial role in identifying your account and ensuring your transactions are secure.

Let's break it down

> What Is PAN?\
> The PAN is made up of 8 to 19 digits, depending on the card issuer.

First 6 to 8 digits: Issuer Identification Number (IIN) or Bank Identification Number (BIN) – this identifies your bank or card provider.\
Remaining digits (except the last one): Your individual account number.\
Last digit: A special check digit used to validate the card number – this is where the magic of the Luhn Algorithm comes in!

# Why Validate PANs with the Luhn Algorithm?

When you enter personal information online, like an email or password, it’s validated to prevent errors. Similarly, the Luhn Algorithm is used to validate credit card numbers and other IDs (like IMEI numbers for phones) to ensure they’re input correctly.

# How the Luhn Algorithm Works?

Let's use my KASPI card number, which is VISA card

```
4400 4302 8383 0898
```

Remove the check digit (last digit).\
PAN without the check digit:

```
4400 4302 8383 089
```
The check digit is 8.

Double every second digit, starting from the end (right to left):

```txt
4 4 0 0 4 3 0 2 8 3 8 3 0 8 9
x
2 1 2 1 2 1 2 1 2 1 2 1 2 1 2
=
8 4 0 0 8 3 2 1 16 3 16 3 0 8 18
```

Sum the value of resulting digits:

```txt
8 + 4 + 0 + 0 + 8 + 3 + 0 + 2 + (1 + 6) + 3 + (1 + 6) + 3 + 0 + 8 + (1 + 8) = 62
```

Apply the Luhn formula:

```txt
f(x)  = (10 - (x mod 10)) mod 10
f(62) = (10 - (62 mod 10)) mod 10
      = (10 - 2) mod 10
      = 8
```

Compare the result with the check digit:\
The calculated value is 8.\
The original check digit was also 8.\
✅ It’s a valid card number!

# Why Does This Matter?

Payment systems like Stripe, KASPI, and others use the Luhn Algorithm to quickly validate card numbers during transactions. This ensures that typos are caught before they cause problems – all without needing to contact your bank.\
You can do it on your own card, and it will work 😁

> The algorithm is not intended to be a cryptographically secure hash function, it is only to validate input!
