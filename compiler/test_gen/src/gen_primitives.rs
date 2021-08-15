#![cfg(test)]

use crate::assert_evals_to;
use crate::assert_llvm_evals_to;
use crate::assert_non_opt_evals_to;
use core::ffi::c_void;
use indoc::indoc;
use roc_std::RocStr;

#[test]
fn basic_int() {
    assert_evals_to!("123", 123, i64);
}

#[test]
fn basic_float() {
    assert_evals_to!("1234.0", 1234.0, f64);
}

#[test]
fn branch_first_float() {
    assert_evals_to!(
        indoc!(
            r#"
                when 1.23 is
                    1.23 -> 12
                    _ -> 34
            "#
        ),
        12,
        i64
    );
}

#[test]
fn branch_second_float() {
    assert_evals_to!(
        indoc!(
            r#"
                when 2.34 is
                    1.23 -> 63
                    _ -> 48
            "#
        ),
        48,
        i64
    );
}

#[test]
fn branch_third_float() {
    assert_evals_to!(
        indoc!(
            r#"
               when 10.0 is
                   1.0 -> 63
                   2.0 -> 48
                   _ -> 112
            "#
        ),
        112,
        i64
    );
}

#[test]
fn branch_first_int() {
    assert_evals_to!(
        indoc!(
            r#"
                when 1 is
                    1 -> 12
                    _ -> 34
            "#
        ),
        12,
        i64
    );
}

#[test]
fn branch_second_int() {
    assert_evals_to!(
        indoc!(
            r#"
                when 2 is
                    1 -> 63
                    _ -> 48
            "#
        ),
        48,
        i64
    );
}

#[test]
fn branch_third_int() {
    assert_evals_to!(
        indoc!(
            r#"
                when 10 is
                    1 -> 63
                    2 -> 48
                    _ -> 112
            "#
        ),
        112,
        i64
    );
}

#[test]
fn branch_store_variable() {
    assert_evals_to!(
        indoc!(
            r#"
                when 0 is
                    1 -> 12
                    a -> a
            "#
        ),
        0,
        i64
    );
}

#[test]
fn when_one_element_tag() {
    assert_evals_to!(
        indoc!(
            r#"
            x : [ Pair (Int a) (Int a) ]
            x = Pair 0x2 0x3

            when x is
                Pair l r -> l + r
            "#
        ),
        5,
        i64
    );
}

#[test]
fn when_two_element_tag_first() {
    assert_evals_to!(
        indoc!(
            r#"
            x : [A (Int *), B (Int *)]
            x = A 0x2

            when x is
                A v -> v
                B v -> v
            "#
        ),
        2,
        i64
    );
}

#[test]
fn when_two_element_tag_second() {
    assert_evals_to!(
        indoc!(
            r#"
            x : [A (Int *), B (Int *)]
            x = B 0x3

            when x is
                A v -> v
                B v -> v
            "#
        ),
        3,
        i64
    );
}

#[test]
fn gen_when_one_branch() {
    assert_evals_to!(
        indoc!(
            r#"
                when 3.14 is
                    _ -> 23
            "#
        ),
        23,
        i64
    );
}

#[test]
fn gen_large_when_int() {
    assert_evals_to!(
        indoc!(
            r#"
                foo = \num ->
                    when num is
                        0 -> 200
                        -3 -> 111 # TODO adding more negative numbers reproduces parsing bugs here
                        3 -> 789
                        1 -> 123
                        2 -> 456
                        _ -> 1000

                foo -3
            "#
        ),
        111,
        i64
    );
}

// #[test]
// fn gen_large_when_float() {
//     assert_evals_to!(
//         indoc!(
//             r#"
//                 foo = \num ->
//                     when num is
//                         0.5 -> 200.1
//                         -3.6 -> 111.2 # TODO adding more negative numbers reproduces parsing bugs here
//                         3.6 -> 789.5
//                         1.7 -> 123.3
//                         2.8 -> 456.4
//                         _ -> 1000.6

//                 foo -3.6
//             "#
//         ),
//         111.2,
//         f64
//     );
// }

#[test]
fn or_pattern() {
    assert_evals_to!(
        indoc!(
            r#"
            when 2 is
                1 | 2 -> 42
                _ -> 1
            "#
        ),
        42,
        i64
    );
}

#[test]
fn apply_identity() {
    assert_evals_to!(
        indoc!(
            r#"
                identity = \a -> a

                identity 5
            "#
        ),
        5,
        i64
    );
}

#[test]
fn apply_unnamed_identity() {
    assert_evals_to!(
        indoc!(
            r#"
            wrapper = \{} ->
                (\a -> a) 5

            wrapper {}
            "#
        ),
        5,
        i64
    );
}

#[test]
fn return_unnamed_fn() {
    assert_evals_to!(
        indoc!(
            r#"
            wrapper = \{} ->
                alwaysFloatIdentity : Int * -> (Float a -> Float a)
                alwaysFloatIdentity = \_ ->
                    (\a -> a)

                (alwaysFloatIdentity 2) 3.14

            wrapper {}
            "#
        ),
        3.14,
        f64
    );
}

#[test]
fn gen_when_fn() {
    assert_evals_to!(
        indoc!(
            r#"
                limitedNegate = \num ->
                    when num is
                        1 -> -1
                        -1 -> 1
                        _ -> num

                limitedNegate 1
            "#
        ),
        -1,
        i64
    );
}

#[test]
fn gen_basic_def() {
    assert_evals_to!(
        indoc!(
            r#"
                answer = 42

                answer
            "#
        ),
        42,
        i64
    );

    assert_evals_to!(
        indoc!(
            r#"
                pi = 3.14

                pi
            "#
        ),
        3.14,
        f64
    );
}

#[test]
fn gen_multiple_defs() {
    assert_evals_to!(
        indoc!(
            r#"
                answer = 42

                pi = 3.14

                if pi > 3 then answer else answer
            "#
        ),
        42,
        i64
    );

    assert_evals_to!(
        indoc!(
            r#"
                answer = 42

                pi = 3.14

                if answer > 3 then pi else pi
            "#
        ),
        3.14,
        f64
    );
}

// These tests caught a bug in how Defs are converted to the mono IR
// but they have UnusedDef or UnusedArgument problems, and don't run any more
//    #[test]
//    fn gen_chained_defs() {
//        assert_evals_to!(
//            indoc!(
//                r#"
//                    x = i1
//                    i3 = i2
//                    i1 = 1337
//                    i2 = i1
//                    y = 12.4
//
//                    i3
//                "#
//            ),
//            1337,
//            i64
//        );
//    }
//
//    #[test]
//    fn gen_nested_defs_old() {
//        assert_evals_to!(
//            indoc!(
//                r#"
//                    x = 5
//
//                    answer =
//                        i3 = i2
//
//                        nested =
//                            a = 1.0
//                            b = 5
//
//                            i1
//
//                        i1 = 1337
//                        i2 = i1
//
//
//                        nested
//
//                    # None of this should affect anything, even though names
//                    # overlap with the previous nested defs
//                    unused =
//                        nested = 17
//
//                        i1 = 84.2
//
//                        nested
//
//                    y = 12.4
//
//                    answer
//                "#
//            ),
//            1337,
//            i64
//        );
//    }
//
//    #[test]
//    fn let_x_in_x() {
//        assert_evals_to!(
//            indoc!(
//                r#"
//                    x = 5
//
//                    answer =
//                        1337
//
//                    unused =
//                        nested = 17
//                        nested
//
//                    answer
//                "#
//            ),
//            1337,
//            i64
//        );
//    }

#[test]
fn factorial() {
    assert_evals_to!(
        indoc!(
            r#"
            factorial = \n, accum ->
                when n is
                    0 ->
                        accum

                    _ ->
                        factorial (n - 1) (n * accum)

            factorial 10 1
            "#
        ),
        3628800,
        i64
    );
}

#[test]
fn peano1() {
    assert_non_opt_evals_to!(
        indoc!(
            r#"
                Peano : [ S Peano, Z ]

                three : Peano
                three = S (S (S Z))

                when three is
                    Z -> 2
                    S _ -> 1
                "#
        ),
        1,
        i64
    );
}

#[test]
fn peano2() {
    assert_non_opt_evals_to!(
        indoc!(
            r#"
                Peano : [ S Peano, Z ]

                three : Peano
                three = S (S (S Z))

                when three is
                    S (S _) -> 1
                    S (_) -> 0
                    Z -> 0
                "#
        ),
        1,
        i64
    );
}

#[test]
fn top_level_constant() {
    assert_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            pi = 3.1415

            main =
                pi + pi
                "#
        ),
        3.1415 + 3.1415,
        f64
    );
}

#[test]
fn linked_list_len_0() {
    assert_non_opt_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            LinkedList a : [ Nil, Cons a (LinkedList a) ]

            len : LinkedList a -> Int *
            len = \list ->
                when list is
                    Nil -> 0
                    Cons _ rest -> 1 + len rest

            main =
                nil : LinkedList F64
                nil = Nil

                len nil
            "#
        ),
        0,
        i64
    );
}

#[test]
fn linked_list_len_twice_0() {
    assert_non_opt_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            LinkedList a : [ Nil, Cons a (LinkedList a) ]

            nil : LinkedList I64
            nil = Nil

            length : LinkedList a -> Int *
            length = \list ->
                when list is
                    Nil -> 0
                    Cons _ rest -> 1 + length rest

            main =
                length nil + length nil
            "#
        ),
        0,
        i64
    );
}

#[test]
fn linked_list_len_1() {
    assert_non_opt_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            LinkedList a : [ Nil, Cons a (LinkedList a) ]

            one : LinkedList (Int *)
            one = Cons 1 Nil

            length : LinkedList a -> Int *
            length = \list ->
                when list is
                    Nil -> 0
                    Cons _ rest -> 1 + length rest

            main =
                length one
            "#
        ),
        1,
        i64
    );
}

#[test]
fn linked_list_len_twice_1() {
    assert_non_opt_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            LinkedList a : [ Nil, Cons a (LinkedList a) ]

            one : LinkedList (Int *)
            one = Cons 1 Nil

            length : LinkedList a -> Int *
            length = \list ->
                when list is
                    Nil -> 0
                    Cons _ rest -> 1 + length rest

            main =
                length one + length one
                "#
        ),
        2,
        i64
    );
}

#[test]
fn linked_list_len_3() {
    assert_non_opt_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            LinkedList a : [ Nil, Cons a (LinkedList a) ]

            three : LinkedList (Int *)
            three = Cons 3 (Cons 2 (Cons 1 Nil))

            length : LinkedList a -> Int *
            length = \list ->
                when list is
                    Nil -> 0
                    Cons _ rest -> 1 + length rest


            main =
                length three
            "#
        ),
        3,
        i64
    );
}

#[test]
fn linked_list_sum_num_a() {
    assert_non_opt_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            LinkedList a : [ Nil, Cons a (LinkedList a) ]

            three : LinkedList (Int *)
            three = Cons 3 (Cons 2 (Cons 1 Nil))


            sum : LinkedList (Num a) -> Num a
            sum = \list ->
                when list is
                    Nil -> 0
                    Cons x rest -> x + sum rest

            main =
                sum three
            "#
        ),
        3 + 2 + 1,
        i64
    )
}

#[test]
fn linked_list_sum_int() {
    assert_non_opt_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            LinkedList a : [ Nil, Cons a (LinkedList a) ]

            zero : LinkedList (Int *)
            zero = Nil

            sum : LinkedList (Int a) -> Int a
            sum = \list ->
                when list is
                    Nil -> 0
                    Cons x rest -> x + sum rest

            main =
                sum zero
            "#
        ),
        0,
        i64
    )
}

#[test]
fn linked_list_map() {
    assert_non_opt_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            LinkedList a : [ Nil, Cons a (LinkedList a) ]

            three : LinkedList (Int *)
            three = Cons 3 (Cons 2 (Cons 1 Nil))

            sum : LinkedList (Num a) -> Num a
            sum = \list ->
                when list is
                    Nil -> 0
                    Cons x rest -> x + sum rest

            map : (a -> b), LinkedList a -> LinkedList b
            map = \f, list ->
                when list is
                    Nil -> Nil
                    Cons x rest -> Cons (f x) (map f rest)

            main =
                sum (map (\_ -> 1) three)
            "#
        ),
        3,
        i64
    );
}

#[test]
fn when_nested_maybe() {
    assert_evals_to!(
        indoc!(
            r#"
            Maybe a : [ Nothing, Just a ]

            x : Maybe (Maybe (Int a))
            x = Just (Just 41)

            when x is
                Just (Just v) -> v + 0x1
                _ -> 0x1
                "#
        ),
        42,
        i64
    );

    assert_evals_to!(
        indoc!(
            r#"
            Maybe a : [ Nothing, Just a ]

            x : Maybe (Maybe (Int *))
            x = Just Nothing

            when x is
                Just (Just v) -> v + 0x1
                Just Nothing -> 0x2
                Nothing -> 0x1
                "#
        ),
        2,
        i64
    );

    assert_evals_to!(
        indoc!(
            r#"
            Maybe a : [ Nothing, Just a ]

            x : Maybe (Maybe (Int *))
            x = Nothing

            when x is
                Just (Just v) -> v + 0x1
                Just Nothing -> 0x2
                Nothing -> 0x1
                "#
        ),
        1,
        i64
    );
}

#[test]
fn when_peano() {
    assert_non_opt_evals_to!(
        indoc!(
            r#"
                Peano : [ S Peano, Z ]

                three : Peano
                three = S (S (S Z))

                when three is
                    S (S _) -> 1
                    S (_) -> 2
                    Z -> 3
                "#
        ),
        1,
        i64
    );

    assert_non_opt_evals_to!(
        indoc!(
            r#"
                Peano : [ S Peano, Z ]

                three : Peano
                three = S Z

                when three is
                    S (S _) -> 1
                    S (_) -> 2
                    Z -> 3
                "#
        ),
        2,
        i64
    );

    assert_non_opt_evals_to!(
        indoc!(
            r#"
                Peano : [ S Peano, Z ]

                three : Peano
                three = Z

                when three is
                    S (S _) -> 1
                    S (_) -> 2
                    Z -> 3
                "#
        ),
        3,
        i64
    );
}

#[test]
#[should_panic(expected = "Roc failed with message: ")]
fn overflow_frees_list() {
    assert_evals_to!(
        indoc!(
            r#"
            myList = [1,2,3]

            # integer overflow; must use the list so it is defined before the overflow
            # the list will then be freed in a cleanup block
            n : I64
            n = 9_223_372_036_854_775_807 + (Num.intCast (List.len myList))

            index = Num.intCast n

            List.get myList index
                 "#
        ),
        3,
        i64
    );
}

#[test]
#[should_panic(expected = "Roc failed with message: ")]
fn undefined_variable() {
    assert_evals_to!(
        indoc!(
            r#"
                 if True then
                     x + z
                 else
                     y + z
                 "#
        ),
        3,
        i64
    );
}

#[test]
#[should_panic(expected = "Roc failed with message: ")]
fn annotation_without_body() {
    assert_evals_to!(
        indoc!(
            r#"
            foo : Int *


            foo
            "#
        ),
        3,
        i64
    );
}

#[test]
fn simple_closure() {
    assert_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            x = 42

            f = \{} -> x


            main =
                f {}
            "#
        ),
        42,
        i64
    );
}

#[test]
fn nested_closure() {
    assert_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            foo = \{} ->
                x = 41
                y = 1
                f = \{} -> x + y
                f

            main =
                g = foo {}
                g {}
            "#
        ),
        42,
        i64
    );
}

#[test]
fn closure_in_list() {
    assert_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            foo = \{} ->
                x = 41

                f = \{} -> x

                [ f ]

            main =
                items = foo {}

                List.len items
            "#
        ),
        1,
        i64
    );
}

#[test]
fn specialize_closure() {
    use roc_std::RocList;

    assert_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            foo = \{} ->
                x = 41
                y = [1]

                f = \{} -> x
                g = \{} -> x + List.len y

                [ f, g ]

            apply = \f -> f {}

            main =
                items = foo {}

                List.map items apply
            "#
        ),
        RocList::from_slice(&[41, 42]),
        RocList<i64>
    );
}

#[test]
fn io_poc_effect() {
    assert_non_opt_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            Effect a : [ @Effect ({} -> a) ]

            succeed : a -> Effect a
            succeed = \x -> @Effect \{} -> x

            runEffect : Effect a -> a
            runEffect = \@Effect thunk -> thunk {}

            foo : Effect F64
            foo =
                succeed 3.14

            main : F64
            main =
                runEffect foo

            "#
        ),
        3.14,
        f64
    );
}

#[test]
fn io_poc_desugared() {
    assert_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            # succeed : a -> ({} -> a)
            succeed = \x -> \_ -> x

            foo : Str -> F64
            foo =
                succeed 3.14

            # runEffect : ({} ->  a) -> a
            runEffect = \thunk -> thunk ""

            main : F64
            main =
                runEffect foo
            "#
        ),
        3.14,
        f64
    );
}

#[test]
fn return_wrapped_function_pointer() {
    assert_non_opt_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            Effect a : [ @Effect ({} -> a) ]

            foo : Effect {}
            foo = @Effect \{} -> {}

            main : Effect {}
            main = foo
            "#
        ),
        1,
        i64,
        |_| 1
    );
}

#[test]
fn return_wrapped_function_pointer_b() {
    assert_non_opt_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"


            foo : { x: (I64 -> Str) }
            foo = { x:  (\_ -> "foobar") }

            main : { x:  (I64 -> Str) }
            main = foo
            "#
        ),
        1,
        i64,
        |_| 1
    );
}

#[test]
fn return_wrapped_closure() {
    assert_non_opt_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            Effect a : [ @Effect ({} -> a) ]

            foo : Effect {}
            foo =
                x = 5

                @Effect (\{} -> if x > 3 then {} else {})

            main : Effect {}
            main = foo
            "#
        ),
        1,
        i64,
        |_| 1
    );
}

#[test]
fn linked_list_is_singleton() {
    assert_non_opt_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            ConsList a : [ Cons a (ConsList a), Nil ]

            empty : ConsList a
            empty = Nil

            isSingleton : ConsList a -> Bool
            isSingleton = \list ->
                when list is
                    Cons _ Nil ->
                        True

                    _ ->
                        False

            main : Bool
            main =
                myList : ConsList I64
                myList = empty

                isSingleton myList
            "#
        ),
        false,
        bool
    );
}

#[test]
fn linked_list_is_empty_1() {
    assert_non_opt_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            ConsList a : [ Cons a (ConsList a), Nil ]

            empty : ConsList a
            empty = Nil

            isEmpty : ConsList a -> Bool
            isEmpty = \list ->
                when list is
                    Cons _ _ ->
                        False

                    Nil ->
                        True

            main : Bool
            main =
                myList : ConsList (Int *)
                myList = empty

                isEmpty myList
            "#
        ),
        true,
        bool
    );
}

#[test]
fn linked_list_is_empty_2() {
    assert_non_opt_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            ConsList a : [ Cons a (ConsList a), Nil ]

            isEmpty : ConsList a -> Bool
            isEmpty = \list ->
                when list is
                    Cons _ _ ->
                        False

                    Nil ->
                        True

            main : Bool
            main =
                myList : ConsList I64
                myList = Cons 0x1 Nil

                isEmpty myList
            "#
        ),
        false,
        bool
    );
}

#[test]
fn linked_list_singleton() {
    // verifies only that valid llvm is produced
    assert_non_opt_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            ConsList a : [ Cons a (ConsList a), Nil ]

            main : ConsList I64
            main = Cons 0x1 Nil
            "#
        ),
        0,
        i64,
        |_| 0
    );
}

#[test]
fn recursive_function_with_rigid() {
    assert_non_opt_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            State a : { count : I64, x : a }

            foo : State a -> Int *
            foo = \state ->
                if state.count == 0 then
                    0
                else
                    1 + foo { count: state.count - 1, x: state.x }

            main : Int *
            main =
                foo { count: 3, x: {} }
            "#
        ),
        3,
        i64
    );
}

#[test]
fn rbtree_insert() {
    assert_non_opt_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            NodeColor : [ Red, Black ]

            RedBlackTree k v : [ Node NodeColor k v (RedBlackTree k v) (RedBlackTree k v), Empty ]

            Key k : Num k

            insert : Key k, v, RedBlackTree (Key k) v -> RedBlackTree (Key k) v
            insert = \key, value, dict ->
                when insertHelp key value dict is
                    Node Red k v l r ->
                        Node Black k v l r

                    x ->
                        x

            insertHelp : (Key k), v, RedBlackTree (Key k) v -> RedBlackTree (Key k) v
            insertHelp = \key, value, dict ->
              when dict is
                Empty ->
                  # New nodes are always red. If it violates the rules, it will be fixed
                  # when balancing.
                  Node Red key value Empty Empty

                Node nColor nKey nValue nLeft nRight ->
                  when Num.compare key nKey is
                    LT ->
                      balance nColor nKey nValue (insertHelp key value nLeft) nRight

                    EQ ->
                      Node nColor nKey value nLeft nRight

                    GT ->
                      balance nColor nKey nValue nLeft (insertHelp key value nRight)

            balance : NodeColor, k, v, RedBlackTree k v, RedBlackTree k v -> RedBlackTree k v
            balance = \color, key, value, left, right ->
              when right is
                Node Red rK rV rLeft rRight ->
                  when left is
                    Node Red lK lV lLeft lRight ->
                      Node
                        Red
                        key
                        value
                        (Node Black lK lV lLeft lRight)
                        (Node Black rK rV rLeft rRight)

                    _ ->
                      Node color rK rV (Node Red key value left rLeft) rRight

                _ ->
                  when left is
                    Node Red lK lV (Node Red llK llV llLeft llRight) lRight ->
                      Node
                        Red
                        lK
                        lV
                        (Node Black llK llV llLeft llRight)
                        (Node Black key value lRight right)

                    _ ->
                      Node color key value left right

            show : RedBlackTree I64 {} -> Str
            show = \tree ->
                when tree is
                    Empty -> "Empty"
                    Node _ _ _ _ _ -> "Node"


            main : Str
            main =
                show (insert 0 {} Empty)
            "#
        ),
        RocStr::from_slice("Node".as_bytes()),
        RocStr
    );
}

#[test]
fn rbtree_balance_3() {
    assert_non_opt_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            RedBlackTree k : [ Node k (RedBlackTree k) (RedBlackTree k), Empty ]

            balance : k, RedBlackTree k -> RedBlackTree k
            balance = \key, left ->
                Node key left Empty

            main : RedBlackTree (Int *)
            main =
                balance 0 Empty
            "#
        ),
        false,
        *const i64,
        |x: *const i64| x.is_null()
    );
}

#[test]
#[ignore]
fn rbtree_layout_issue() {
    // there is a flex var in here somewhere that blows up layout creation
    assert_non_opt_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            NodeColor : [ Red, Black ]

            RedBlackTree k v : [ Node NodeColor k v (RedBlackTree k v) (RedBlackTree k v), Empty ]

            # balance : NodeColor, k, v, RedBlackTree k v -> RedBlackTree k v
            balance = \color, key, value, right ->
              when right is
                Node Red _ _ rLeft rRight ->
                    Node color key value rLeft rRight


                _ ->
                    Empty

            show : RedBlackTree * * -> Str
            show = \tree ->
                when tree is
                    Empty -> "Empty"
                    Node _ _ _ _ _ -> "Node"

            zero : I64
            zero = 0

            main : Str
            main = show (balance Red zero zero Empty)
            "#
        ),
        RocStr::from_slice("Empty".as_bytes()),
        RocStr
    );
}

#[test]
#[ignore]
fn rbtree_balance_mono_problem() {
    // because of how the function is written, only `Red` is used and so in the function's
    // type, the first argument is a unit and dropped. Apparently something is weird with
    // constraint generation where the specialization required by `main` does not fix the
    // problem. As a result, the first argument is dropped and we run into issues down the line
    //
    // concretely, the `rRight` symbol will not be defined
    assert_non_opt_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            NodeColor : [ Red, Black ]

            RedBlackTree k v : [ Node NodeColor k v (RedBlackTree k v) (RedBlackTree k v), Empty ]

            # balance : NodeColor, k, v, RedBlackTree k v, RedBlackTree k v -> RedBlackTree k v
            balance = \color, key, value, left, right ->
              when right is
                Node Red rK rV rLeft rRight ->
                  when left is
                    Node Red lK lV lLeft lRight ->
                      Node
                        Red
                        key
                        value
                        (Node Black lK lV lLeft lRight)
                        (Node Black rK rV rLeft rRight)

                    _ ->
                      Node color rK rV (Node Red key value left rLeft) rRight

                _ ->
                    Empty

            show : RedBlackTree * * -> Str
            show = \tree ->
                when tree is
                    Empty -> "Empty"
                    Node _ _ _ _ _ -> "Node"


            main : Str
            main = show (balance Red 0 0 Empty Empty)
            "#
        ),
        RocStr::from_slice("Empty".as_bytes()),
        RocStr
    );
}

#[test]
fn rbtree_balance_full() {
    assert_non_opt_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            NodeColor : [ Red, Black ]

            RedBlackTree k v : [ Node NodeColor k v (RedBlackTree k v) (RedBlackTree k v), Empty ]

            balance : NodeColor, k, v, RedBlackTree k v, RedBlackTree k v -> RedBlackTree k v
            balance = \color, key, value, left, right ->
              when right is
                Node Red rK rV rLeft rRight ->
                  when left is
                    Node Red lK lV lLeft lRight ->
                      Node
                        Red
                        key
                        value
                        (Node Black lK lV lLeft lRight)
                        (Node Black rK rV rLeft rRight)

                    _ ->
                      Node color rK rV (Node Red key value left rLeft) rRight

                _ ->
                  when left is
                    Node Red lK lV (Node Red llK llV llLeft llRight) lRight ->
                      Node
                        Red
                        lK
                        lV
                        (Node Black llK llV llLeft llRight)
                        (Node Black key value lRight right)

                    _ ->
                      Node color key value left right

            main : RedBlackTree F64 F64
            main =
                balance Red 0 0 Empty Empty
            "#
        ),
        true,
        usize,
        |x| x != 0
    );
}

#[test]
fn nested_pattern_match_two_ways() {
    // exposed an issue in the ordering of pattern match checks when ran with `--release` mode
    assert_non_opt_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            ConsList a : [ Cons a (ConsList a), Nil ]

            balance : ConsList (Int *) -> Int *
            balance = \right ->
              when right is
                Cons 1 foo ->
                    when foo is
                        Cons 1 _ -> 3
                        _ -> 3
                _ -> 3

            main : Int *
            main =
                when balance Nil is
                    _ -> 3
            "#
        ),
        3,
        i64
    );

    assert_non_opt_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            ConsList a : [ Cons a (ConsList a), Nil ]

            balance : ConsList (Int *) -> Int *
            balance = \right ->
              when right is
                Cons 1 (Cons 1 _) -> 3
                _ -> 3

            main : Int *
            main =
                when balance Nil is
                    _ -> 3
            "#
        ),
        3,
        i64
    );
}

#[test]
fn linked_list_guarded_double_pattern_match() {
    // the important part here is that the first case (with the nested Cons) does not match
    // TODO this also has undefined behavior
    assert_non_opt_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            ConsList a : [ Cons a (ConsList a), Nil ]

            balance : ConsList (Int *) -> Int *
            balance = \right ->
              when right is
                Cons 1 foo ->
                    when foo is
                        Cons 1 _ -> 3
                        _ -> 3
                _ -> 3

            main : Int *
            main =
                when balance Nil is
                    _ -> 3
            "#
        ),
        3,
        i64
    );
}

#[test]
fn linked_list_double_pattern_match() {
    assert_non_opt_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            ConsList a : [ Cons a (ConsList a), Nil ]

            foo : ConsList (Int a) -> Int a
            foo = \list ->
                when list is
                    Cons _ (Cons x _) -> x
                    _ -> 0

            main : Int *
            main =
                foo (Cons 1 (Cons 32 Nil))
            "#
        ),
        32,
        i64
    );
}

#[test]
fn binary_tree_double_pattern_match() {
    assert_non_opt_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            BTree : [ Node BTree BTree, Leaf I64 ]

            foo : BTree -> I64
            foo = \btree ->
                when btree is
                    Node (Node (Leaf x) _) _ -> x
                    _ -> 1

            main : I64
            main =
                foo (Node (Node (Leaf 32) (Leaf 2)) (Leaf 3))
            "#
        ),
        32,
        i64
    );
}

#[test]
fn unified_empty_closure_bool() {
    // none of the Closure tags will have a payload
    // this was not handled correctly in the past
    assert_non_opt_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            foo = \{} ->
                when A is
                    A -> (\_ -> 3.14)
                    B -> (\_ -> 3.14)

            main : Float *
            main =
                (foo {}) 0
            "#
        ),
        3.14,
        f64
    );
}

#[test]
fn unified_empty_closure_byte() {
    // none of the Closure tags will have a payload
    // this was not handled correctly in the past
    assert_non_opt_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            foo = \{} ->
                when A is
                    A -> (\_ -> 3.14)
                    B -> (\_ -> 3.14)
                    C -> (\_ -> 3.14)

            main : Float *
            main =
                (foo {}) 0
            "#
        ),
        3.14,
        f64
    );
}

#[test]
fn task_always_twice() {
    assert_non_opt_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            Effect a : [ @Effect ({} -> a) ]

            effectAlways : a -> Effect a
            effectAlways = \x ->
                inner = \{} -> x

                @Effect inner

            effectAfter : Effect a, (a -> Effect b) -> Effect b
            effectAfter = \(@Effect thunk), transform -> transform (thunk {})

            Task a err : Effect (Result a err)

            always : a -> Task a *
            always = \x -> effectAlways (Ok x)

            fail : err -> Task * err
            fail = \x -> effectAlways (Err x)

            after : Task a err, (a -> Task b err) -> Task b err
            after = \task, transform ->
                effectAfter task \res ->
                    when res is
                        Ok x -> transform x
                        Err e -> fail e

            main : Task {} (Float *)
            main = after (always "foo") (\_ -> always {})

            "#
        ),
        0,
        i64,
        |_| 0
    );
}

#[test]
fn wildcard_rigid() {
    assert_non_opt_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            Effect a : [ @Effect ({} -> a) ]

            Task a err : Effect (Result a err)

            # this failed because of the `*`, but worked with `err`
            always : a -> Task a *
            always = \x ->
                inner = \{} -> (Ok x)

                @Effect inner


            main : Task {} (Float *)
            main = always {}
            "#
        ),
        0,
        i64,
        |_| 0
    );
}

#[test]
#[ignore]
fn todo_bad_error_message() {
    assert_non_opt_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            Effect a : [ @Effect ({} -> a) ]

            effectAlways : a -> Effect a
            effectAlways = \x ->
                inner = \{} -> x

                @Effect inner

            effectAfter : Effect a, (a -> Effect b) -> Effect b
            effectAfter = \(@Effect thunk), transform -> transform (thunk {})

            Task a err : Effect (Result a err)

            always : a -> Task a (Float *)
            always = \x -> effectAlways (Ok x)

            # the problem is that this restricts to `Task {} *`
            fail : err -> Task {} err
            fail = \x -> effectAlways (Err x)

            after : Task a err, (a -> Task b err) -> Task b err
            after = \task, transform ->
                effectAfter task \res ->
                    when res is
                        Ok x -> transform x
                        # but here it must be `forall b. Task b {}`
                        Err e -> fail e

            main : Task {} (Float *)
            main =
                after (always "foo") (\_ -> always {})
            "#
        ),
        0,
        i64,
        |_| 0
    );
}

#[test]
fn hof_conditional() {
    // exposed issue with the if condition being just a symbol
    assert_evals_to!(
        indoc!(
            r#"
                passTrue = \f -> f True

                passTrue (\trueVal -> if trueVal then False else True)
            "#
        ),
        0,
        u8
    );
}

#[test]
#[should_panic(
    expected = "Roc failed with message: \"Shadowing { original_region: |L 3-3, C 4-5|, shadow: |L 6-6, C 8-9| Ident"
)]
fn pattern_shadowing() {
    assert_evals_to!(
        indoc!(
            r#"
            x = 4

            when 4 is
                x -> x
            "#
        ),
        0,
        i64
    );
}

#[test]
#[should_panic(expected = "TODO non-exhaustive pattern")]
fn non_exhaustive_pattern_let() {
    assert_evals_to!(
        indoc!(
            r#"
            x : Result (Int a) (Float b)
            x = Ok 4

            (Ok y) = x

            y
            "#
        ),
        0,
        i64
    );
}

#[test]
#[ignore]
#[should_panic(expected = "")]
fn unsupported_pattern_str_interp() {
    assert_evals_to!(
        indoc!(
            r#"
            { x: 4 } = { x : 4 }

            x
            "#
        ),
        0,
        i64
    );
}

#[test]
#[ignore]
fn fingertree_basic() {
    assert_non_opt_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            Some a : [ One a, Two a a, Three a a a ]

            Tuple a : [ Pair a a, Triple a a a ]

            # a FingerTree implementation
            Seq a : [ Nil, Unit a, More (Some a) (Seq (Tuple a)) (Some a) ]

            # cons : a, Seq a -> Seq a
            cons = \x, s ->
                when s is
                    Nil -> Unit x
                    Unit y -> More (One x) Nil (One y)
                    More some q u ->
                        when some is
                            One y -> More (Two x y) q u
                            Two y z -> More (Three x y z) q u
                            Three y z w -> More (Two x y) (consTuple (Pair z w) q) u

            consTuple : Tuple a, Seq (Tuple a) -> Seq (Tuple a)
            consTuple = \a, b -> cons a b

            main : Bool
            main =
                when cons 0x1 Nil is
                    Unit 1 -> True
                    _ -> False
            "#
        ),
        true,
        bool
    );
}

#[test]
fn case_or_pattern() {
    // the `0` branch body should only be generated once in the future
    // it is currently duplicated
    assert_evals_to!(
        indoc!(
            r#"
            x : [ Red, Green, Blue ]
            x = Red

            when x is
                Red | Green -> 0
                Blue -> 1
            "#
        ),
        0,
        i64
    );
}

#[test]
#[ignore]
fn rosetree_basic() {
    assert_non_opt_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            Tree a : [ Tree a (List (Tree a)) ]

            singleton : a -> Tree a
            singleton = \x -> Tree x []

            main : Bool
            main =
                x : Tree F64
                x = singleton 3
                when x is
                    Tree 3.0 _ -> True
                    _ -> False
            "#
        ),
        true,
        bool
    );
}

#[test]
fn case_jump() {
    // the decision tree will generate a jump to the `1` branch here
    assert_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            ConsList a : [ Cons a (ConsList a), Nil ]

            x : ConsList I64
            x = Nil

            main =
                when Pair x x is
                    Pair Nil _ -> 1
                    Pair _ Nil -> 2
                    Pair (Cons a _) (Cons b _) -> a + b + 3
            "#
        ),
        1,
        i64
    );
}

#[test]
fn nullable_eval_cfold() {
    // the decision tree will generate a jump to the `1` branch here
    assert_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            Expr : [ Var, Val I64, Add Expr Expr, Mul Expr Expr ]

            mkExpr : I64, I64 -> Expr
            mkExpr = \n , v ->
                when n is
                    0 -> if v == 0 then Var else Val v
                    _ -> Add (mkExpr (n-1) (v+1)) (mkExpr (n-1) (max (v-1) 0))

            max : I64, I64 -> I64
            max = \a, b -> if a > b then a else b

            eval : Expr -> I64
            eval = \e ->
                when e is
                    Var   -> 0
                    Val v -> v
                    Add l r -> eval l + eval r
                    Mul l r -> eval l * eval r

            main : I64
            main = eval (mkExpr 3 1)
            "#
        ),
        11,
        i64
    );
}

#[test]
fn nested_switch() {
    // exposed bug with passing the right symbol/layout down into switch branch generation
    assert_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            Expr : [ ZAdd Expr Expr, Val I64, Var I64 ]

            eval : Expr -> I64
            eval = \e ->
                when e is
                    Var _ -> 0
                    Val v -> v
                    ZAdd l r -> eval l + eval r

            constFolding : Expr -> Expr
            constFolding = \e ->
                when e is
                    ZAdd e1 e2 ->
                        when Pair e1 e2 is
                            Pair (Val a) (Val b) -> Val (a+b)
                            Pair (Val a) (ZAdd x (Val b)) -> ZAdd (Val (a+b)) x
                            Pair _ _                     -> ZAdd e1 e2


                    _ -> e


            expr : Expr
            expr = ZAdd (Val 3) (ZAdd (Val 4) (Val 5))

            main : I64
            main = eval (constFolding expr)
            "#
        ),
        12,
        i64
    );
}

#[test]
fn count_deriv_x() {
    // exposed bug with basing the block_of_memory on a specific (smaller) tag layout
    assert_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            Expr : [ Ln Expr, Pow Expr Expr, Var Str ]

            count : Expr -> I64
            count = \expr ->
                when expr is
                    (Var _) -> 1
                    (Pow f g) -> count f + count g
                    (Ln f)    -> count f

            main : I64
            main = count (Var "x")
            "#
        ),
        1,
        i64
    );
}

#[test]
fn deriv_pow() {
    // exposed bug with ordering of variable declarations before switch
    assert_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            Expr : [ Ln Expr, Pow Expr Expr, Var Str, Val I64 ]

            count : Expr -> I64
            count = \expr ->
                when expr is
                    (Var _) -> 1
                    (Val n) -> n
                    (Pow f g) -> count f + count g
                    (Ln f)    -> count f

            pow : Expr, Expr -> Expr
            pow = \a,b ->
                when Pair a b is
                    Pair (Val _) (Val _) -> Val -1
                    Pair _       (Val 0) -> Val 1
                    Pair f       (Val 1) -> f
                    Pair (Val 0) _       -> Val 0
                    Pair f       g       -> Pow f g

            main : I64
            main = count (pow (Var "x") (Var "x"))
            "#
        ),
        2,
        i64
    );
}

#[test]
fn multiple_increment() {
    // the `leaf` value will be incremented multiple times at once
    assert_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"


            Color : [ Red, Black ]

            Tree a b : [ Leaf, Node Color (Tree a b) a b (Tree a b) ]

            Map : Tree I64 Bool

            main : I64
            main =
                leaf : Map
                leaf = Leaf

                m : Map
                m = Node Black (Node Black leaf 10 False leaf) 11 False (Node Black leaf 12 False (Node Red leaf 13 False leaf))

                when m is
                    Leaf -> 0
                    Node _ _ _ _ _ -> 1
            "#
        ),
        1,
        i64
    );
}

#[test]
fn switch_fuse_rc_non_exhaustive() {
    assert_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            Foo : [ A I64 Foo, B I64 Foo, C I64 Foo, Empty ]

            sum : Foo, I64 -> I64
            sum = \foo, accum ->
                when foo is
                    A x resta -> sum resta (x + accum)
                    B x restb -> sum restb (x + accum)
                    # Empty -> accum
                    # C x restc -> sum restc (x + accum)
                    _ -> accum

            main : I64
            main =
                A 1 (B 2 (C 3 Empty))
                    |> sum 0
            "#
        ),
        3,
        i64
    );
}

#[test]
fn switch_fuse_rc_exhaustive() {
    assert_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            Foo : [ A I64 Foo, B I64 Foo, C I64 Foo, Empty ]

            sum : Foo, I64 -> I64
            sum = \foo, accum ->
                when foo is
                    A x resta -> sum resta (x + accum)
                    B x restb -> sum restb (x + accum)
                    C x restc -> sum restc (x + accum)
                    Empty -> accum

            main : I64
            main =
                A 1 (B 2 (C 3 Empty))
                    |> sum 0
            "#
        ),
        6,
        i64
    );
}

#[test]
fn build_then_apply_closure() {
    assert_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            main : Str
            main =
                x = "long string that is malloced"

                (\_ -> x) {}
            "#
        ),
        "long string that is malloced",
        &'static str
    );
}

#[test]
fn expanded_result() {
    assert_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            a : Result I64 Str
            a = Ok 4

            after = \x, f ->
                when x is
                    Ok v -> f v
                    Err e -> Err e

            main : I64
            main =
                helper = after a (\x -> Ok x)

                when helper is
                    Ok v -> v
                    Err _ -> 0

            "#
        ),
        4,
        i64
    );
}

#[test]
#[ignore]
fn backpassing_result() {
    assert_evals_to!(
        indoc!(
            r#"
            app "test" provides [ main ] to "./platform"

            a : Result I64 Str
            a = Ok 1

            f = \x -> Ok (x + 1)
            g = \y -> Ok (y * 2)

            main : I64
            main =
                helper =
                    x <- Result.after a
                    y <- Result.after (f x)
                    z <- Result.after (g y)

                    Ok z

                helper
                    |> Result.withDefault 0

            "#
        ),
        4,
        i64
    );
}

#[test]
#[should_panic(
    expected = "Shadowing { original_region: |L 3-3, C 4-5|, shadow: |L 5-5, C 6-7| Ident"
)]
fn function_malformed_pattern() {
    assert_evals_to!(
        indoc!(
            r#"
                x = 3

                (\x -> x) 42
            "#
        ),
        3,
        i64
    );
}

#[test]
#[should_panic(expected = "Hit an erroneous type when creating a layout for")]
fn call_invalid_layout() {
    assert_llvm_evals_to!(
        indoc!(
            r#"
                f : I64 -> I64
                f = \x -> x

                f {}
            "#
        ),
        3,
        i64,
        |x| x,
        false,
        true
    );
}

#[test]
#[should_panic(expected = "assert failed!")]
fn expect_fail() {
    assert_evals_to!(
        indoc!(
            r#"
            expect 1 == 2

            3
            "#
        ),
        3,
        i64
    );
}

#[test]
fn increment_or_double_closure() {
    assert_evals_to!(
        indoc!(
            r#"
                app "test" provides [ main ] to "./platform"


                apply : (a -> a), a -> a
                apply = \f, x -> f x

                main =
                    one : I64
                    one = 1

                    two : I64
                    two = 2

                    b : Bool
                    b = True

                    increment : I64 -> I64
                    increment = \x -> x + one

                    double : I64 -> I64
                    double = \x -> if b then x * two else x

                    f = (if True then increment else double)

                    apply f 42
            "#
        ),
        43,
        i64
    );
}

#[test]
fn module_thunk_is_function() {
    assert_evals_to!(
        indoc!(
            r#"
                app "test" provides [ main ] to "./platform"

                main = helper "foo" "bar"
                helper = Str.concat
            "#
        ),
        RocStr::from_slice(b"foobar"),
        RocStr
    );
}

#[test]
#[should_panic(expected = "Roc failed with message: ")]
fn hit_unresolved_type_variable() {
    assert_evals_to!(
        indoc!(
            r#"
                app "test" provides [ main ] to "./platform"

                main : Str
                main =
                    (accept Bool.isEq) "B"


                accept : * -> (b -> b)
                accept = \_ ->
                    \input -> input
            "#
        ),
        RocStr::from_slice(b"B"),
        RocStr
    );
}

#[test]
fn pattern_match_empty_record() {
    assert_evals_to!(
        indoc!(
            r#"
                app "test" provides [ main ] to "./platform"

                main : I64
                main =
                    when {} is
                        {} -> 0

            "#
        ),
        0,
        i64
    );
}

#[test]
fn pattern_match_unit_tag() {
    assert_evals_to!(
        indoc!(
            r#"
                app "test" provides [ main ] to "./platform"

                unit : [ Unit ]
                unit = Unit

                main : I64
                main =
                    when unit is
                        Unit -> 0

            "#
        ),
        0,
        i64
    );
}

#[test]
fn mirror_llvm_alignment_padding() {
    // see https://github.com/rtfeldman/roc/issues/1569
    assert_evals_to!(
        indoc!(
            r#"
                app "test" provides [ main ] to "./platform"

                main : Str
                main =
                    p1 = {name : "test1", test: 1 == 1 }

                    List.map [p1, p1 ] (\{ test } -> if test  then "pass" else "fail")
                       |> Str.joinWith "\n"

            "#
        ),
        RocStr::from_slice(b"pass\npass"),
        RocStr
    );
}
