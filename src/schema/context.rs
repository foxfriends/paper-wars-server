use super::{Database, Loader};
use data::*;
use diesel_citext::types::CiString;
use uuid::Uuid;

pub struct Context {
    account_loader: Loader<Uuid, Account>,
    archetype_loader: Loader<Uuid, Archetype>,
    archetype_version_loader: Loader<(Uuid, i32), ArchetypeVersion>,
    contributor_loader: Loader<(Uuid, Uuid), Contributor>,
    email_loader: Loader<CiString, Email>,
    entity_loader: Loader<Uuid, Entity>,
    game_loader: Loader<Uuid, Game>,
    login_loader: Loader<Uuid, Login>,
    map_loader: Loader<Uuid, Map>,
    map_version_loader: Loader<(Uuid, i32), MapVersion>,
    player_loader: Loader<(Uuid, Uuid), Player>,
    universe_loader: Loader<Uuid, Universe>,
    universe_version_loader: Loader<(Uuid, i32), UniverseVersion>,
    universe_version_archetype_loader: Loader<(Uuid, i32, Uuid), UniverseVersionArchetype>,
    universe_version_map_loader: Loader<(Uuid, i32, Uuid), UniverseVersionMap>,
}

impl Context {
    pub fn new(database: Database) -> Self {
        Self {
            account_loader: Loader::new(database.clone()),
            archetype_loader: Loader::new(database.clone()),
            archetype_version_loader: Loader::new(database.clone()),
            contributor_loader: Loader::new(database.clone()),
            email_loader: Loader::new(database.clone()),
            entity_loader: Loader::new(database.clone()),
            game_loader: Loader::new(database.clone()),
            login_loader: Loader::new(database.clone()),
            map_loader: Loader::new(database.clone()),
            map_version_loader: Loader::new(database.clone()),
            player_loader: Loader::new(database.clone()),
            universe_loader: Loader::new(database.clone()),
            universe_version_loader: Loader::new(database.clone()),
            universe_version_archetype_loader: Loader::new(database.clone()),
            universe_version_map_loader: Loader::new(database),
        }
    }

    pub fn accounts(&self) -> &Loader<Uuid, Account> {
        &self.account_loader
    }

    pub fn archetypes(&self) -> &Loader<Uuid, Archetype> {
        &self.archetype_loader
    }

    pub fn archetype_versions(&self) -> &Loader<(Uuid, i32), ArchetypeVersion> {
        &self.archetype_version_loader
    }

    pub fn contributors(&self) -> &Loader<(Uuid, Uuid), Contributor> {
        &self.contributor_loader
    }

    pub fn emails(&self) -> &Loader<CiString, Email> {
        &self.email_loader
    }

    pub fn entities(&self) -> &Loader<Uuid, Entity> {
        &self.entity_loader
    }

    pub fn games(&self) -> &Loader<Uuid, Game> {
        &self.game_loader
    }

    pub fn logins(&self) -> &Loader<Uuid, Login> {
        &self.login_loader
    }

    pub fn maps(&self) -> &Loader<Uuid, Map> {
        &self.map_loader
    }

    pub fn map_versions(&self) -> &Loader<(Uuid, i32), MapVersion> {
        &self.map_version_loader
    }

    pub fn players(&self) -> &Loader<(Uuid, Uuid), Player> {
        &self.player_loader
    }

    pub fn universes(&self) -> &Loader<Uuid, Universe> {
        &self.universe_loader
    }

    pub fn universe_versions(&self) -> &Loader<(Uuid, i32), UniverseVersion> {
        &self.universe_version_loader
    }

    pub fn universe_version_archetypes(
        &self,
    ) -> &Loader<(Uuid, i32, Uuid), UniverseVersionArchetype> {
        &self.universe_version_archetype_loader
    }

    pub fn universe_version_maps(&self) -> &Loader<(Uuid, i32, Uuid), UniverseVersionMap> {
        &self.universe_version_map_loader
    }
}

impl juniper::Context for Context {}
