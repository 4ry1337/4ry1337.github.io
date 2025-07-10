+++
title = "Four Nines of Availability"
date = 2025-01-02
description = "An introduction to the concept of high availability in software systems, focusing on the significance of achieving 99.99% uptime and the challenges involved."
[taxonomies]
tags = ["Availability", "SLA", "Reliability", "DevOps", "Rust"]

[extra]
sources = [
    "https://en.wikipedia.org/wiki/High_availability",
    "https://www.zero2prod.com/index.html"
]
+++
# What is a reliable software?

Reliable means a lot of things, for commerce it is SLA (Service Level Agreement) that companies are obligated to provide a certain level of reliability and compensate customers if this agreement is broken. If a user cannot access the system, it is – from the user's point of view – *unavailable*.

For solo devs that are rolling up some APIs means that your API should successfully respond to 99.99% of well-formed request, or often referred as **"four nines of availability"**.

You might think it is reasonable thing to do, but put it in perspective you can only afford up to 52 minutes of downtime throughout a whole year.

Achieving four nines of availability is tough. Choose your VPSs correctly and write better containerization.

