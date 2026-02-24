use sea_orm::ActiveValue::{NotSet, Set};
use crate::dto::{UserRequestDto, UserUpdateDto, UserResponseDto, UserType, UserStatus};
use crate::entity::user;


impl UserRequestDto {
    pub fn into_create_model(self, operator_id: i32, hashed_password: String) -> user::ActiveModel {
        let now = chrono::Utc::now();

        user::ActiveModel {
            id: NotSet,
            username: Set(self.username),
            password: Set(hashed_password),
            kind: Set(user_type_dto_to_entity(self.kind)),
            status: Set(user_status_dto_to_entity(self.status)),
            is_deleted: Set(false),
            created_at: Set(now),
            updated_at: NotSet,
            created_by: Set(operator_id),
            updated_by: NotSet,
        }
    }
}


impl UserUpdateDto {
    pub fn into_update_model(
        &self,
        mut model: user::ActiveModel,
        operator_id: i32,
        hashed_password: Option<String>,
    ) -> user::ActiveModel {
        if let Some(password) = hashed_password {
            model.password = Set(password);
        }

        if let Some(kind) = self.kind {
            model.kind = Set(user_type_dto_to_entity(kind));
        }

        if let Some(status) = self.status {
            model.status = Set(user_status_dto_to_entity(status));
        }

        model.updated_at = Set(Some(chrono::Utc::now()));
        model.updated_by = Set(Some(operator_id));

        model
    }
}


impl From<user::Model> for UserResponseDto {
    fn from(entity: user::Model) -> Self {
        Self {
            id: entity.id,
            username: entity.username,
            kind: user_type_entity_to_dto(entity.kind),
            status: user_status_entity_to_dto(entity.status),
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}


pub fn user_type_entity_to_dto(entity_kind: user::UserType) -> UserType {
    match entity_kind {
        user::UserType::Normal => UserType::Normal,
        user::UserType::Admin => UserType::Admin,
    }
}


pub fn user_type_dto_to_entity(dto_kind: UserType) -> user::UserType {
    match dto_kind {
        UserType::Normal => user::UserType::Normal,
        UserType::Admin => user::UserType::Admin,
    }
}


pub fn user_status_entity_to_dto(entity_status: user::UserStatus) -> UserStatus {
    match entity_status {
        user::UserStatus::Enabled => UserStatus::Enabled,
        user::UserStatus::Disabled => UserStatus::Disabled,
    }
}


pub fn user_status_dto_to_entity(dto_status: UserStatus) -> user::UserStatus {
    match dto_status {
        UserStatus::Enabled => user::UserStatus::Enabled,
        UserStatus::Disabled => user::UserStatus::Disabled,
    }
}
