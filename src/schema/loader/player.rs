use super::Loader;
use data::Player;
use uuid::Uuid;

batch_fn!(players => Player { game_id: Uuid, account_id: Uuid });

impl Loader<(Uuid, Uuid), Player> {
    join!(players => for_account(account_id: Uuid) -> Player);
}
