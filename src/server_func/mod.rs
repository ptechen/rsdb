use std::collections::HashMap;
use std::sync::Mutex;


lazy_static!{
    static ref KEY_VAL_STRING:Mutex<HashMap<&'static str, &'static str>> = Mutex::new(HashMap::new());
    static ref KEYI32_VAL_STRING:Mutex<HashMap<i32, &'static str>> = Mutex::new(HashMap::new());
}

pub trait Data {
    fn set(key:String, val:String) -> i64;
}

pub struct Database;

impl Data for Database {
    fn set(key: String, val: String) -> i64 {
        let key_str = Box::leak(key.into_boxed_str());
        let val_str = Box::leak(val.into_boxed_str());
        let mut a = KEY_VAL_STRING.lock().unwrap();
        a.insert(key_str, val_str);
        return 0
    }
}


pub async fn set(key: String, val: String) -> Result<i64, Box<dyn std::error::Error>>  {
    let key_str = Box::leak(key.into_boxed_str());
    let val_str = Box::leak(val.into_boxed_str());
    let mut a = KEY_VAL_STRING.lock().unwrap();
    let res = a.insert(key_str, val_str);
    Ok(0)
}