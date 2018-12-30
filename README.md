# Rust-Catch

This is a Rust unit-testing framework based on the excellent C++ library [Catch/Catch2](https://github.com/catchorg/Catch2).

The goals of the Rust library are the same as the C++ one; write unit-test code using the patterns and idioms of the Rust language.

## Motivation

In Rust, unit-tests are written as functions using the `#[test]` attribute. e.g.

```rust
#[test]
fn add_test() {
    assert_eq!(add(1, 2), 3);
}
```

There are some issues with this:
* People write short names for functions, rather than good descriptions for the test.
* Tests have to test multiple things at once or repeat setup code. There is no default support for text fixtures.

## Solution

Rust-Catch aims to take the best things from default unit-testing in Rust and add facilities that make it easier to write maintainable tests.

The above test could be written using Rust-Catch as follows:

```rust
test_suite! {
    test_case!("add works with positive numbers") {
        assert_eq!(add(1, 2), 3);
    }

    test_case("add works with negative numbers") {
        assert_eq!(add(5, -1), 4);
    }
}

// This is equivalent to the following:

#[test]
fn add_works_with_positive_numbers() {
    assert_eq!(add(1, 2), 3);
}

#[test]
fn add_works_with_negative_numbers() {
    assert_eq!(add(5, -1), 4);
}
```


## Unimplemented Features

### Sections

Sections are an answer to text fixtures from other testing frameworks. The setup code or teardown code in the test case becomes part of each section. This reducing the amount of repeated code in tests and reduces the amount of specialist code required for unit-tests.

```rust
test_suite! {
    test_case!("Vec can be expanded and shrunk") {
        
        // Setup
        let mut vec = Vec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);

        section!("Expanding a vec increases capacity & length") {
            vec.push(4);

            assert_eq!(vec.len(), 4);
            assert_eq!(vec.capacity(), 4);
        }

        section!("reducing the vec decreases length but not capacity") {
            vec.pop();

            assert_eq!(vec.len(), 2);
            assert_eq!(vec.capacity(), 3);
        }
    }
}

// Is converted to:

mod Vec_can_be_expanded_and_shrunk {
    #[test]
    fn Expanding_a_vec_increases_capacity_&_length() {

        // Setup
        let mut vec = Vec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);

        // Test code
        vec.push(4);

        assert_eq!(vec.len(), 4);
        assert_eq!(vec.capacity(), 4);
    }

    #[test]
    fn reducing_the_vec_decreases_length_but_not_capacity() {

        // Setup
        let mut vec = Vec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);

        // Test code
        vec.pop();

        assert_eq!(vec.len(), 2);
        assert_eq!(vec.capacity(), 3);
    }
}
```

## Implementation Details

Rust is implemented using procedural macros.

Due to Rust parsing rules `test_case`'s have to be surrounded by a `test_suite`.

`test_suite` is currently removed from the generated code but I'm not sure that's the best approach. An alternative is to wrap all `test_case`'s with a `mod` e.g.

```rust
test_suite! {
    ...
}

// Becomes
#[cfg(test)] // Is this necessary?
mod test {
    ...
}
```