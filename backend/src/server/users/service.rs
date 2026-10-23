use crate::server::shared::events::traits::{EntityEventFlags, EntityScope, Event};
use crate::server::{
    auth::middleware::auth::AuthenticatedEntity,
    shared::{
        entities::{ChangeTriggersTopologyStaleness, EntityDiscriminants},
        events::{
            bus::EventBus,
            types::{EntityOperation, UpdateReason},
        },
        services::traits::{CrudService, EventBusService},
        storage::{
            child::{GenericChildStorage, ParentRef},
            filter::StorableFilter,
            generic::GenericPostgresStorage,
            traits::{Entity, Storable, Storage},
        },
    },
    tags::entity_tags::EntityTagService,
    users::r#impl::{
        base::User,
        email_settings::EmailSettings,
        permissions::UserOrgPermissions,
        site_access::{UserSiteAccess, UserSiteAccessBase},
    },
};
use anyhow::Error;
use anyhow::Result;
use async_trait::async_trait;
use chrono::{NaiveDate, Utc};
use email_address::EmailAddress;
use std::sync::Arc;
use uuid::Uuid;

pub struct UserService {
    user_storage: Arc<GenericPostgresStorage<User>>,
    site_access_storage: Arc<GenericChildStorage<UserSiteAccess>>,
    event_bus: Arc<EventBus>,
}

impl EventBusService<User> for UserService {
    fn event_bus(&self) -> &Arc<EventBus> {
        &self.event_bus
    }

    fn get_site_id(&self, _entity: &User) -> Option<Uuid> {
        None
    }
    fn get_organization_id(&self, entity: &User) -> Option<Uuid> {
        Some(entity.base.organization_id)
    }
}

#[async_trait]
impl CrudService<User> for UserService {
    fn storage(&self) -> &Arc<GenericPostgresStorage<User>> {
        &self.user_storage
    }

    fn entity_tag_service(&self) -> Option<&Arc<EntityTagService>> {
        None // Users are not taggable entities
    }

    /// Create a new user
    async fn create(&self, user: User, authentication: AuthenticatedEntity) -> Result<User, Error> {
        let email_taken = self
            .user_storage
            .exists(StorableFilter::<User>::new_from_email(&user.base.email))
            .await?;
        if email_taken {
            return Err(anyhow::anyhow!(
                "User with email {} already exists",
                user.base.email
            ));
        }

        // Capture site_ids before creating the user (since they're stored in junction table)
        let site_ids = user.base.site_ids.clone();

        let user = if user.id() == Uuid::nil() {
            User::new(user.base)
        } else {
            user
        };
        let created = self.user_storage.create(&user).await?;

        // Persist site_ids to the junction table
        if !site_ids.is_empty() {
            self.set_site_ids(&created.id, &site_ids).await?;
        }

        let trigger_stale = created.triggers_staleness(None);

        if let Some(scope) = EntityScope::from_ids(
            created.id,
            created.clone().into(),
            self.get_site_id(&created),
            self.get_organization_id(&created),
        ) {
            self.event_bus()
                .publish(
                    Event::new(scope, EntityOperation::Created, authentication).with_flags(
                        EntityEventFlags {
                            trigger_stale,
                            ..Default::default()
                        },
                    ),
                )
                .await;
        }

        Ok(created)
    }
}

impl UserService {
    pub fn new(
        user_storage: Arc<GenericPostgresStorage<User>>,
        site_access_storage: Arc<GenericChildStorage<UserSiteAccess>>,
        event_bus: Arc<EventBus>,
    ) -> Self {
        Self {
            user_storage,
            site_access_storage,
            event_bus,
        }
    }

    pub async fn get_user_by_oidc(&self, oidc_subject: &str) -> Result<Option<User>> {
        let oidc_filter = StorableFilter::<User>::new_from_oidc_subject(oidc_subject.to_string());
        self.user_storage
            .get_unique(oidc_filter)
            .await?
            .at_most_one()
    }

    /// Look up a user by their email address. Used by the email send pipeline
    /// to resolve the recipient's pause preferences.
    pub async fn get_by_email(&self, email: &EmailAddress) -> Result<Option<User>> {
        self.user_storage
            .get_unique(StorableFilter::<User>::new_from_email(email))
            .await?
            .at_most_one()
    }

    /// Look up the user whose one-click unsubscribe link carries `token`.
    pub async fn get_by_unsubscribe_token(&self, token: &Uuid) -> Result<Option<User>> {
        self.user_storage
            .get_unique(StorableFilter::<User>::new_from_email_unsubscribe_token(
                token,
            ))
            .await?
            .at_most_one()
    }

    /// The token for `user`'s one-click unsubscribe links. Minted and stored
    /// the first time a user needs one, then reused, so every link the user
    /// has received keeps working.
    pub async fn ensure_unsubscribe_token(&self, user: &User) -> Result<Uuid> {
        if let Some(token) = user.base.email_unsubscribe_token {
            return Ok(token);
        }
        let token = Uuid::new_v4();
        let mut updated = user.clone();
        updated.base.email_unsubscribe_token = Some(token);
        self.update(
            &mut updated,
            AuthenticatedEntity::System,
            UpdateReason::UserUnsubscribeTokenIssued,
        )
        .await?;
        Ok(token)
    }

    /// Apply `change` to `user`'s email settings and store the result when it
    /// changed anything. Returns whether it did.
    pub async fn update_email_settings(
        &self,
        mut user: User,
        reason: UpdateReason,
        change: impl FnOnce(&mut EmailSettings) -> bool + Send,
    ) -> Result<bool> {
        if !change(&mut user.base.email_settings) {
            return Ok(false);
        }
        self.update(&mut user, AuthenticatedEntity::System, reason)
            .await?;
        Ok(true)
    }

    /// Whether this server has any user at all. A server without one is a
    /// fresh install.
    pub async fn any_exist(&self) -> Result<bool> {
        self.user_storage
            .exists(StorableFilter::<User>::new_for_server_wide_job())
            .await
    }

    /// Record that `user` has seen the release notes up to `version`.
    pub async fn mark_release_notes_seen(&self, mut user: User, version: &str) -> Result<()> {
        if user.base.release_notes_seen_version.as_deref() == Some(version) {
            return Ok(());
        }
        user.base.release_notes_seen_version = Some(version.to_string());
        self.update(
            &mut user,
            AuthenticatedEntity::System,
            UpdateReason::UserReleaseNotesSeen,
        )
        .await?;
        Ok(())
    }

    /// Record that `user` accepted the terms now.
    pub async fn accept_terms(&self, mut user: User) -> Result<User> {
        user.base.terms_accepted_at = Some(Utc::now());
        let actor = AuthenticatedEntity::from(user.clone());
        self.update(&mut user, actor, UpdateReason::UserTermsAccepted)
            .await
    }

    /// Record that the user `user_id` has been emailed notice of the terms
    /// update effective on `version`. Reads the user afresh, so a long sweep
    /// never writes back a copy it fetched before the user changed it.
    pub async fn mark_terms_notified(&self, user_id: &Uuid, version: NaiveDate) -> Result<()> {
        let Some(mut user) = self.get_by_id(user_id).await? else {
            return Ok(());
        };
        if user.base.terms_notice_version >= Some(version) {
            return Ok(());
        }
        user.base.terms_notice_version = Some(version);
        self.update(
            &mut user,
            AuthenticatedEntity::System,
            UpdateReason::UserTermsNotified,
        )
        .await?;
        Ok(())
    }

    pub async fn get_organization_owners(&self, organization_id: &Uuid) -> Result<Vec<User>> {
        let filter = StorableFilter::<User>::new_from_org_id(organization_id)
            .user_permissions(&UserOrgPermissions::Owner);

        self.user_storage.get_all(filter).await
    }

    /// Returns all users with access to `site_id` within `organization_id`.
    ///
    /// Access is the union of:
    /// - Users with an explicit row in the `user_site_access` junction.
    /// - Org Owners and Admins, who have implicit access to every site in
    ///   their organization regardless of the junction table.
    ///
    /// Deduplicates users that fall into both sets. The org-id parameter is
    /// required so the implicit set is scoped — junction rows alone don't
    /// carry the org context the way `User.organization_id` does.
    pub async fn get_users_with_site_access(
        &self,
        site_id: &Uuid,
        organization_id: &Uuid,
    ) -> Result<Vec<User>> {
        let explicit_user_ids: Vec<Uuid> = self
            .site_access_storage
            .inner()
            .get_all(StorableFilter::<UserSiteAccess>::new_from_site_ids(&[
                *site_id,
            ]))
            .await?
            .iter()
            .map(|a| a.user_id())
            .collect();

        let explicit = if explicit_user_ids.is_empty() {
            Vec::new()
        } else {
            // The users table's PK is `id`; the `user_id` column lives on the
            // user_site_access junction. Filter on entity ids.
            let filter = StorableFilter::<User>::new_from_entity_ids(&explicit_user_ids);
            self.user_storage.get_all(filter).await?
        };

        let implicit_filter = StorableFilter::<User>::new_from_org_id(organization_id)
            .user_permissions_in(&[UserOrgPermissions::Owner, UserOrgPermissions::Admin]);
        let implicit = self.user_storage.get_all(implicit_filter).await?;

        let mut seen: std::collections::HashSet<Uuid> =
            std::collections::HashSet::with_capacity(explicit.len() + implicit.len());
        let mut out: Vec<User> = Vec::with_capacity(explicit.len() + implicit.len());
        for user in explicit.into_iter().chain(implicit.into_iter()) {
            if seen.insert(user.id) {
                out.push(user);
            }
        }
        Ok(out)
    }

    /// Get site_ids for a user from the user_site_access junction table
    pub async fn get_site_ids(&self, user_id: &Uuid) -> Result<Vec<Uuid>> {
        let access = self.site_access_storage.get_for_parent(user_id).await?;
        Ok(access.iter().map(|a| a.site_id()).collect())
    }

    /// Set site_ids for a user - replaces all existing entries in user_site_access
    pub async fn set_site_ids(&self, user_id: &Uuid, site_ids: &[Uuid]) -> Result<()> {
        let rows: Vec<UserSiteAccess> = site_ids
            .iter()
            .map(|site_id| UserSiteAccess::new(UserSiteAccessBase::new(*user_id, *site_id)))
            .collect();
        self.site_access_storage
            .save_for_parent(&ParentRef::new(EntityDiscriminants::User, *user_id), &rows)
            .await?;
        Ok(())
    }

    /// Add a site_id to a user's access
    pub async fn add_site_access(&self, user_id: &Uuid, site_id: &Uuid) -> Result<()> {
        let access = UserSiteAccess::new(UserSiteAccessBase::new(*user_id, *site_id));
        // The storage will handle the unique constraint violation gracefully
        let _ = self.site_access_storage.inner().create(&access).await;
        Ok(())
    }

    /// Remove a site_id from a user's access
    pub async fn remove_site_access(&self, user_id: &Uuid, site_id: &Uuid) -> Result<()> {
        let filter = StorableFilter::<UserSiteAccess>::new_from_user_id(user_id)
            .uuid_column("site_id", site_id);
        self.site_access_storage
            .inner()
            .delete_by_filter(filter)
            .await?;
        Ok(())
    }

    /// Hydrate site_ids for a single user
    pub async fn hydrate_site_ids(&self, user: &mut User) -> Result<()> {
        user.base.site_ids = self.get_site_ids(&user.id).await?;
        Ok(())
    }

    /// Hydrate site_ids for multiple users (batch operation)
    pub async fn hydrate_site_ids_batch(&self, users: &mut [User]) -> Result<()> {
        if users.is_empty() {
            return Ok(());
        }

        let user_ids: Vec<Uuid> = users.iter().map(|u| u.id).collect();
        let mut access_map = self.site_access_storage.get_for_parents(&user_ids).await?;

        for user in users.iter_mut() {
            user.base.site_ids = access_map
                .remove(&user.id)
                .unwrap_or_default()
                .iter()
                .map(|a| a.site_id())
                .collect();
        }

        Ok(())
    }
}
