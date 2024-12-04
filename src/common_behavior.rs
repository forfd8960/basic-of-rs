#[derive(Debug)]
pub enum Status {
    Pending,
    Scheduled,
    Running,
    Idle,
    Stopped,
}

pub trait AppRunner {
    fn run_app(&self) -> Status;
}

pub struct Mac {}

impl AppRunner for Mac {
    fn run_app(&self) -> Status {
        println!("Mac runing app");
        Status::Pending
    }
}

pub struct IPhone {}

impl AppRunner for IPhone {
    fn run_app(&self) -> Status {
        println!("IPhone runing app");
        Status::Scheduled
    }
}

pub struct Android {}

impl AppRunner for Android {
    fn run_app(&self) -> Status {
        println!("Android runing app");
        Status::Running
    }
}

pub fn run_apps(runners: Vec<Box<dyn AppRunner>>) {
    for runner in runners {
        println!("{:?}", runner.run_app());
    }
}
