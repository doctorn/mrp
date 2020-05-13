# MRP

[![Build Status](https://travis-ci.com/mitten-lang/mrp.svg?token=KhJmSEzcFG1aixcpAAvB&branch=master)](https://travis-ci.com/mitten-lang/mrp)

The ultimate objective of the Mitten Programming Language project is to develop a modern, high-performance functional programming language with full compile-time garbage collection via ASAP (Proust, 2017). Although [`micro-mitten`](https://github.com/doctorn/micro-mitten) proves ASAP's viability, its performance is far from optimal. Enter the MRP...

The MRP is a compiler infrastructure built on top of LLVM, focused on enabling research into static memory management. The ultimate objective of the MRP is to serve as the back-end for `mittenc`. However, the near term goal of the MRP is to enable further research and development of ASAP. Goals of this research include:

- Generating high performance scanning code (the 'Ultrascan' sub-project)
- Providing foundations for mutability and type-polymorphism
- Improving on `micro-mitten`'s precision
- Reducing ASAP's compile-time overheads

## Further Reading

- [Practical Static Memory Management](http://nathancorbyn.com/nc513.pdf) (Corbyn, 2020)
- [ASAP: As Static As Possible memory management](https://www.cl.cam.ac.uk/techreports/UCAM-CL-TR-908.pdf) (Proust, 2017)
