#[macro_export]
macro_rules! compute{
    ($bind:expr, $unit:expr => unit $e:expr) => {
        $unit($e)
    };

    ($bind:expr, $unit:expr => $var:ident <- $e:expr; $($rest:tt)+) => {
        $bind($e, move |$var| compute!($bind, $unit => $($rest)+))
    };

    ($bind:expr, $unit:expr => let $var:pat = $e:expr ; $($rest:tt)+) => {
        { 
            let $var = $e;
            compute!($bind, $unit => $($rest)+)
        }
    };

    ($bind:expr, $unit:expr => let $var:ident : $var_type:ty = $e:expr ; $($rest:tt)+) => {
        { 
            let $var : $var_type = $e;
            compute!($bind, $unit => $($rest)+)
        }
    };

    ($bind:expr, $unit:expr => let $var:pat | $var_type:ty = $e:expr ; $($rest:tt)+) => {
        { 
            let $var : $var_type = $e;
            compute!($bind, $unit => $($rest)+)
        }
    };
}

#[cfg(test)]
mod tests {
    fn v_unit<T>( t : T ) -> Vec<T> {
        vec![t]
    }

    fn v_bind<T, S>( a : Vec<T>, next : impl Fn(T) -> Vec<S> ) -> Vec<S> {
        fn map<T, S>(f : impl Fn(T) -> S, t : Vec<T>) -> Vec<S> {
            t.into_iter().map(f).collect()
        }

        fn mu<T>(t : Vec<Vec<T>>) -> Vec<T> {
            t.into_iter().flatten().collect()
        }

        mu(map(next, a))
    }

    #[test] 
    fn should_work_with_just_unit() {
        let output = compute!( v_bind, v_unit => unit 8 );
        assert_eq!( output, vec![8] );
    }

    #[test]
    fn should_compute() {
        let output = compute!{ v_bind, v_unit => 
            x <- vec![1,2,3];
            y <- vec![1,2,3];
            unit x + y
        };

        assert_eq!( output, vec![2, 3, 4, 3, 4, 5, 4, 5, 6] );
    }

    #[test]
    fn should_compute_with_single_bind() {
        let output = compute!{ v_bind, v_unit => 
            x <- vec![1,2,3];
            unit x
        };

        assert_eq!( output, vec![1, 2, 3] );
    }

    #[test]
    fn should_compute_with_function_expr() {
        fn create_vec(z : u32) -> Vec<u32> {
            vec![z, z, z]
        }

        let output = compute!{ v_bind, v_unit => 
            x <- vec![1,2,3];
            w <- create_vec(x);
            unit w
        };

        assert_eq!( output, vec![1, 1, 1, 2, 2, 2, 3, 3, 3] );
    }

    #[test]
    fn should_compute_with_let() {
        let output = compute!{ v_bind, v_unit => 
            x <- vec![1,2,3];
            let y = 7;
            w <- vec![1,2,3];
            unit x + y + w
        };

        assert_eq!( output, vec![9, 10, 11, 10, 11, 12, 11, 12, 13] );
    }

    #[test]
    fn should_compute_with_let_with_type() {
        let output = compute!{ v_bind, v_unit => 
            x <- vec![1,2,3];
            let y : u32 = 7;
            w <- vec![1,2,3];
            unit x + y + w
        };

        assert_eq!( output, vec![9, 10, 11, 10, 11, 12, 11, 12, 13] );
    }

    #[test]
    fn should_compute_with_let_with_pattern() {
        let output = compute!{ v_bind, v_unit => 
            x <- vec![1,2,3];
            let (y, h) = (7, 8);
            w <- vec![1,2,3];
            unit x + y + w + h
        };

        assert_eq!( output, vec![17, 18, 19, 18, 19, 20, 19, 20, 21] );
    }

    #[test]
    fn should_compute_with_let_with_pattern_with_type() {
        let output = compute!{ v_bind, v_unit => 
            x <- vec![1,2,3];
            let (y, h) | (u32, u32) = (7, 8);
            w <- vec![1,2,3];
            unit x + y + w + h
        };

        assert_eq!( output, vec![17, 18, 19, 18, 19, 20, 19, 20, 21] );
    }

    #[test]
    fn should_compute_with_block_expr_in_let() {
        let output = compute!{ v_bind, v_unit => 
            x <- vec![1,2,3];
            let y = { let a = 1;
                      let b = 2;
                      a + b
                    };
            w <- vec![1,2,3];
            unit x + y + w 
        };

        assert_eq!( output, vec![5, 6, 7, 6, 7, 8, 7, 8, 9] );
    }
}