#Batch_sqlinsert: Postgres Insert Batcher

Provides fast inserting of high volumes of rows in a Postgres table.

Typically used in a Rust language project using cargo.

##Usage:

Assuming an existing table called test_batcher with a date_time, myint, and mystring column:

\d+ test_batcher
                                                          Table "public.test_batcher"
  Column   |           Type           | Collation | Nullable |                 Default                  | Storage  |
-----------+--------------------------+-----------+----------+------------------------------------------+----------+
 pk        | integer                  |           | not null | nextval('test_batcher_pk_seq'::regclass) | plain    |
 date_time | timestamp with time zone |           |          |                                          | plain    |
 myint     | integer                  |           |          |                                          | plain    |
 mystring  | character varying(60)    |           |          |                                          | extended |
Indexes:
    "test_batcher_pkey" PRIMARY KEY, btree (pk)


Set up a postgresql connection, say conn

let mut batcher = InsertBatcher::new(&conn, "test_batcher", vec!["date_time".to_string(), "myint".to_string(), "mystring".to_string()]);

let mut variants: Vec<Variant> = Vec::new();

variants.push(Variant::timeStampValue(Timestamp::Value(the_date)));
let myint = 2;
variants.push(Variant::i32Value(myint));
let mystring = String::from("Test");
variants.push(Variant::stringValue(mystring));

 batcher.insert_batch(variants);

- You can loop through and insert many of these records of Variants

- Then finally, for the database insert:

match batcher.execute() {
      Ok(value) => { table_pks.extend(value); println!("Successful Insert!");},
      Err(error) => { panic!("DaqMsgs batcher execute error, Error: {:?}", error); },    
};