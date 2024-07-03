pub trait Logger {
    fn log_info(str:String)
    {
        let log = format!("[{}][INFO]{}", std::any::type_name::<Self>(), str);

        println!("{}", log);
    }

    fn log_error(str:String)
    {
        let log = format!("[{}][ERROR]{}", std::any::type_name::<Self>(), str);

        println!("{}", log);
    }
}