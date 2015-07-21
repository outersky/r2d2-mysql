#![crate_name="r2d2_mysql"]
#![crate_type="rlib"]
#![crate_type="dylib"]

extern crate mysql;
extern crate rustc_serialize as serialize;
extern crate r2d2;

use std::collections::HashMap;
use std::fmt;
use std::iter::FromIterator;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::Debug;

use mysql::conn::QueryResult;
use mysql::consts::ColumnType;
use mysql::value::{Value,FromValue,from_value};

mod param;
mod url;
mod pool;

pub use pool::MysqlConnectionManager;
pub use param::connect;

/// Column info 
#[derive(Debug)]
pub struct Column {
    name: String,
    column_type: ColumnType,
}

/// The resulting rows of a query.
// RowSet returned from Statement
#[derive(Debug)]
pub struct RowSet {
    pub columns: Rc<Vec<Column>>,
    pub rows: RefCell<Vec<Row>>, // Could be 0-size
}

/// one Row data in RowSet 
pub struct Row {
    pub data: Vec<Value>,
    pub rowset: Rc<RowSet>,
}

impl RowSet {
    pub fn columns_ref(&self) -> &[Column] {
        let ref columns = self.columns; 
        columns.as_ref()
    }

    pub fn column_index(&self, name:&str) -> Option<usize> {
        self.columns.iter().position(|d| d.name == name)
    }

    pub fn add(&self, row:Row){
        self.rows.borrow_mut().push(row);
    }

    /// return HashMap<column_index, column_name>
    pub fn column_name_map(&self) -> HashMap<usize,String> { 
        HashMap::from_iter(self.columns_ref().iter().map(|column| column.name.to_string()).enumerate())
    }

}

impl Debug for Row {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result{
    	write!(fmt,"Row({})", self.data.len())
    }
}

impl Iterator for RowSet {
    type Item = Row;
    fn next(&mut self) -> Option<Row> {
        self.rows.borrow_mut().pop()
    }
}

impl Row {
    /// get Option value from row
    pub fn get_opt<I, T>(&self, idx: I) -> Option<T> where I: RowIndex + fmt::Debug + Clone, T: FromValue {
        idx.idx(self).map(|index|
            mysql::value::from_value::<T>(&self.data[index])
        )
    }

    /// get value from row: let name : String = row.get(0); or let name : String = row.get("current_user");
    /// will panic if idx does not exist
    pub fn get<I, T>(&self, idx: I) -> T where I: RowIndex + fmt::Debug + Clone, T: FromValue {
        let index = idx.idx(self).unwrap();
        mysql::value::from_value::<T>(&self.data[index])
    }
}

impl std::ops::Index<usize> for Row {
    type Output = Value;

    fn index<'a>(&'a self, _index: usize) -> &'a Value {
        &self.data[_index]
    }
}

/// A trait implemented by types that can index into columns of a row.
pub trait RowIndex {
    /// Returns the index of the appropriate column, or `None` if no such
    /// column exists.
    fn idx(&self, row: &Row) -> Option<usize>;
}

impl RowIndex for usize {
    #[inline]
    fn idx(&self, row: &Row) -> Option<usize> {
        if *self >= row.rowset.columns.len() {
            None
        } else {
            Some(*self)
        }
    }
}

impl<'a> RowIndex for &'a str {
    #[inline]
    fn idx(&self, row: &Row) -> Option<usize> {
        row.rowset.column_index(*self)
    }
}

trait ToRowSet {
    // Add code here
    fn to_rowset(&mut self, columns:Vec<Column>) -> Rc<RowSet>;
}

impl<'conn> ToRowSet for QueryResult<'conn> {
    fn to_rowset(&mut self, columns:Vec<Column>) -> Rc<RowSet>{
        let columns = Rc::new(columns);

        let rowset = RowSet {
            columns: columns.clone(),
            rows: RefCell::new(Vec::new()),
        };
        let rowset = Rc::new(rowset);
        for row_data in self { 
            let rowset = rowset.clone();
            let row = Row{
                data: row_data.unwrap(),
                rowset: rowset.clone(),
            };
            rowset.add(row);
        }
        rowset
    }
}

pub fn get_columns(columns : &Option<&[mysql::conn::Column]>) -> Vec<Column> {
    let mut column_list = Vec::new();
    if columns.is_some() {
        columns.map(|cs|{
            cs.iter().fold((),|_,c| {
                column_list.push(Column{
                    name : String::from_utf8(c.name.clone()).unwrap(),
                    column_type: c.column_type,
                });
                ()
            });
        });
    }
    column_list
}

/*
//Error: Compiler bug.
pub fn get_columns_fn(columns : &Option<&[mysql::conn::Column]>) -> Box<FnMut(QueryResult) -> Rc<RowSet>> {
    let columns = get_columns(columns);

    Box::new(move |mut result: QueryResult|{
        result.to_rowset(columns)
    })
}
*/

/// can be used as : conn.query(sql).map(to_rowset).map(|rowset:Rc<RowSet>| { ... });
pub fn to_rowset(mut result: QueryResult, columns : Vec<Column>) -> Rc<RowSet> {
    result.to_rowset(columns)
}

#[cfg(test)]
mod test {
    use mysql::conn::MyConn;
    use r2d2;
    use std::fmt;
    use std::rc::Rc;
    use std::sync::Arc;
    use std::thread;
    use super::{RowSet,to_rowset,get_columns,MysqlConnectionManager,connect};

    const DB_URL : &'static str =  "mysql://root:12345678@localhost:3306/test";

    //#[derive(Debug,Display)] 
    pub struct Person {
        id: i64,
        name:String,
        not_exist:i64,
    }

    impl Person {
        fn _fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, r#"Person{{id:{}, name:"{}", not_exist:{} }}"#, self.id, self.name,self.not_exist)
        }
    }
    impl fmt::Debug for Person {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self._fmt(f)
        }
    }

    fn get_connect() -> MyConn {
        connect(DB_URL).unwrap()
    }

    fn init<'a>(conn: &'a mut MyConn){
        assert!(conn.query("CREATE TEMPORARY TABLE tbl_person(\
                                    id INT,\
                                    name varchar(30),\
                                    create_time DATETIME\
                                )").is_ok());

        let _ = conn.prepare("INSERT INTO tbl_person(id,name,create_time) VALUES (?, ?, now())")
                .map(|mut stmt| {
                    assert!(stmt.execute(&[&1,&b"tom".to_vec(),]).is_ok());
                    assert!(stmt.execute(&[&2,&b"amy".to_vec(),]).is_ok());
                }).unwrap();
    }

    #[test]
    fn query_any(){
        let conn =&mut get_connect();
        init(conn);
        let sql = "drop table tbl_person";
        assert!(conn.query(sql).is_ok());
    }

    #[test]
    fn query_struct(){
        let conn =&mut get_connect();
        init(conn);
        let sql = "select id,name,create_time from tbl_person";
        let list = conn.prepare(sql).map(|mut stmt|{
            let columns = get_columns(& stmt.columns_ref()); // Vec<Column>
            let rowset = stmt.execute(&[]).map(|qr| to_rowset(qr,columns) );  // Option<Rc<RowSet>>
            rowset.map(|rowset:Rc<RowSet>| {
                let rows = rowset.rows.borrow();
                rows.iter().map(|row|
                    Person {
                        id: row.get("id"),
                        name:row.get("name"),
                        not_exist:row.get_opt("not_exist").unwrap_or(-1),
                    }
                ).collect::<Vec<Person>>()
            }).map_err(|err| println!("execute statement error in line:{} ! error: {:?}", line!(), err) )
        }).map_err(|err| println!("prepare query error in line:{} ! error: {:?}", line!(), err) );
        assert!(list.is_ok());
        assert_eq!(list.unwrap().unwrap().len(),2);
    }

    #[test]
    fn query_pool(){
        let config = r2d2::config::Builder::new().pool_size(30).build();   // r2d2::Config::default()
        let manager = MysqlConnectionManager::new(DB_URL).unwrap();
        let pool = Arc::new(r2d2::Pool::new(config, manager).unwrap());

        let mut tasks = vec![];

        for _ in 0..3 {
            let pool = pool.clone();
            let th = thread::spawn(move || {
                let mut conn = pool.get().map_err(|err| println!("get connection from pool error in line:{} ! error: {:?}", line!(), err) ).unwrap();
                conn.query("select user()").map_err(|err| println!("execute query error in line:{} ! error: {:?}", line!(), err) ).unwrap();
            });
            tasks.push(th);
        }

        for th in tasks {
            let _ = th.join();
        }

    }
}
