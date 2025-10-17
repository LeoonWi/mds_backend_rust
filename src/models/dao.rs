use crate::models::dto;
use chrono::{DateTime, Utc};
use sqlx::{Decode, Encode, Type, encode::IsNull, error::BoxDynError, postgres::PgTypeInfo};

#[derive(Debug, sqlx::FromRow)]
pub struct Service {
    id: Option<i64>,
    name: String,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

impl Service {
    pub fn to_dto(from: Service) -> dto::Service {
        dto::Service {
            id: from.id,
            name: from.name,
            created_at: Some(from.created_at),
            updated_at: from.updated_at,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i16)]
pub enum Role {
    Employee = 0,
    Manager = 1,
    Superadmin = 2,
}

impl Role {
    pub fn to_dto(&self) -> String {
        match self {
            Role::Employee => String::from("Сотрудник"),
            Role::Manager => String::from("Менеджер"),
            Role::Superadmin => String::from("Суперадмин"),
        }
    }

    pub fn from(str: Option<String>) -> Result<Role, dto::Error> {
        match str.as_deref() {
            Some("Сотрудник") => Ok(Role::Employee),
            Some("Менеджер") => Ok(Role::Manager),
            Some("Суперадмин") => Ok(Role::Superadmin),
            Some(_) => Err(dto::Error::BadRequest(String::from("Unknown string role"))),
            None => Err(dto::Error::BadRequest(String::from("Role string is None"))),
        }
    }
}

impl Type<sqlx::Postgres> for Role {
    fn type_info() -> PgTypeInfo {
        <i16 as Type<sqlx::Postgres>>::type_info()
    }
}

impl<'q> Encode<'q, sqlx::Postgres> for Role {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<IsNull, BoxDynError> {
        <i16 as Encode<'q, sqlx::Postgres>>::encode(*self as i16, buf)
    }
}

impl<'r> Decode<'r, sqlx::Postgres> for Role {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        let id: i16 = <i16 as Decode<'r, sqlx::Postgres>>::decode(value)?;
        Ok(match id {
            0 => Role::Employee,
            1 => Role::Manager,
            2 => Role::Superadmin,
            _ => return Err("Invalid role value".into()),
        })
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct Employee {
    pub id: i64,
    pub name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub email: String,
    pub password: String,
    pub role: Role,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Employee {
    pub fn to_dto(from: Employee) -> dto::Employee {
        dto::Employee {
            id: Some(from.id),
            name: Some(from.name),
            last_name: Some(from.last_name),
            middle_name: from.middle_name,
            email: Some(from.email),
            password: Some(from.password),
            role: Some(from.role.to_dto()),
            services: None,
            // services: Some(
            //     from.services
            //         .into_iter()
            //         .map(|x| Service::to_dto(x))
            //         .collect(),
            // ),
            active: Some(from.active),
            created_at: Some(from.created_at),
            updated_at: from.updated_at,
        }
    }
}
