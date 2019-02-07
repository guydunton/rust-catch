#![cfg_attr(test, deny(warnings))]

use rust_catch::test_suite;



fn add(a: i32, b: i32) -> i32 {
    a + b
}


test_suite! {

    test_case("add works with positive numbers") {
        assert_eq!(add(1, 2), 3);
    }

    test_case("add works with negative numbers") {
        assert_eq!(add(5, -1), 4);
    }

}

/*

expands to:

#[test]
fn add_works_with_positive_numbers() {
    assert_eq!(add(1, 2), 3);
}

#[test]
fn add_works_with_negative_numbers() {
    assert_eq!(add(5, -1), 4);
}

*/


test_suite! {
    test_case("sectioned test case") {
        
        let mut a = 3;

        section("adding 1 increases the value") {
            a += 1;
            assert_eq!(a, 4);
        }

        section("minus 1 decreases the value") {
            a -= 1;
            assert_eq!(a, 2);
        }
    }
}


test_suite! {
    test_case("another test case") {

        let mut a = 4; // This shouldn't warn on remove mut

        section("test which modifies a") {
            a += 2;
            assert_eq!(a, 6);
        }

        section("test that doesnt modify a") {
            assert_eq!(a, 4);
        }

        section("sec doesnt use a") {
            assert_eq!(4, 4);
        }
    }
}

/*

mod another_test_case {

    #[test]
    fn test_which_modifies_a() {
        let __rust_catch_section = 0;
        let mut a = 4; // This shouldn't warn on remove mut

        if __rust_catch_section == 0 {
            a += 2;
            assert_eq!(a, 6);
        }

        if __rust_catch_section == 1 {
            assert_eq!(a, 4);
        }

        if __rust_catch_section == 2 {
            assert_eq!(4, 4);
        }
    }

    #[test]
    fn test_that_doesnt_modify_a() {
        let __rust_catch_section = 1;
        let mut a = 4; // This shouldn't warn on remove mut
        
        if __rust_catch_section == 0 {
            a += 2;
            assert_eq!(a, 6);
        }

        if __rust_catch_section == 1 {
            assert_eq!(a, 4);
        }

        if __rust_catch_section == 2 {
            assert_eq!(4, 4);
        }
    }

    #[test]
    fn sec_doesnt_use_a() {
        let __rust_catch_section = 2;
        let mut a = 4; 

        if __rust_catch_section == 0 {
            a += 2;
            assert_eq!(a, 6);
        }

        if __rust_catch_section == 1 {
            assert_eq!(a, 4);
        }

        if __rust_catch_section == 2 {
            assert_eq!(4, 4);
        }
    }
}

*/
test_suite! {
    test_case("add tests") {
        section("Add works with regular numbers") {
            assert_eq!(add(1, 2), 3);
        }

        section("Add works with negative numbers") {
            assert_eq!(add(5, -1), 4);
        }
    }
}


test_suite! {
    test_case("test something") {
        let mut vals = vec![1, 2, 3, 4];

        section("I can add things") {
            vals.push(5);
            assert_eq!(vals.len(), 5);
            assert!(vals.capacity() >= 5);
        }

        section("I can remove things") {
            vals.pop();
            assert_eq!(vals.len(), 3);
            assert_eq!(vals.capacity(), 4);
        }
    }
}

/*
// This will give an error but will highlight the test_case name only
test_suite! {
    test_case("") {
        section("") {
        }
    }
}
*/

/*

#[test_case]
test "" {

}

*/


fn main() {
    println!("Adding numbers: {}", add(1, 2));
}
