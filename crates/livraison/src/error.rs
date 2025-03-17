use quick_error::quick_error;

quick_error! {
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum LivraisonError {
        Io(err: std::io::Error) {
            from()
            display("{}", err)
            source(err)
        }
        Utf8(err: std::str::Utf8Error) {
            display("utf8 error")
        }
    }
}

pub type LivraisonResult<T> = Result<T, LivraisonError>;
