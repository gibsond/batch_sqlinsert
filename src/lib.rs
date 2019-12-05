#[macro_use]
extern crate postgres;
#[macro_use]
extern crate postgres_derive;
extern crate chrono;
//#[macro_use]
#[cfg(test)]
extern crate r2d2_postgres;
//#[macro_use(info, log)]
//extern crate r2d2;


use postgres::{Connection, TlsMode};
//use postgres::{Connection, TlsMode, transaction };
use postgres::types::ToSql;
//use r2d2_postgres::{TlsMode, PostgresConnectionManager};

use std::io;
use chrono::prelude::*;

// Oct 15, 2019 - For tests
//use postgres::{Connection, transaction};
//use postgres::types::ToSql;

//#[cfg(test)]
//use r2d2_postgres::{TlsMode, PostgresConnectionManager};

pub mod batcher;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn test_batch() {
        use r2d2_postgres::{TlsMode, PostgresConnectionManager};
        use postgres::types::Timestamp;
        use chrono::prelude::*;
        use batcher::*;
        //use r2d2::*;
        // CREATE TABLE test_batcher(pk serial NOT NULL PRIMARY KEY, date_time timestamp with time zone, myint integer, mystring varchar(60));
        // INSERT INTO test_batcher (date_time, myint, mystring) VALUES ( '2019-10-15 11:00 -0400', 0, 'Hello');
        // INSERT INTO test_batcher (date_time, myint, mystring) VALUES ( '20191015 11:00 -0400', 1, 'Goodbye');
        let dsn = "postgresql://username:pswd@ipaddr:port/database_name";
        let manager = PostgresConnectionManager::new(dsn, TlsMode::None).unwrap();
        let dpool = r2d2::Pool::builder()
            .max_size(100)
            .build(manager)
            .unwrap();
        let mut conn = dpool.get().unwrap();
        let mut table_pks: Vec<i32> = Vec::new(); 
        
        let mut batcher = InsertBatcher::new(&conn, "test_batcher", vec!["date_time".to_string(), "myint".to_string(), "mystring".to_string()]);

        let mut variants: Vec<Variant> = Vec::new();
        let the_date_str = String::from("2019-10-15 11:00:00 -0400");
        // Commented format below is for if you use let mylocal:DateTime<Local> = Local::now();
        //let fmt = "%Y-%b-%d %H:%M:%S %z";
        let fmt = "%Y-%m-%d %H:%M:%S %z";
        let the_date =  match DateTime::parse_from_str(&the_date_str, fmt) {
            Ok(date) => date,
            Err(error) => {
                println!("mystr: {:?}", &the_date_str);
                panic!("Date error: {:?}", error);
            },
        };
        
        //variants.push(Variant::stringValue(the_date));
        // The below has ::NegInfinity, ::PosInfinity as well as ::Value
        variants.push(Variant::timeStampValue(Timestamp::Value(the_date)));
        let myint = 2;
        variants.push(Variant::i32Value(myint));
        let mystring = String::from("Daniel");
        variants.push(Variant::stringValue(mystring));

        batcher.insert_batch(variants);

        match batcher.execute() {
            Ok(value) => { table_pks.extend(value); println!("Successful Insert!");},
            Err(error) => { panic!("DaqMsgs batcher execute error, Error: {:?}", error); },    
        };
        
    }
    
}
