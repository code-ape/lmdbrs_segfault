use env_logger;
use std::path::Path;
use lmdb::{EnvBuilder, DbFlags};

// general action that works exactly
// as expected, no issues
pub fn action1() {
    env_logger::init().unwrap();
    debug!("checkpoint 1.A");
    let num : u8 = {
        match get_vec() {
            Ok(val) => {
                let val2 = val[0];
                val2
            },
            Err(_)  => {
                0
            }
        }
    };
    debug!("checkpoint 1.B");
    debug!("num = {}", num);

}

fn get_vec() -> Result<Vec<u8>,()> {
    Ok(vec![1,2,3])
}

pub fn action2() {
    let path = Path::new("lmdb_data");
    let env = Box::new(EnvBuilder::new().open(path, 0o777).unwrap());

    debug!("checkpoint 2.A");
    let num : u8 = {
        let db_handle = env.get_default_db(DbFlags::empty()).unwrap();
        let reader = env.get_reader().unwrap();
        let db_ref = reader.bind(&db_handle);
        match db_ref.get::<&[u8]>(&"log/0") {
            Ok(val) => {
                let val2 = val[0];
                val2
            },
            Err(_)  => {
                0
            }
        }
    };
    debug!("checkpoint 2.B");
    println!("num = {}", num);
}
