
        use thiserror::Error;

        #[derive(Error, Debug)]
        pub enum CustomError {
            #[error("An error occurred.")]
            AnError,
        }
        