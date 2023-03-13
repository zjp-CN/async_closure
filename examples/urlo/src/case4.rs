use async_closure::{async_closure_mut as cb, capture_lifetimes::AsyncFnMut};
type Result<T> = ::std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn main() {
    println!("Hello, world!");
    let db = AppDatabase(0);
    let r = db
        .execute_transaction(
            cb!({}; async |db: &AppDatabase, session: &mut DatabaseSession| -> Result<()> {
                let val = 5;
                db.insert_with_session(val, session).await
            }),
        )
        .await;
    println!("{:?}", r);
}

struct AppDatabase(i32);
struct DatabaseSession(i32);

impl DatabaseSession {
    fn commit(&self) {}
    fn abort(&self) {}
}

impl AppDatabase {
    async fn execute_transaction<'env, F>(&self, mut f: F) -> Result<()>
    where
        F: for<'any> AsyncFnMut<
            'env,
            (&'any AppDatabase, &'any mut DatabaseSession),
            Output = Result<()>,
        >,
    {
        let mut session = DatabaseSession(0);
        let result = f.call_mut((self, &mut session)).await;
        if result.is_err() {
            session.abort();
            println!("rollback transaction here");
        } else {
            session.commit();
            println!("commit transaction here");
        }
        Ok(())
    }

    async fn insert_with_session(&self, val: i32, session: &mut DatabaseSession) -> Result<()> {
        println!("some dummy insert operation here");
        session.0 = val;
        Ok(())
    }
}
