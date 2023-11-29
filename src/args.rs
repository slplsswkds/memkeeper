pub struct Args {
    pub app: String,
    pub args: Vec<String>
}

impl TryFrom<Vec<String>> for Args {
    type Error = String;

    fn try_from(mut args: Vec<String>) ->  Result<Self, Self::Error> {
        if args.len() > 1 {
            args.remove(0);
            let app = args.remove(0);
            Ok(Self {
                app: app,
                args: args
            })
        } else {
            Err("Invalid number of elements in the input vector".to_string())
        }
    }
}