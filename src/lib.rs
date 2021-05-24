pub mod utils {
    use std::ops::Shr;
    pub struct Pipe<T>(T);

    impl <T> Pipe<T> {
        pub fn new(content: T) -> Pipe<T> {
            Pipe(content)
        }
    }

    impl <A, B, F> Shr<F> for Pipe<A> 
    where 
        F: FnOnce(A) -> B,
    {
        type Output = Pipe<B>;

        fn shr(self, func: F) -> Self::Output {
            Pipe(func(self.0))
        }
    }
}



