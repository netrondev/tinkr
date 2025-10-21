#[cfg(feature = "ssr")]
use crate::AppError;
#[cfg(feature = "ssr")]
use crate::db_init;

use partial_struct::Partial;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use surrealdb::{Datetime, RecordId};

#[cfg(feature = "ssr")]
use crate::user::AdapterUser;

#[cfg(not(feature = "ssr"))]
use crate::{Datetime, RecordId};

// Team model
#[derive(Debug, Clone, Serialize, Deserialize, Partial)]
#[partial(
    "CreateTeam",
    derive(Serialize, Deserialize, Clone),
    omit(id, organization_id, created_by_user_id, created_at, updated_at)
)]
#[partial(
    "UpdateTeam",
    derive(Debug, Serialize, Deserialize, Clone),
    omit(id, organization_id, created_by_user_id, created_at)
)]
pub struct Team {
    pub id: RecordId,
    pub name: String,
    pub description: Option<String>,
    pub organization_id: RecordId,
    pub created_by_user_id: RecordId,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

// Team member model
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TeamRole {
    Member,
    Admin,
    Owner,
}

#[derive(Debug, Clone, Serialize, Deserialize, Partial)]
#[partial(
    "CreateTeamMember",
    derive(Serialize, Deserialize, Clone),
    omit(id, joined_at)
)]
#[partial(
    "UpdateTeamMember",
    derive(Debug, Serialize, Deserialize, Clone),
    omit(id, team_id, user_id, joined_at)
)]
pub struct TeamMember {
    pub id: RecordId,
    pub team_id: RecordId,
    pub user_id: RecordId,
    pub role: TeamRole,
    pub joined_at: Datetime,
}

// Team invitation model
#[derive(Debug, Clone, Serialize, Deserialize, Partial)]
#[partial(
    "CreateTeamInvitation",
    derive(Serialize, Deserialize, Clone),
    omit(id, created_at, accepted_at)
)]
pub struct TeamInvitation {
    pub id: RecordId,
    pub team_id: RecordId,
    pub email: String,
    pub role: TeamRole,
    pub invited_by_user_id: RecordId,
    pub token: String,
    pub created_at: Datetime,
    pub expires_at: Datetime,
    pub accepted_at: Option<Datetime>,
}

// Implementation for Team
#[cfg(feature = "ssr")]
impl Team {
    pub async fn create(
        data: CreateTeam,
        organization_id: RecordId,
        created_by_user_id: RecordId,
    ) -> Result<Team, AppError> {
        let db = db_init().await?;

        #[derive(Serialize)]
        struct CreateTeamData {
            name: String,
            description: Option<String>,
            organization_id: RecordId,
            created_by_user_id: RecordId,
            created_at: Datetime,
            updated_at: Datetime,
        }

        let now = Datetime::default();
        let team_data = CreateTeamData {
            name: data.name,
            description: data.description,
            organization_id,
            created_by_user_id: created_by_user_id.clone(),
            created_at: now.clone(),
            updated_at: now,
        };

        let created: Team = db
            .create("team")
            .content(team_data)
            .await?
            .ok_or(AppError::DatabaseError("Failed to create team".to_string()))?;

        // Add creator as owner
        let member_data = CreateTeamMember {
            team_id: created.id.clone(),
            user_id: created_by_user_id,
            role: TeamRole::Owner,
        };

        let _: TeamMember =
            db.create("team_member")
                .content(member_data)
                .await?
                .ok_or(AppError::DatabaseError(
                    "Failed to create initial team member".to_string(),
                ))?;

        Ok(created)
    }

    pub async fn get_by_id(id: RecordId) -> Result<Team, AppError> {
        let db = db_init().await?;
        let team: Option<Team> = db.select(id).await?;
        team.ok_or_else(|| AppError::NotFound("Team not found".into()))
    }

    pub async fn get_organization_teams(organization_id: RecordId) -> Result<Vec<Team>, AppError> {
        let db = db_init().await?;

        let teams: Vec<Team> = db
            .query("SELECT * FROM team WHERE organization_id = $org_id")
            .bind(("org_id", organization_id))
            .await?
            .take(0)?;

        Ok(teams)
    }

    pub async fn get_user_teams(user_id: RecordId) -> Result<Vec<Team>, AppError> {
        let db = db_init().await?;

        let query = "
            SELECT team.* FROM team_member 
            WHERE user_id = $user_id
            FETCH team
        ";

        let mut result = db.query(query).bind(("user_id", user_id)).await?;

        let teams: Vec<Team> = result.take(0)?;
        Ok(teams)
    }

    pub async fn update(id: RecordId, data: UpdateTeam) -> Result<Team, AppError> {
        let db = db_init().await?;

        #[derive(Serialize)]
        struct UpdateTeamData {
            name: String,
            description: Option<String>,
            updated_at: Datetime,
        }

        let update_data = UpdateTeamData {
            name: data.name,
            description: data.description,
            updated_at: Datetime::default(),
        };

        let updated: Option<Team> = db.update(id).content(update_data).await?;
        updated.ok_or_else(|| AppError::NotFound("Team not found".into()))
    }

    pub async fn delete(id: RecordId) -> Result<(), AppError> {
        let db = db_init().await?;

        // Delete all team members
        let _: Vec<TeamMember> = db
            .query("DELETE team_member WHERE team_id = $team_id")
            .bind(("team_id", id.clone()))
            .await?
            .take(0)?;

        // Delete the team
        let _: Option<Team> = db.delete(id).await?;
        Ok(())
    }
}

// Implementation for TeamMember
#[cfg(feature = "ssr")]
impl TeamMember {
    pub async fn add_member(
        team_id: RecordId,
        user_id: RecordId,
        role: TeamRole,
    ) -> Result<TeamMember, AppError> {
        let db = db_init().await?;

        // Check if member already exists
        let existing: Vec<TeamMember> = db
            .query("SELECT * FROM team_member WHERE team_id = $team_id AND user_id = $user_id")
            .bind(("team_id", team_id.clone()))
            .bind(("user_id", user_id.clone()))
            .await?
            .take(0)?;

        if !existing.is_empty() {
            return Err(AppError::ErrorReason(
                "User is already a member of this team".into(),
            ));
        }

        #[derive(Serialize)]
        struct CreateMemberData {
            team_id: RecordId,
            user_id: RecordId,
            role: TeamRole,
            joined_at: Datetime,
        }

        let member_data = CreateMemberData {
            team_id,
            user_id,
            role,
            joined_at: Datetime::default(),
        };

        let created: TeamMember =
            db.create("team_member")
                .content(member_data)
                .await?
                .ok_or(AppError::DatabaseError(
                    "Failed to create team member".to_string(),
                ))?;
        Ok(created)
    }

    pub async fn update_role(
        team_id: RecordId,
        user_id: RecordId,
        role: TeamRole,
    ) -> Result<TeamMember, AppError> {
        let db = db_init().await?;

        let query =
            "UPDATE team_member SET role = $role WHERE team_id = $team_id AND user_id = $user_id";

        let mut result = db
            .query(query)
            .bind(("role", role))
            .bind(("team_id", team_id))
            .bind(("user_id", user_id))
            .await?;

        let updated: Option<TeamMember> = result.take(0)?;
        updated.ok_or_else(|| AppError::NotFound("Team member not found".into()))
    }

    pub async fn remove_member(team_id: RecordId, user_id: RecordId) -> Result<(), AppError> {
        let db = db_init().await?;

        let query = "DELETE team_member WHERE team_id = $team_id AND user_id = $user_id";

        let _: Vec<TeamMember> = db
            .query(query)
            .bind(("team_id", team_id))
            .bind(("user_id", user_id))
            .await?
            .take(0)?;

        Ok(())
    }

    pub async fn get_team_members(
        team_id: RecordId,
    ) -> Result<Vec<(TeamMember, AdapterUser)>, AppError> {
        let db = db_init().await?;

        let query = "
            SELECT *, user.* FROM team_member 
            WHERE team_id = $team_id
            FETCH user
        ";

        let mut result = db.query(query).bind(("team_id", team_id)).await?;

        let members: Vec<(TeamMember, AdapterUser)> = result.take(0)?;
        Ok(members)
    }
}

// Implementation for TeamInvitation
#[cfg(feature = "ssr")]
impl TeamInvitation {
    pub async fn create(
        team_id: RecordId,
        email: String,
        role: TeamRole,
        invited_by_user_id: RecordId,
    ) -> Result<TeamInvitation, AppError> {
        let db = db_init().await?;

        use rand::Rng;

        // Generate a random token using the same pattern as bot.rs
        let random_num: u64 = rand::rng().random();
        let token = format!("invite_{:x}", random_num); // This creates a hex-based token

        #[derive(Serialize)]
        struct CreateInvitationData {
            team_id: RecordId,
            email: String,
            role: TeamRole,
            invited_by_user_id: RecordId,
            token: String,
            created_at: Datetime,
            expires_at: Datetime,
            accepted_at: Option<Datetime>,
        }

        let now = Datetime::default();
        let expires_at = chrono::Utc::now() + chrono::Duration::days(7);

        let invitation_data = CreateInvitationData {
            team_id,
            email,
            role,
            invited_by_user_id,
            token,
            created_at: now,
            expires_at: Datetime::from(expires_at),
            accepted_at: None,
        };

        let created: TeamInvitation = db
            .create("team_invitation")
            .content(invitation_data)
            .await?
            .ok_or(AppError::DatabaseError(
                "Failed to create team invitation".to_string(),
            ))?;
        Ok(created)
    }

    pub async fn accept(token: String, user_id: RecordId) -> Result<TeamMember, AppError> {
        let db = db_init().await?;

        // Find the invitation
        let invitations: Vec<TeamInvitation> = db
            .query(
                "SELECT * FROM team_invitation WHERE token = $invite_token AND accepted_at = NONE",
            )
            .bind(("invite_token", token))
            .await?
            .take(0)?;

        let invitation = invitations
            .into_iter()
            .next()
            .ok_or_else(|| AppError::NotFound("Invalid or already used invitation".into()))?;

        // Check if expired
        let now = Datetime::from(chrono::Utc::now());
        if now > invitation.expires_at {
            return Err(AppError::ErrorReason("Invitation has expired".into()));
        }

        // Mark invitation as accepted
        let _: Option<TeamInvitation> = db
            .update(invitation.id.clone())
            .merge(serde_json::json!({
                "accepted_at": Datetime::default()
            }))
            .await?;

        // Add user to team
        TeamMember::add_member(invitation.team_id, user_id, invitation.role).await
    }
}
