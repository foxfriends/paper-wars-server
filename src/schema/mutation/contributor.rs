use super::{Context, Contributor, Mutation};
use data::{contributors, ContributorRole};
use diesel::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(juniper::GraphQLInputObject)]
pub struct InviteContributor {
    account_id: Uuid,
    universe_id: Uuid,
}

#[derive(juniper::GraphQLInputObject)]
pub struct ContributorInvitation {
    universe_id: Uuid,
}

impl Mutation {
    pub(super) fn invite_contributor(
        &self,
        context: &Context,
        contributor: InviteContributor,
    ) -> anyhow::Result<Contributor> {
        let account_id = context.try_authenticated_account()?;
        let invitation = context.transaction(|conn| {
            self.assert_universe_owner(contributor.universe_id, account_id, conn)?;
            let existing_contributor = contributors::table
                .filter(contributors::account_id.eq(contributor.account_id))
                .filter(contributors::universe_id.eq(contributor.universe_id))
                .filter(contributors::role.ne(ContributorRole::Declined));
            let contributor_exists: bool = select(exists(existing_contributor)).get_result(conn)?;
            anyhow::ensure!(
                !contributor_exists,
                "That account ({}) is already a contributor to this universe ({})",
                contributor.account_id,
                contributor.universe_id,
            );

            let invitation: data::Contributor = insert_into(contributors::table)
                .values((
                    contributors::universe_id.eq(contributor.universe_id),
                    contributors::account_id.eq(contributor.account_id),
                    contributors::role.eq(ContributorRole::Pending),
                ))
                .returning(contributors::all_columns)
                .get_result(conn)?;
            Ok(invitation)
        })?;
        let query = Contributor::new(invitation.universe_id, invitation.account_id);
        context.contributors().prime(invitation);
        Ok(query)
    }

    pub(super) fn respond_to_contributor_invitation(
        &self,
        context: &Context,
        ContributorInvitation { universe_id }: ContributorInvitation,
        accepted: bool,
    ) -> anyhow::Result<Contributor> {
        let account_id = context.try_authenticated_account()?;
        let contributor = context.transaction(|conn| {
            let mut contributor: data::Contributor = contributors::table
                .filter(contributors::universe_id.eq(universe_id))
                .filter(contributors::account_id.eq(account_id))
                .filter(contributors::role.eq(ContributorRole::Pending))
                .get_result(conn)
                .map_err(|_| {
                    anyhow::anyhow!(
                        "You ({}) havenot been invited to contribute to this universe ({}).",
                        account_id,
                        universe_id
                    )
                })?;
            contributor.role = if accepted {
                ContributorRole::Contributor
            } else {
                ContributorRole::Declined
            };
            update(&contributor)
                .set(contributors::role.eq(contributor.role))
                .execute(conn)?;
            Ok(contributor)
        })?;
        let query = Contributor::new(contributor.universe_id, contributor.account_id);
        context.contributors().prime(contributor);
        Ok(query)
    }
}
