#[cfg(feature = "ssr")]
use crate::{user::AdapterUser, StorageAuthed};

#[cfg(feature = "ssr")]
use crate::AppError;
use partial_struct::Partial;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use crate::db_init;

#[cfg(feature = "ssr")]
use surrealdb::{Datetime, RecordId};

#[cfg(not(feature = "ssr"))]
use crate::{Datetime, RecordId};

#[cfg(feature = "ssr")]
use super::super::team::team::Team;

// Organization model
#[derive(Debug, Clone, Serialize, Deserialize, Partial)]
#[partial(
    "CreateOrganization",
    derive(Debug, Serialize, Deserialize, Clone),
    omit(id, created_by_user_id, created_at, updated_at)
)]
pub struct Organization {
    pub id: RecordId,
    pub name: String,
    pub description: Option<String>,
    pub logo_url: Option<String>,
    pub website: Option<String>,
    pub created_by_user_id: RecordId,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

#[cfg(feature = "ssr")]
impl StorageAuthed<CreateOrganization, Organization> for Organization {
    const TABLE_NAME: &str = "organization";
}

// ----------------------------------------------------------------------

// Organization member model
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrganizationRole {
    Member,
    Admin,
    Owner,
}

#[derive(Debug, Clone, Serialize, Deserialize, Partial)]
#[partial(
    "CreateOrganizationMember",
    derive(Serialize, Deserialize, Clone),
    omit(id, joined_at)
)]
#[partial(
    "UpdateOrganizationMember",
    derive(Debug, Serialize, Deserialize, Clone),
    omit(id, organization_id, user_id, joined_at)
)]
pub struct OrganizationMember {
    pub id: RecordId,
    pub organization_id: RecordId,
    pub user_id: RecordId,
    pub role: OrganizationRole,
    pub joined_at: Datetime,
}

#[cfg(feature = "ssr")]
impl StorageAuthed<CreateOrganizationMember, OrganizationMember> for OrganizationMember {
    const TABLE_NAME: &str = "organization_member";
}

// Implementation for Organization
#[cfg(feature = "ssr")]
impl Organization {
    pub async fn create(
        data: CreateOrganization,
        created_by_user_id: RecordId,
    ) -> Result<Organization, AppError> {
        let db = db_init().await?;

        #[derive(Serialize)]
        struct CreateOrgData {
            name: String,
            description: Option<String>,
            logo_url: Option<String>,
            website: Option<String>,
            created_by_user_id: RecordId,
            created_at: Datetime,
            updated_at: Datetime,
        }

        let now = Datetime::default();
        let org_data = CreateOrgData {
            name: data.name,
            description: data.description,
            logo_url: data.logo_url,
            website: data.website,
            created_by_user_id: created_by_user_id.clone(),
            created_at: now.clone(),
            updated_at: now,
        };

        let created: Organization =
            db.create("organization")
                .content(org_data)
                .await?
                .ok_or(AppError::DatabaseError(
                    "Failed to create organization".to_string(),
                ))?;

        // Add creator as owner
        #[derive(Serialize)]
        struct CreateOrgMemberData {
            organization_id: RecordId,
            user_id: RecordId,
            role: OrganizationRole,
            joined_at: Datetime,
        }

        let member_data = CreateOrgMemberData {
            organization_id: created.id.clone(),
            user_id: created_by_user_id,
            role: OrganizationRole::Owner,
            joined_at: Datetime::default(),
        };

        let _: OrganizationMember = db
            .create("organization_member")
            .content(member_data)
            .await?
            .ok_or(AppError::DatabaseError(
                "Failed to create organization member".to_string(),
            ))?;

        Ok(created)
    }

    pub async fn get_by_id(id: RecordId) -> Result<Organization, AppError> {
        let db = db_init().await?;
        let org: Option<Organization> = db.select(id).await?;
        org.ok_or_else(|| AppError::NotFound("Organization not found".into()))
    }

    pub async fn get_user_organizations(user_id: RecordId) -> Result<Vec<Organization>, AppError> {
        let db = db_init().await?;

        // First get the organization IDs for this user
        let query = "
            SELECT organization_id FROM organization_member
            WHERE user_id = $user_id
        ";

        let mut result = db.query(query).bind(("user_id", user_id)).await?;

        #[derive(Deserialize)]
        struct OrgMember {
            organization_id: RecordId,
        }

        let members: Vec<OrgMember> = result.take(0)?;

        // Then fetch the organizations
        let mut orgs = Vec::new();
        for member in members {
            if let Ok(Some(org)) = db
                .select::<Option<Organization>>(member.organization_id)
                .await
            {
                orgs.push(org);
            }
        }

        Ok(orgs)
    }

    pub async fn update(_id: RecordId, data: Organization) -> Result<Organization, AppError> {
        // let db = db_init().await?;

        // #[derive(Serialize)]
        // struct UpdateOrgData {
        //     name: String,
        //     description: Option<String>,
        //     logo_url: Option<String>,
        //     website: Option<String>,
        //     updated_at: Datetime,
        // }

        // let update_data = UpdateOrgData {
        //     name: data.name,
        //     description: data.description,
        //     logo_url: data.logo_url,
        //     website: data.website,
        //     updated_at: Datetime::default(),
        // };

        // let updated: Option<Organization> = db.update(id).content(update_data).await?;
        // updated.ok_or_else(|| AppError::NotFound("Organization not found".into()))

        let updated = data.update_self().await?;

        Ok(updated)
    }

    pub async fn delete(id: RecordId) -> Result<(), AppError> {
        let db = db_init().await?;

        // Delete all teams in the organization
        let _: Vec<Team> = db
            .query("DELETE team WHERE organization_id = $org_id")
            .bind(("org_id", id.clone()))
            .await?
            .take(0)?;

        // Delete all organization members
        let _: Vec<OrganizationMember> = db
            .query("DELETE organization_member WHERE organization_id = $org_id")
            .bind(("org_id", id.clone()))
            .await?
            .take(0)?;

        // Delete the organization
        let _: Option<Organization> = db.delete(id).await?;
        Ok(())
    }
}

// Implementation for OrganizationMember
#[cfg(feature = "ssr")]
impl OrganizationMember {
    pub async fn add_member(
        organization_id: RecordId,
        user_id: RecordId,
        role: OrganizationRole,
    ) -> Result<OrganizationMember, AppError> {
        let db = db_init().await?;

        // Check if member already exists
        let existing: Vec<OrganizationMember> = db.query("SELECT * FROM organization_member WHERE organization_id = $org_id AND user_id = $user_id")
            .bind(("org_id", organization_id.clone()))
            .bind(("user_id", user_id.clone()))
            .await?
            .take(0)?;

        if !existing.is_empty() {
            return Err(AppError::ErrorReason(
                "User is already a member of this organization".into(),
            ));
        }

        #[derive(Serialize)]
        struct CreateMemberData {
            organization_id: RecordId,
            user_id: RecordId,
            role: OrganizationRole,
            joined_at: Datetime,
        }

        let member_data = CreateMemberData {
            organization_id,
            user_id,
            role,
            joined_at: Datetime::default(),
        };

        let created: OrganizationMember = db
            .create("organization_member")
            .content(member_data)
            .await?
            .ok_or(AppError::DatabaseError(
                "Failed to create organization member".to_string(),
            ))?;
        Ok(created)
    }

    pub async fn update_role(
        organization_id: RecordId,
        user_id: RecordId,
        role: OrganizationRole,
    ) -> Result<OrganizationMember, AppError> {
        let db = db_init().await?;

        let query = "UPDATE organization_member SET role = $role WHERE organization_id = $org_id AND user_id = $user_id";

        let mut result = db
            .query(query)
            .bind(("role", role))
            .bind(("org_id", organization_id))
            .bind(("user_id", user_id))
            .await?;

        let updated: Option<OrganizationMember> = result.take(0)?;
        updated.ok_or_else(|| AppError::NotFound("Organization member not found".into()))
    }

    pub async fn remove_member(
        organization_id: RecordId,
        user_id: RecordId,
    ) -> Result<(), AppError> {
        let db = db_init().await?;

        let query =
            "DELETE organization_member WHERE organization_id = $org_id AND user_id = $user_id";

        let _: Vec<OrganizationMember> = db
            .query(query)
            .bind(("org_id", organization_id))
            .bind(("user_id", user_id))
            .await?
            .take(0)?;

        Ok(())
    }

    pub async fn get_organization_members(
        organization_id: RecordId,
    ) -> Result<Vec<(OrganizationMember, AdapterUser)>, AppError> {
        let db = db_init().await?;

        let query = "
            SELECT *, user.* FROM organization_member
            WHERE organization_id = $org_id
            FETCH user
        ";

        let mut result = db.query(query).bind(("org_id", organization_id)).await?;

        let members: Vec<(OrganizationMember, AdapterUser)> = result.take(0)?;
        Ok(members)
    }
}
