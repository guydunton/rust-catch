use rust_catch::test_suite;

fn add(a: i32, b: i32) -> i32 {
    a + b
}


test_suite! {

    test_case!("add works with positive numbers") {
        assert_eq!(add(1, 2), 3);
    }

    test_case!("add works with negative numbers") {
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
    test_case!("sectioned test case") {
        
        let mut a = 3;

        section!("adding 1 increases the value") {
            a += 1;
            assert_eq!(a, 4);
        }

        section!("minus 1 decreases the value") {
            a -= 1;
            assert_eq!(a, 2);
        }
    }
}



fn main() {
    println!("Adding numbers: {}", add(1, 2));
}
