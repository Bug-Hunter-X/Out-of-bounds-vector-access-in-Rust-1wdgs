# Out of Bounds Vector Access in Rust

This repository demonstrates a common error in Rust: accessing an element of a vector using an index that is out of bounds.  This will cause a runtime panic.  The `bug.rs` file contains the buggy code, while `bugSolution.rs` shows how to fix it.

The problem occurs because the vector `vec` only has two elements (indices 0 and 1), but the code attempts to access element at index 2, resulting in an out-of-bounds panic.

**Solution:** Before accessing an element, always check that the index is within the valid range of indices (0 to vec.len()-1).