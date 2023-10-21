use crate::types::accounts::{NewAccount, SaveAccountSuccess};
use crate::users::{service, storage};
use tracing::{debug, error};

pub async fn register(
    new_account: NewAccount,
    service: service::Service<impl storage::Storer>,
) -> Result<impl warp::Reply, warp::Rejection> {
    debug!("start registering a new user: {}", new_account.email);

    match service.add_account(new_account.clone()).await {
        Ok(account_id) => {
            debug!("new account was saved: {}", account_id);

            let result = SaveAccountSuccess::new(account_id);

            Ok(warp::reply::json(&result))
        }
        Err(e) => {
            error!("adding account {:?}", new_account);
            Err(warp::reject::custom(e))
        }
    }
}
