use sqlx::mssql::Mssql;
use sqlx_test::test_type;

test_type!(null<Option<i32>>(Mssql,
    "CAST(NULL as INT)" == None::<i32>
));

test_type!(i8(
    Mssql,
    "CAST(5 AS TINYINT)" == 5_i8,
    "CAST(0 AS TINYINT)" == 0_i8
));

test_type!(i16(Mssql, "CAST(21415 AS SMALLINT)" == 21415_i16));

test_type!(i32(Mssql, "CAST(2141512 AS INT)" == 2141512_i32));

test_type!(i64(Mssql, "CAST(32324324432 AS BIGINT)" == 32324324432_i64));

test_type!(f32(
    Mssql,
    "CAST(3.1410000324249268 AS REAL)" == 3.141f32 as f64 as f32
));

test_type!(f64(
    Mssql,
    "CAST(939399419.1225182 AS FLOAT)" == 939399419.1225182_f64
));

test_type!(str_nvarchar<String>(Mssql,
    "CAST('this is foo' as NVARCHAR)" == "this is foo",
));

test_type!(str<String>(Mssql,
    "'this is foo'" == "this is foo",
    "''" == "",
));

test_type!(bool(
    Mssql,
    "CAST(1 as BIT)" == true,
    "CAST(0 as BIT)" == false
));

#[cfg(feature = "chrono")]
mod chrono {
    use super::*;
    use sqlx::types::chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
    test_type!(chrono_date<NaiveDateTime>(Mssql,
        "CAST('2020-07-08 01:0:0.000' as datetime)" == NaiveDate::from_ymd(2020, 7, 8).and_hms_milli(1, 0, 0, 0),
        "CAST('2020-07-08 01:41:21.900' as datetime)" == NaiveDate::from_ymd(2020, 7, 8).and_hms_milli(1, 41, 21, 900),
        "CAST('2020-07-08 01:43:18.537' as datetime)" == NaiveDate::from_ymd(2020, 7, 8).and_hms_milli(1, 43, 18, 530)
    ));
}
