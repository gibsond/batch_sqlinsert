use super::postgres;
use super::ToSql;
use super::postgres::types::Timestamp;
use chrono::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Variant {
    stringValue(String),
    boolValue(bool),
    i32Value(i32),
    u32Value(u32),
    i64Value(i64),
    //u64Value(u64),
    f32Value(f32),
    f64Value(f64),
    //dateValue(DateTime<UTC>)
    //timeStampValue(Timestamp<String>),
    //timeStampValue(Timestamp<DateTime<Local>>),
    timeStampValue(Timestamp<DateTime<FixedOffset>>),
    //timeStampValue(Timestamp<TimeZone>),
    optionI32Value(Option<i32>),
    
}

pub struct InsertBatcher<'a> {
    conn: &'a postgres::Connection,
    table_name: String,
    column_names: Vec<String>,
    original_insert_string: String,
    insert_string: String,
    values_string: String,
    batch_count: usize,
    batch_reset: bool,
    batched_values: Vec<Box<ToSql>>,
}

impl<'a> InsertBatcher<'a> {
    pub fn new(conn: &'a postgres::Connection, table_name: &str, column_names: Vec<String>) -> Self {

        let column_names_str = column_names.join(",");
        let sql = format!("INSERT INTO {}({}) VALUES", table_name, column_names_str);

        InsertBatcher {
            conn: conn,
            table_name: table_name.to_string(),
            column_names: column_names,
            original_insert_string: sql.clone(),
            insert_string: sql.clone(),
            values_string: "".to_string(),
            batch_count: 0,
            batch_reset: false,
            batched_values: Vec::<Box<ToSql>>::new()
        }
    }

    pub fn insert_batch(&mut self, batch : Vec<Variant>) -> Result<(),()> {
        if self.batch_reset {
            self.batch_reset = false;
            self.batched_values.truncate(0); // Empty vector so can be used again.
        }
 
        for value in batch.iter() {

            match value {
                
                &Variant::stringValue(ref v) => {
                    self.batched_values.push(Box::new(v.clone()));
                },
                &Variant::boolValue(ref v) => {
                    self.batched_values.push(Box::new(v.clone()));
                },
                &Variant::i32Value(v) => {
                    self.batched_values.push(Box::new(v.clone()));
                },
                &Variant::u32Value(v) => {
                    self.batched_values.push(Box::new(v.clone()));
                },
                &Variant::i64Value(v) => {
                    self.batched_values.push(Box::new(v.clone()));
                },
                //&Variant::u64Value(v) => {
                //    self.batched_values.push(Box::new(v.clone()));
                //},
                &Variant::f32Value(v) => {
                    self.batched_values.push(Box::new(v.clone()));
                },
                &Variant::f64Value(v) => {
                    self.batched_values.push(Box::new(v.clone()));
                },
                &Variant::timeStampValue(ref v) => {
                    self.batched_values.push(Box::new(v.clone()));
                },
                //&Variant::utcValue(ref v) => {
                //    self.batched_values.push(Box::new(v.clone()));
                //},  
                //&Variant::dateValue(ref v) => {
                //    self.batched_values.push(Box::new(v.clone()));
                //},
                &Variant::optionI32Value(v) => {
                    self.batched_values.push(Box::new(v.clone()));
                },
            };
        };
        let number_of_fields = self.column_names.len();
        let start_value = self.batch_count * number_of_fields;

        self.batch_count += 1;

        let mut place_holder_strs : Vec<String> = Vec::new();

        for i in start_value..(start_value+number_of_fields) {
            place_holder_strs.push(format!("${}", i+1));
        }

        let place_holder_str = place_holder_strs.join(",");
        self.values_string += &format!(" ({}),", place_holder_str);
        
        //println!("{}", self.insert_string);
        //println!("{}", self.values_string);

        Ok(())
        //Ok()
    }

    pub fn execute(&mut self) -> Result<(Vec<i32>),()> {
        let mut pks: Vec<i32> = Vec::new();
        // Example Fast Import
        // INSERT INTO films (code, title, did, date_prod, kind) VALUES
        //     ('B6717', 'Tampopo', 110, '1985-02-10', 'Comedy'),
        //     ('HG120', 'The Dinner Game', 140, DEFAULT, 'Comedy');

        // INSERT INTO sensor_values(timestamp,sensor_id,value) 
        //    VALUES  ($1,$2,$3), ($4,$5,$6), ($7,$8,$9), ($10,$11,$12), ($13,$14,$15), ($16,$17,$18);

        //println!("Here");

        let binds_borrowed = self.batched_values.iter().map(|s| &**s).collect::<Vec<_>>();
        //let binds_borrowed = self.batched_values.iter().map(|s| *s).collect::<Vec<_>>();
        //let binds_borrowed = self.batched_values.iter().map(|s| &format!("&{:?}",*s)).collect::<Vec<_>>();
        //println!("binds_borrowed: {:?}", binds_borrowed);
        //println!("Here2");

        // August 30, 2019 - This pop below was causing issues with aggregator.
        self.values_string.pop();
        //self.insert_string = self.insert_string.to_string() + " " + &self.values_string;
        self.insert_string = self.insert_string.to_string() + " " + &self.values_string + " RETURNING pk";

        //println!("Insert_string: {}", self.insert_string);
        //println!("binds_borrowed: {:?}", binds_borrowed);
        //println!("batch_count: {}", self.batch_count);
        // Need to check for error below.
        //self.conn.execute(&self.insert_string, &*binds_borrowed).unwrap();
        //match self.conn.execute(&self.insert_string, &*binds_borrowed) {
        match self.conn.query(&self.insert_string, &*binds_borrowed) {
        
            Err(why) => {
                 //warn!("{}", self.insert_string);
                 //bail!("error: {}", why);

                //println!("{}", self.insert_string);
                let why_string = why.to_string();
                //println!("why string len: {}", why_string.trim().len());
                // Note changing conn.execute to con.query above got rid of this annoying error
                // that did insert okay anyway.
                if why_string == "database error: ERROR: syntax error at end of input" {
                    self.batch_reset = true;
                } else { println!("error: {}", why); };
                //Err(why)
            }
            
            Ok(result) => {
                //self.batched_values.truncate(0); //empty vector so can be reused.
                //println!("Database batch insert successful, result: {:?}", result);
                // Found out from below that you do not get the primary keys back like you
                // do if you insert just one record unless you say "RETURNING pk.
                for row in result.iter() {
                    let pk:i32 = row.get(0);
                    pks.push(pk);
                    //println!("row pk: {:?}", pk);
                };
                self.batch_reset = true;
                //Ok(())
                //println!("success");
            },
         };

        //self.conn.commit();
        //self.conn.batch_execute("COMMIT").unwrap();
        //io::stdout().flush().unwrap();
        Ok((pks))
        //Ok()
    }
} // End of Impl
