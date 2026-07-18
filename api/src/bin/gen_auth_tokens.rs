use thing_ranker::account::{create_login_token, upsert_account};
use thing_ranker::app::Config;
use thing_ranker::db::{MIGRATOR, create_pool};

#[tokio::main]
async fn main() {
    let config = Config::load();
    let pool = create_pool(&config.db).await;
    MIGRATOR.run(&pool).await.unwrap();
    for account_role in config.roles {
        let account = upsert_account(&account_role.email, account_role.role, &pool)
            .await
            .unwrap();
        let account_jwt =
            create_login_token(account, &config.auth.jwt_secret, config.auth.jwt_exp_secs)
                .await
                .unwrap();
        println!();
        println!("{}", account_role.email);
        println!("{account_jwt}");
    }
}
