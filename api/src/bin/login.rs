use thing_ranker::account::{create_login_token, upsert_account};
use thing_ranker::app::Config;
use thing_ranker::db::create_pool;

/// CLI utility that prints out auth tokens for accounts specified in the
/// `account_settings` section of config.yml file(s).
/// This is useful for local manual testing via an HTTP client like Postman or Bruno.
#[tokio::main]
async fn main() {
    let config = Config::load();
    let pool = create_pool(&config.db, true).await;
    for account_settings in config.account_settings {
        let account = upsert_account(&account_settings.email, account_settings.role, &pool)
            .await
            .unwrap();
        let account_jwt =
            create_login_token(account, &config.auth.jwt_secret, config.auth.jwt_exp_secs)
                .await
                .unwrap();
        println!();
        println!("{}", account_settings.email);
        println!("{account_jwt}");
    }
}
