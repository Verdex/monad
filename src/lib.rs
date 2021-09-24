#[macro_export]
macro_rules! compute{
    ($bind:expr, $unit:expr => unit $e:expr) => {
        $unit($e)
    };

    ($bind:expr, $unit:expr => $var:ident <- $e:expr; $($rest:tt)+) => {
        $bind($e, move |$var| compute!($bind, $unit => $($rest)+))
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
}
