~~~~~ Understanding Ownership in Rust ~~~~~
Rust has many different rules for ownership, which are what allows it to be
    memory safe while also not needing a garbace collector. This chapter will
    dive in to this ownership, covering Borrowing, Slices, and how data
    itself is layed out in memory with Rust.



~~~ What is Ownership? ~~~
"Ownership" is a set of rules that governs how Rust manages memory in a program.

In Rust, memory is managed through a system of ownership with a set of rules
    that the compiler uses to compile them.



~~~ The stack and the heap ~~~
In most programming languages, you don't have to worry about the stack or 
    heap, but since Rust is a systems programming language it's important to
    know where a value is stored because it will largely affect performance.

Here's a brief overview of the stack and heap in Rust.

The stack stores values in the order that it recieves them in a 
    "last in, first out" order. Adding data is called "pushing" and removing
    data is called "popping". All data on the stack must have a known, fixed
    size. Data with unknown size is stored on the heap.

The heap is less organized and more dynamic. When you store data onto the heap
    you have to request a certain amount of space. When you allocate this
    memory, the memory allocator will find an empty spot in the heap, mark it
    as in use, and return a pointer to the address. This is called "allocating
    the heap".

Pushing to the stack is faster than allocating the heap, because the stack never
    has to search for a new place to store data. It always just stores it on the
    top of the stack. It's also slower to follow the pointer to access the data
    in the heap as opposed to the stack, where it is just right on top.

When code calls a function, the values passed into the function along with the
    function's local variables are pushed onto the stack. When the function
    ends, these values are popped off of the stack.

