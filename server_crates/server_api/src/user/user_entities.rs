use sentc_crypto_common::{EncryptionKeyPairId, SignKeyPairId, UserId};
use serde::{Deserialize, Serialize};

use crate::take_or_err;

//generated with browser console: btoa(String.fromCharCode.apply(null, window.crypto.getRandomValues(new Uint8Array(128/8))));
//the value with the used alg
pub static SERVER_RANDOM_VALUE: (&'static str, &'static str) = ("zx4AKPCMHkeZnh21ciQ62w==", sentc_crypto::util_pub::ARGON_2_OUTPUT);

//__________________________________________________________________________________________________
//Jwt

pub struct JwtSignKey(pub String);

#[cfg(feature = "mysql")]
impl mysql_async::prelude::FromRow for JwtSignKey
{
	fn from_row_opt(mut row: mysql_async::Row) -> Result<Self, mysql_async::FromRowError>
	where
		Self: Sized,
	{
		Ok(JwtSignKey(take_or_err!(row, 0, String)))
	}
}

#[cfg(feature = "sqlite")]
impl crate::core::db::FromSqliteRow for JwtSignKey
{
	fn from_row_opt(row: &rusqlite::Row) -> Result<Self, crate::core::db::FormSqliteRowError>
	where
		Self: Sized,
	{
		Ok(JwtSignKey(take_or_err!(row, 0)))
	}
}

pub struct JwtVerifyKey(pub String);

#[cfg(feature = "mysql")]
impl mysql_async::prelude::FromRow for JwtVerifyKey
{
	fn from_row_opt(mut row: mysql_async::Row) -> Result<Self, mysql_async::FromRowError>
	where
		Self: Sized,
	{
		Ok(JwtVerifyKey(take_or_err!(row, 0, String)))
	}
}

#[cfg(feature = "sqlite")]
impl crate::core::db::FromSqliteRow for JwtVerifyKey
{
	fn from_row_opt(row: &rusqlite::Row) -> Result<Self, crate::core::db::FormSqliteRowError>
	where
		Self: Sized,
	{
		Ok(JwtVerifyKey(take_or_err!(row, 0)))
	}
}

#[derive(Serialize, Deserialize)]
pub struct UserJwtEntity
{
	pub id: UserId,
	pub identifier: String,
	//aud if it is an app user or an customer
	pub aud: String,
	pub sub: String, //the app id
}

//__________________________________________________________________________________________________
//User info

#[derive(Serialize, Deserialize)]
pub struct UserEntity
{
	id: String,
	name: String,
	time: u128,
}

#[cfg(feature = "mysql")]
impl mysql_async::prelude::FromRow for UserEntity
{
	fn from_row_opt(mut row: mysql_async::Row) -> Result<Self, mysql_async::FromRowError>
	where
		Self: Sized,
	{
		Ok(UserEntity {
			id: take_or_err!(row, 0, String),
			name: take_or_err!(row, 1, String),
			time: take_or_err!(row, 2, u128),
		})
	}
}

#[cfg(feature = "sqlite")]
impl crate::core::db::FromSqliteRow for UserEntity
{
	fn from_row_opt(row: &rusqlite::Row) -> Result<Self, crate::core::db::FormSqliteRowError>
	where
		Self: Sized,
	{
		//time needs to parse from string to the value
		let time: String = take_or_err!(row, 2);
		let time: u128 = time.parse().map_err(|e| {
			crate::core::db::FormSqliteRowError {
				msg: format!("err in db fetch: {:?}", e),
			}
		})?;

		Ok(UserEntity {
			id: take_or_err!(row, 0),
			name: take_or_err!(row, 1),
			time: time,
		})
	}
}

//__________________________________________________________________________________________________
//User exists

#[derive(Serialize, Deserialize)]
pub struct UserExistsEntity(pub i64); //i64 for sqlite

#[cfg(feature = "mysql")]
impl mysql_async::prelude::FromRow for UserExistsEntity
{
	fn from_row_opt(mut row: mysql_async::Row) -> Result<Self, mysql_async::FromRowError>
	where
		Self: Sized,
	{
		Ok(UserExistsEntity(take_or_err!(row, 0, i64)))
	}
}

#[cfg(feature = "sqlite")]
impl crate::core::db::FromSqliteRow for UserExistsEntity
{
	fn from_row_opt(row: &rusqlite::Row) -> Result<Self, crate::core::db::FormSqliteRowError>
	where
		Self: Sized,
	{
		Ok(UserExistsEntity(take_or_err!(row, 0)))
	}
}

//__________________________________________________________________________________________________
//User login data

#[derive(Serialize, Deserialize)]
pub struct UserLoginDataEntity
{
	pub client_random_value: String,
	pub hashed_authentication_key: String,
	pub derived_alg: String,
}

#[cfg(feature = "mysql")]
impl mysql_async::prelude::FromRow for UserLoginDataEntity
{
	fn from_row_opt(mut row: mysql_async::Row) -> Result<Self, mysql_async::FromRowError>
	where
		Self: Sized,
	{
		Ok(UserLoginDataEntity {
			client_random_value: take_or_err!(row, 0, String),
			hashed_authentication_key: take_or_err!(row, 1, String),
			derived_alg: take_or_err!(row, 2, String),
		})
	}
}

#[cfg(feature = "sqlite")]
impl crate::core::db::FromSqliteRow for UserLoginDataEntity
{
	fn from_row_opt(row: &rusqlite::Row) -> Result<Self, crate::core::db::FormSqliteRowError>
	where
		Self: Sized,
	{
		Ok(UserLoginDataEntity {
			client_random_value: take_or_err!(row, 0),
			hashed_authentication_key: take_or_err!(row, 1),
			derived_alg: take_or_err!(row, 2),
		})
	}
}

//__________________________________________________________________________________________________
//User done login data

pub struct DoneLoginServerKeysOutputEntity
{
	pub encrypted_master_key: String,
	pub encrypted_private_key: String,
	pub public_key_string: String,
	pub keypair_encrypt_alg: String,
	pub encrypted_sign_key: String,
	pub verify_key_string: String,
	pub keypair_sign_alg: String,
	pub keypair_encrypt_id: EncryptionKeyPairId,
	pub keypair_sign_id: SignKeyPairId,
	pub user_id: UserId,
}

#[cfg(feature = "mysql")]
impl mysql_async::prelude::FromRow for DoneLoginServerKeysOutputEntity
{
	fn from_row_opt(mut row: mysql_async::Row) -> Result<Self, mysql_async::FromRowError>
	where
		Self: Sized,
	{
		let k_id: String = take_or_err!(row, 7, String);
		let keypair_encrypt_id = k_id.to_string();
		let keypair_sign_id = k_id.to_string();

		Ok(Self {
			encrypted_master_key: take_or_err!(row, 0, String),
			encrypted_private_key: take_or_err!(row, 1, String),
			public_key_string: take_or_err!(row, 2, String),
			keypair_encrypt_alg: take_or_err!(row, 3, String),
			encrypted_sign_key: take_or_err!(row, 4, String),
			verify_key_string: take_or_err!(row, 5, String),
			keypair_sign_alg: take_or_err!(row, 6, String),
			keypair_encrypt_id,
			keypair_sign_id,
			user_id: take_or_err!(row, 8, String),
		})
	}
}

#[cfg(feature = "sqlite")]
impl crate::core::db::FromSqliteRow for DoneLoginServerKeysOutputEntity
{
	fn from_row_opt(row: &rusqlite::Row) -> Result<Self, crate::core::db::FormSqliteRowError>
	where
		Self: Sized,
	{
		let k_id: String = take_or_err!(row, 7);
		let keypair_encrypt_id = k_id.to_string();
		let keypair_sign_id = k_id.to_string();

		Ok(Self {
			encrypted_master_key: take_or_err!(row, 0),
			encrypted_private_key: take_or_err!(row, 1),
			public_key_string: take_or_err!(row, 2),
			keypair_encrypt_alg: take_or_err!(row, 3),
			encrypted_sign_key: take_or_err!(row, 4),
			verify_key_string: take_or_err!(row, 5),
			keypair_sign_alg: take_or_err!(row, 6),
			keypair_encrypt_id,
			keypair_sign_id,
			user_id: take_or_err!(row, 8),
		})
	}
}

//__________________________________________________________________________________________________
