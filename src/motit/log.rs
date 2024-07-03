pub fn log_info<T>(str:String)
{
    let log = format!("[{}][INFO]{}", std::any::type_name::<T>(), str);

    println!("{}", log);
}

pub fn log_error<T>(str:String)
{
    let log = format!("[{}][ERROR]{}", std::any::type_name::<T>(), str);

    println!("{}", log);
}
