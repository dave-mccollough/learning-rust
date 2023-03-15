# Ownership

## Rules of ownership
- Each value has a varaible called it's owner
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dropped (freed)



## Stack
- Stores values in the order it receives them, releases them in the opposite order
    - LIFO (Last in, First out)
- All data stored on the stack must have a known fixed size
- Data without a known size or data that might change will be stored on the heap
- Pushing data onto the stack is faster than the heap
- Searching for data on the stack is faster than the heap
- Variables passed into a function or local variables are all pushed to the stack
    - When the function is over, they are popped off the stack (removed from stack)


## Heap
- Heap is less organized than the stack
- When a value is added to the heap, you request a space and are returned a pointer



