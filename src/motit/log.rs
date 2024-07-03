pub trait Log {
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

impl<T> Log for T {
    fn log_info(str:String) {
        let log = format!("[{}][INFO]{}", std::any::type_name::<T>(), str);

        println!("{}", log);
    }

    fn log_error(str:String){
        let log = format!("[{}][ERROR]{}", std::any::type_name::<T>(), str);

        println!("{}", log);
    }
}