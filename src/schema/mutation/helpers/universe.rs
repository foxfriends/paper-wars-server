use super::{Context, Mutation};
use data::ContributorRole;
use uuid::Uuid;

impl Mutation {
    pub fn assert_universe_owner(
        &self,
        context: &Context,
        universe_id: Uuid,
        account_id: Uuid,
    ) -> anyhow::Result<()> {
        let is_universe_owner = context
            .contributors()
            .load((universe_id, account_id))
            .map(|relationship| relationship.role == ContributorRole::Owner)
            .unwrap_or(false);
        anyhow::ensure!(
            is_universe_owner,
            "You ({}) are not the owner of this universe ({})",
            account_id,
            universe_id,
        );
        Ok(())
    }
}
