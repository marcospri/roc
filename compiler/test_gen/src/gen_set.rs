#![cfg(test)]

use crate::assert_evals_to;
use crate::assert_llvm_evals_to;
use indoc::indoc;

#[test]
fn empty_len() {
    assert_evals_to!(
        indoc!(
            r#"
            Set.len Set.empty
            "#
        ),
        0,
        usize
    );
}

#[test]
fn singleton_len() {
    assert_evals_to!(
        indoc!(
            r#"
            Set.len (Set.singleton 42)
            "#
        ),
        1,
        usize
    );
}

#[test]
fn singleton_to_list() {
    assert_evals_to!(
        indoc!(
            r#"
            Set.toList (Set.singleton 42)
            "#
        ),
        &[42],
        &[i64]
    );

    assert_evals_to!(
        indoc!(
            r#"
            Set.toList (Set.singleton 1)
            "#
        ),
        &[1],
        &[i64]
    );

    assert_evals_to!(
        indoc!(
            r#"
            Set.toList (Set.singleton 1.0)
            "#
        ),
        &[1.0],
        &[f64]
    );
}

#[test]
fn insert() {
    assert_evals_to!(
        indoc!(
            r#"
            Set.empty
                |> Set.insert 0
                |> Set.insert 1
                |> Set.insert 2
                |> Set.toList
            "#
        ),
        &[0, 1, 2],
        &[i64]
    );
}

#[test]
fn remove() {
    assert_evals_to!(
        indoc!(
            r#"
            Set.empty
                |> Set.insert 0
                |> Set.insert 1
                |> Set.remove 1
                |> Set.remove 2
                |> Set.toList
            "#
        ),
        &[0],
        &[i64]
    );
}

#[test]
fn union() {
    assert_evals_to!(
        indoc!(
            r#"
            set1 : Set I64
            set1 = Set.fromList [1,2]

            set2 : Set I64
            set2 = Set.fromList [1,3,4] 

            Set.union set1 set2
                |> Set.toList
            "#
        ),
        &[4, 2, 3, 1],
        &[i64]
    );
}

#[test]
fn difference() {
    assert_evals_to!(
        indoc!(
            r#"
            set1 : Set I64
            set1 = Set.fromList [1,2]

            set2 : Set I64
            set2 = Set.fromList [1,3,4] 

            Set.difference set1 set2
                |> Set.toList
            "#
        ),
        &[2],
        &[i64]
    );
}

#[test]
fn intersection() {
    assert_evals_to!(
        indoc!(
            r#"
            set1 : Set I64
            set1 = Set.fromList [1,2]

            set2 : Set I64
            set2 = Set.fromList [1,3,4] 

            Set.intersection set1 set2
                |> Set.toList
            "#
        ),
        &[1],
        &[i64]
    );
}

#[test]
fn walk_sum() {
    assert_evals_to!(
        indoc!(
            r#"
            Set.walk (Set.fromList [1,2,3]) (\x, y -> x + y) 0
            "#
        ),
        6,
        i64
    );
}

#[test]
fn contains() {
    assert_evals_to!(
        indoc!(
            r#"
            Set.contains (Set.fromList [1,3,4]) 4
            "#
        ),
        true,
        bool
    );

    assert_evals_to!(
        indoc!(
            r#"
            Set.contains (Set.fromList [1,3,4]) 2
            "#
        ),
        false,
        bool
    );
}

#[test]
fn from_list() {
    let empty_list: &'static [i64] = &[];

    assert_evals_to!(
        indoc!(
            r#"
            [1,2,2,3,1,4]
                |> Set.fromList
                |> Set.toList
            "#
        ),
        &[4, 2, 3, 1],
        &[i64]
    );

    assert_evals_to!(
        indoc!(
            r#"
            []
                |> Set.fromList
                |> Set.toList
            "#
        ),
        empty_list,
        &[i64]
    );

    assert_evals_to!(
        indoc!(
            r#"
            empty : List I64
            empty = []

            empty
                |> Set.fromList
                |> Set.toList
            "#
        ),
        empty_list,
        &[i64]
    );
}