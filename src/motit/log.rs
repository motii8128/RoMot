pub fn log_info(name:&str, str:&str)
{
    let log = format!("[{}][INFO]{}", name, str);

    println!("{}", log);
}

pub fn log_error(name:&str, str:&str)
{
    let log = format!("[{}][ERROR]{}", name, str);

    println!("{}", log);
}
