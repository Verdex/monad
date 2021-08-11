#[macro_export]
macro_rules! compute{
    ($bind:expr, $unit:expr => unit $e:expr) => {
        $unit($e)
    };

    ($bind:expr, $unit:expr => $var:ident <- $e:expr; $($rest:tt)+) => {
        $bind($e, |$var| compute!($bind, $unit => $($rest)+))
    };
}

#[cfg(test)]
mod tests {
    fn unit<T>( t : T ) -> Vec<T> {
        vec![t]
    }

    fn bind<T, S, F : Fn(T) -> Vec<S>>( a : Vec<T>, next : F ) -> Vec<S> {
        fn map<T, S, F : Fn(T) -> S>(f : F, t : Vec<T>) -> Vec<S> {
            t.into_iter().map(f).collect()
        }

        fn mu<T>(t : Vec<Vec<T>>) -> Vec<T> {
            t.into_iter().flatten().collect()
        }

        mu(map(next, a))
    }

    #[test] 
    fn should_work_with_just_unit() {
        let output = compute!( bind, unit => unit 8 );
        assert_eq!( output, vec![8] );
    }

    #[test]
    fn should_compute() {
        let output = compute!{ bind, unit => 
            x <- vec![1,2,3];
            y <- vec![1,2,3];
            unit x + y
        };

        assert_eq!( output, vec![2, 3, 4, 3, 4, 5, 4, 5, 6] );
    }

    #[test]
    fn should_compute_with_single_bind() {
        let output = compute!{ bind, unit => 
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

        let output = compute!{ bind, unit => 
            x <- vec![1,2,3];
            w <- create_vec(x);
            unit w
        };

        assert_eq!( output, vec![1, 1, 1, 2, 2, 2, 3, 3, 3] );
    }
}
