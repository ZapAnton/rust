//! Tests for binary-search-tree
//!
//! Generated by [script][script] using [canonical data][canonical-data]
//!
//! [script]: https://github.com/exercism/rust/blob/master/bin/init_exercise.py
//! [canonical-data]: https://raw.githubusercontent.com/exercism/problem-specifications/master/exercises/binary-search-tree/canonical_data.json
//!
//! Each test case assumes an empty/new tree.
//! As per exercism/problem-specifications#996 key 'treeData' counts as an input
//! to test generators.
//! The key 'treeData' represents an array of numbers for which the data should be
//! inserted/added to the tree as it appears in the array from left to right.
//! e.g. treeData: ['2', '1', '3', '6', '7', '5']
//! Insert 2. Insert 1. Insert 3. Insert 6, so on...
//! This canonical-data does not restrict the data type of array elements to either
//! strings or integers.
//! The key 'expected' represents tree state as JSON object of schema :
//! {
//!     'title':'nodeObject',
//!     'type':'object',
//!     'properties':{
//!         'data':{
//!             'type':'string'
//!         },
//!         'left':{
//!             'type':'nodeObject'
//!         },
//!         'right':{
//!             'type':'nodeObject'
//!         }
//!     },
//!     'required':['data', 'left', 'right']
//! }

extern crate binary_search_tree;
use binary_search_tree::*;

/// Process a single test case for the property `sortedData`
///
/// All cases for the `sortedData` property are implemented
/// in terms of this function.
///
/// Note that you'll need to both name the expected transform which
/// the student needs to write, and name the types of the inputs and outputs.
/// While rustc _may_ be able to handle things properly given a working example,
/// students will face confusing errors if the `I` and `O` types are not concrete.
///
/// Expected input format: ('input',)
fn process_sorteddata_case<I, O>(input: I, expected: O) {
    // typical implementation:
    // assert_eq!(
    //     student_sortedData_func(input),
    //     expected
    // )
    unimplemented!()
}

/// Process a single test case for the property `data`
///
/// All cases for the `data` property are implemented
/// in terms of this function.
///
/// Note that you'll need to both name the expected transform which
/// the student needs to write, and name the types of the inputs and outputs.
/// While rustc _may_ be able to handle things properly given a working example,
/// students will face confusing errors if the `I` and `O` types are not concrete.
///
/// Expected input format: ('input',)
fn process_data_case<I, O>(input: I, expected: O) {
    // typical implementation:
    // assert_eq!(
    //     student_data_func(input),
    //     expected
    // )
    unimplemented!()
}

#[test]
/// data is retained
fn test_data_is_retained() {
    process_data_case(
        {
            let mut hm = ::std::collections::HashMap::new();
            hm.insert("treeData", vec!["4"]);
            hm
        },
        {
            let mut hm = ::std::collections::HashMap::new();
            hm.insert("data", "4");
            hm.insert("left", None);
            hm.insert("right", None);
            hm
        },
    );
}

// insert data at proper node

#[test]
#[ignore]
/// smaller number at left node
fn test_smaller_number_at_left_node() {
    process_data_case(
        {
            let mut hm = ::std::collections::HashMap::new();
            hm.insert("treeData", vec!["4", "2"]);
            hm
        },
        {
            let mut hm = ::std::collections::HashMap::new();
            hm.insert("data", "4");
            hm.insert("left", {
                let mut hm = ::std::collections::HashMap::new();
                hm.insert("data", "2");
                hm.insert("left", None);
                hm.insert("right", None);
                hm
            });
            hm.insert("right", None);
            hm
        },
    );
}

#[test]
#[ignore]
/// same number at left node
fn test_same_number_at_left_node() {
    process_data_case(
        {
            let mut hm = ::std::collections::HashMap::new();
            hm.insert("treeData", vec!["4", "4"]);
            hm
        },
        {
            let mut hm = ::std::collections::HashMap::new();
            hm.insert("data", "4");
            hm.insert("left", {
                let mut hm = ::std::collections::HashMap::new();
                hm.insert("data", "4");
                hm.insert("left", None);
                hm.insert("right", None);
                hm
            });
            hm.insert("right", None);
            hm
        },
    );
}

#[test]
#[ignore]
/// greater number at right node
fn test_greater_number_at_right_node() {
    process_data_case(
        {
            let mut hm = ::std::collections::HashMap::new();
            hm.insert("treeData", vec!["4", "5"]);
            hm
        },
        {
            let mut hm = ::std::collections::HashMap::new();
            hm.insert("data", "4");
            hm.insert("left", None);
            hm.insert("right", {
                let mut hm = ::std::collections::HashMap::new();
                hm.insert("data", "5");
                hm.insert("left", None);
                hm.insert("right", None);
                hm
            });
            hm
        },
    );
}

#[test]
#[ignore]
/// can create complex tree
fn test_can_create_complex_tree() {
    process_data_case(
        {
            let mut hm = ::std::collections::HashMap::new();
            hm.insert("treeData", vec!["4", "2", "6", "1", "3", "5", "7"]);
            hm
        },
        {
            let mut hm = ::std::collections::HashMap::new();
            hm.insert("data", "4");
            hm.insert("left", {
                let mut hm = ::std::collections::HashMap::new();
                hm.insert("data", "2");
                hm.insert("left", {
                    let mut hm = ::std::collections::HashMap::new();
                    hm.insert("data", "1");
                    hm.insert("left", None);
                    hm.insert("right", None);
                    hm
                });
                hm.insert("right", {
                    let mut hm = ::std::collections::HashMap::new();
                    hm.insert("data", "3");
                    hm.insert("left", None);
                    hm.insert("right", None);
                    hm
                });
                hm
            });
            hm.insert("right", {
                let mut hm = ::std::collections::HashMap::new();
                hm.insert("data", "6");
                hm.insert("left", {
                    let mut hm = ::std::collections::HashMap::new();
                    hm.insert("data", "5");
                    hm.insert("left", None);
                    hm.insert("right", None);
                    hm
                });
                hm.insert("right", {
                    let mut hm = ::std::collections::HashMap::new();
                    hm.insert("data", "7");
                    hm.insert("left", None);
                    hm.insert("right", None);
                    hm
                });
                hm
            });
            hm
        },
    );
}

// can sort data

#[test]
#[ignore]
/// can sort single number
fn test_can_sort_single_number() {
    process_sorteddata_case(
        {
            let mut hm = ::std::collections::HashMap::new();
            hm.insert("treeData", vec!["2"]);
            hm
        },
        vec!["2"],
    );
}

#[test]
#[ignore]
/// can sort if second number is smaller than first
fn test_can_sort_if_second_number_is_smaller_than_first() {
    process_sorteddata_case(
        {
            let mut hm = ::std::collections::HashMap::new();
            hm.insert("treeData", vec!["2", "1"]);
            hm
        },
        vec!["1", "2"],
    );
}

#[test]
#[ignore]
/// can sort if second number is same as first
fn test_can_sort_if_second_number_is_same_as_first() {
    process_sorteddata_case(
        {
            let mut hm = ::std::collections::HashMap::new();
            hm.insert("treeData", vec!["2", "2"]);
            hm
        },
        vec!["2", "2"],
    );
}

#[test]
#[ignore]
/// can sort if second number is greater than first
fn test_can_sort_if_second_number_is_greater_than_first() {
    process_sorteddata_case(
        {
            let mut hm = ::std::collections::HashMap::new();
            hm.insert("treeData", vec!["2", "3"]);
            hm
        },
        vec!["2", "3"],
    );
}

#[test]
#[ignore]
/// can sort complex tree
fn test_can_sort_complex_tree() {
    process_sorteddata_case(
        {
            let mut hm = ::std::collections::HashMap::new();
            hm.insert("treeData", vec!["2", "1", "3", "6", "7", "5"]);
            hm
        },
        vec!["1", "2", "3", "5", "6", "7"],
    );
}