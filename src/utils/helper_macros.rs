pub mod macros {
    /**
     This is useful to concisely initialize `Vec<Vec<i32>>`, as required for this problem.
    For example, instead of doing the following (for each test case):
    ```
    vec![[1, 5]]
            .into_iter()
            .map(|x| x.to_vec())
            .collect::<Vec<Vec<i32>>>()
    ```
    This isn't necessarily performance-optimized (space not pre-allocated),
    but it gets the job done for added ease/readability in these unit tests : )
    */
    macro_rules! vec_2d {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x.to_vec());
            )*
            temp_vec
        }
    };
  }
    pub(crate) use vec_2d;
}
