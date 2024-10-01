# Heap Allocation and Embedded Programming

In embedded programming, you often need to provide a custom heap allocator because embedded systems typically do not have an operating system (OS) that manages process memory space. Here’s a more detailed explanation:

## Explanation

1. **No OS Memory Management**: In a typical Rust program running on a general-purpose OS, the OS provides memory management, including heap allocation. The OS manages the process memory space, and the Rust standard library can rely on the OS to handle dynamic memory allocation.

2. **Embedded Systems**: In contrast, embedded systems often run without an OS or with a very minimal OS that does not provide advanced memory management features. This means that the program itself must manage memory allocation.

3. **Custom Heap Allocator**: To handle dynamic memory allocation in an embedded environment, you need to provide a custom heap allocator. This allocator is responsible for managing a fixed block of memory that you designate as the heap.

4. **Global Allocator**: In Rust, you can specify a global allocator using the `#[global_allocator]` attribute. This allocator will be used for all heap allocations, such as those performed by `Box`, `Vec`, and other heap-allocating types.

### Example from Your Code

In this project code, we are using the [`embedded_alloc`](https://github.com/rust-embedded/embedded-alloc) crate to provide a custom heap allocator.
