
#![feature(macro_rules)]

pub trait ToSql {
    fn to_sql(&self) -> String;
}

#[deriving(Clone)]
pub enum ExprValue<T> {
    ExpressionValue,
    DefaultValue
}

#[allow(dead_code)]
#[deriving(Clone)]
pub enum Insert<T, V, M> {
    InsertDefaultValues,
    InsertValues(Vec<V>)
}

#[allow(dead_code)]
#[deriving(Clone)]
pub struct InsertQuery<T, V, M> {
    pub values: Insert<T, V, M>
}

impl<T,V,M> InsertQuery<T,V,M> {
    pub fn get_values(&self) -> &Insert<T, V, M> { &self.values }
}

macro_rules! to_sql_for_insert_tuple(
    ($fmt:expr, $($t:ident, $var:ident),+) => (
        impl<$($t,)+> ToSql for ($(ExprValue<$t>),+,)  {
            fn to_sql(&self) -> String {
                "".to_string()
            }
        }

    )
)

impl ToSql for ()  {
    fn to_sql(&self) -> String {
        "DEFAULT VALUES".to_string()
    }
}

to_sql_for_insert_tuple!("{}", T1, t1)
to_sql_for_insert_tuple!("{}, {}", T1, t1, T2, t2)

impl<T: Clone, V: ToSql, M: Clone> ToSql for Insert<T, V, M> {
    fn to_sql(&self) -> String {
        match self {
            &InsertDefaultValues => {
                "DEFAULT VALUES".to_string()
            },
            &InsertValues(ref rows) => {
                let val: Vec<&V> = rows.iter().collect();
                "".to_string()
            }
            
        }
    }
}

impl<T: Clone, V: Clone+ToSql, M: Clone> ToSql for InsertQuery<T, V, M> {
    fn to_sql(&self) -> String {
        // ..
        format!("{}", self.get_values().to_sql())
    }
}