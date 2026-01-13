#![cfg(all(feature = "chrono", feature = "jiff-0_2"))]

use async_graphql::*;

#[tokio::test]
async fn test_both_backends_coexist() {
    struct Query;

    #[Object]
    impl Query {
        async fn get_chrono_datetime(&self) -> chrono::DateTime<chrono::Utc> {
            chrono::Utc::now()
        }

        async fn get_jiff_datetime(&self) -> jiff::Zoned {
            "2024-01-15T10:30:00Z".parse().unwrap()
        }
    }

    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);

    // Test that ChronoDateTime scalar exists
    let query = r#"{ __type(name: "ChronoDateTime") { name } }"#;
    let res = schema.execute(query).await.into_result().unwrap().data;
    assert_eq!(
        res,
        value!({
            "__type": {
                "name": "ChronoDateTime"
            }
        })
    );

    // Test that JiffDateTime scalar exists
    let query = r#"{ __type(name: "JiffDateTime") { name } }"#;
    let res = schema.execute(query).await.into_result().unwrap().data;
    assert_eq!(
        res,
        value!({
            "__type": {
                "name": "JiffDateTime"
            }
        })
    );
}

#[tokio::test]
async fn test_chrono_naive_types_prefixed() {
    struct Query;

    #[Object]
    impl Query {
        async fn get_naive_date(&self) -> chrono::NaiveDate {
            chrono::NaiveDate::from_ymd_opt(2024, 1, 15).unwrap()
        }

        async fn get_naive_time(&self) -> chrono::NaiveTime {
            chrono::NaiveTime::from_hms_opt(10, 30, 0).unwrap()
        }

        async fn get_naive_datetime(&self) -> chrono::NaiveDateTime {
            chrono::NaiveDate::from_ymd_opt(2024, 1, 15)
                .unwrap()
                .and_hms_opt(10, 30, 0)
                .unwrap()
        }
    }

    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);

    // Test ChronoNaiveDate
    let query = r#"{ __type(name: "ChronoNaiveDate") { name } }"#;
    let res = schema.execute(query).await.into_result().unwrap().data;
    assert_eq!(
        res,
        value!({
            "__type": {
                "name": "ChronoNaiveDate"
            }
        })
    );

    // Test ChronoNaiveTime
    let query = r#"{ __type(name: "ChronoNaiveTime") { name } }"#;
    let res = schema.execute(query).await.into_result().unwrap().data;
    assert_eq!(
        res,
        value!({
            "__type": {
                "name": "ChronoNaiveTime"
            }
        })
    );

    // Test ChronoNaiveDateTime
    let query = r#"{ __type(name: "ChronoNaiveDateTime") { name } }"#;
    let res = schema.execute(query).await.into_result().unwrap().data;
    assert_eq!(
        res,
        value!({
            "__type": {
                "name": "ChronoNaiveDateTime"
            }
        })
    );
}

#[tokio::test]
async fn test_jiff_civil_types_exist() {
    struct Query;

    #[Object]
    impl Query {
        async fn get_date(&self) -> jiff::civil::Date {
            "2024-01-15".parse().unwrap()
        }

        async fn get_time(&self) -> jiff::civil::Time {
            "10:30:00".parse().unwrap()
        }

        async fn get_datetime(&self) -> jiff::civil::DateTime {
            "2024-01-15T10:30:00".parse().unwrap()
        }
    }

    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);

    // Test JiffDate
    let query = r#"{ __type(name: "JiffDate") { name } }"#;
    let res = schema.execute(query).await.into_result().unwrap().data;
    assert_eq!(
        res,
        value!({
            "__type": {
                "name": "JiffDate"
            }
        })
    );

    // Test JiffTime
    let query = r#"{ __type(name: "JiffTime") { name } }"#;
    let res = schema.execute(query).await.into_result().unwrap().data;
    assert_eq!(
        res,
        value!({
            "__type": {
                "name": "JiffTime"
            }
        })
    );

    // Test JiffCivilDateTime
    let query = r#"{ __type(name: "JiffCivilDateTime") { name } }"#;
    let res = schema.execute(query).await.into_result().unwrap().data;
    assert_eq!(
        res,
        value!({
            "__type": {
                "name": "JiffCivilDateTime"
            }
        })
    );
}

#[tokio::test]
#[cfg(feature = "chrono-duration")]
async fn test_chrono_duration_prefixed() {
    struct Query;

    #[Object]
    impl Query {
        async fn get_duration(&self) -> chrono::Duration {
            chrono::Duration::hours(2)
        }
    }

    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);

    let query = r#"{ __type(name: "ChronoDuration") { name } }"#;
    let res = schema.execute(query).await.into_result().unwrap().data;
    assert_eq!(
        res,
        value!({
            "__type": {
                "name": "ChronoDuration"
            }
        })
    );
}

#[tokio::test]
async fn test_jiff_span_exists() {
    struct Query;

    #[Object]
    impl Query {
        async fn get_span(&self) -> jiff::Span {
            "P1DT2H".parse().unwrap()
        }
    }

    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);

    let query = r#"{ __type(name: "JiffSpan") { name } }"#;
    let res = schema.execute(query).await.into_result().unwrap().data;
    assert_eq!(
        res,
        value!({
            "__type": {
                "name": "JiffSpan"
            }
        })
    );
}
