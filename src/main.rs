use basic_of_rs::common_behavior::{run_apps, Android, AppRunner, IPhone, Mac};

fn main() {
    println!("Hello, world!");

    let system: Vec<Box<dyn AppRunner>> =
        vec![Box::new(Mac {}), Box::new(IPhone {}), Box::new(Android {})];

    /*
    Hello, world!
    Mac runing app
    Pending
    IPhone runing app
    Scheduled
    Android runing app
    Running
    */
    run_apps(system);
}
