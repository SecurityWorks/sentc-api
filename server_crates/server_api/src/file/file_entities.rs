use sentc_crypto_common::file::BelongsToType;
use sentc_crypto_common::{FileId, PartId, SymKeyId, UserId};
use serde::Serialize;
use server_core::take_or_err;

pub struct FileSessionCheck
{
	pub file_id: FileId,
	pub created_at: u128,
	pub max_chunk_size: usize,
}

#[cfg(feature = "mysql")]
impl mysql_async::prelude::FromRow for FileSessionCheck
{
	fn from_row_opt(mut row: mysql_async::Row) -> Result<Self, mysql_async::FromRowError>
	where
		Self: Sized,
	{
		Ok(Self {
			file_id: take_or_err!(row, 0, String),
			created_at: take_or_err!(row, 1, u128),
			max_chunk_size: take_or_err!(row, 2, usize),
		})
	}
}

#[cfg(feature = "sqlite")]
impl server_core::db::FromSqliteRow for FileSessionCheck
{
	fn from_row_opt(row: &rusqlite::Row) -> Result<Self, server_core::db::FormSqliteRowError>
	where
		Self: Sized,
	{
		let created_at: String = take_or_err!(row, 1);
		let created_at: u128 = created_at.parse().map_err(|e| {
			server_core::db::FormSqliteRowError {
				msg: format!("err in db fetch: {:?}", e),
			}
		})?;

		let max_chunk_size: String = take_or_err!(row, 2);
		let max_chunk_size: usize = max_chunk_size.parse().map_err(|e| {
			server_core::db::FormSqliteRowError {
				msg: format!("err in db fetch: {:?}", e),
			}
		})?;

		Ok(Self {
			file_id: take_or_err!(row, 0),
			created_at,
			max_chunk_size,
		})
	}
}

//__________________________________________________________________________________________________

#[derive(Serialize)]
pub struct FileMetaData
{
	pub file_id: FileId,
	pub owner: UserId,
	pub belongs_to: Option<String>,
	pub belongs_to_type: BelongsToType,
	pub key_id: SymKeyId,
	pub time: u128,
	pub part_list: Vec<FilePartListItem>,
}

impl Into<sentc_crypto_common::file::FileData> for FileMetaData
{
	fn into(self) -> sentc_crypto_common::file::FileData
	{
		let mut part_list: Vec<sentc_crypto_common::file::FilePartListItem> = Vec::with_capacity(self.part_list.len());

		for part in self.part_list {
			part_list.push(part.into());
		}

		sentc_crypto_common::file::FileData {
			file_id: self.file_id,
			owner: self.owner,
			belongs_to: self.belongs_to,
			belongs_to_type: self.belongs_to_type,
			key_id: self.key_id,
			part_list,
		}
	}
}

#[cfg(feature = "mysql")]
impl mysql_async::prelude::FromRow for FileMetaData
{
	fn from_row_opt(mut row: mysql_async::Row) -> Result<Self, mysql_async::FromRowError>
	where
		Self: Sized,
	{
		let belongs_to = match row.take_opt::<Option<String>, _>(2) {
			Some(value) => {
				match value {
					Ok(ir) => ir,
					Err(mysql_async::FromValueError(_value)) => {
						return Err(mysql_async::FromRowError(row));
					},
				}
			},
			None => return Err(mysql_async::FromRowError(row)),
		};

		let belongs_to_type = take_or_err!(row, 3, i32);
		let belongs_to_type = match belongs_to_type {
			0 => BelongsToType::None,
			1 => BelongsToType::Group,
			2 => BelongsToType::User,
			_ => BelongsToType::None,
		};

		Ok(Self {
			file_id: take_or_err!(row, 0, String),
			owner: take_or_err!(row, 1, String),
			belongs_to,
			belongs_to_type,
			key_id: take_or_err!(row, 4, String),
			time: take_or_err!(row, 5, u128),
			part_list: Vec::new(),
		})
	}
}

#[cfg(feature = "sqlite")]
impl server_core::db::FromSqliteRow for FileMetaData
{
	fn from_row_opt(row: &rusqlite::Row) -> Result<Self, server_core::db::FormSqliteRowError>
	where
		Self: Sized,
	{
		let time: String = take_or_err!(row, 5);
		let time: u128 = time.parse().map_err(|e| {
			server_core::db::FormSqliteRowError {
				msg: format!("err in db fetch: {:?}", e),
			}
		})?;

		let belongs_to_type: i32 = take_or_err!(row, 3);
		let belongs_to_type = match belongs_to_type {
			0 => BelongsToType::None,
			1 => BelongsToType::Group,
			2 => BelongsToType::User,
			_ => BelongsToType::None,
		};

		Ok(Self {
			file_id: take_or_err!(row, 0),
			owner: take_or_err!(row, 1),
			belongs_to: take_or_err!(row, 2),
			belongs_to_type,
			key_id: take_or_err!(row, 4),
			time,
			part_list: Vec::new(),
		})
	}
}

//__________________________________________________________________________________________________

#[derive(Serialize)]
pub struct FilePartListItem(pub PartId);

impl Into<sentc_crypto_common::file::FilePartListItem> for FilePartListItem
{
	fn into(self) -> sentc_crypto_common::file::FilePartListItem
	{
		sentc_crypto_common::file::FilePartListItem(self.0)
	}
}

#[cfg(feature = "mysql")]
impl mysql_async::prelude::FromRow for FilePartListItem
{
	fn from_row_opt(mut row: mysql_async::Row) -> Result<Self, mysql_async::FromRowError>
	where
		Self: Sized,
	{
		Ok(Self(take_or_err!(row, 0, String)))
	}
}

#[cfg(feature = "sqlite")]
impl server_core::db::FromSqliteRow for FilePartListItem
{
	fn from_row_opt(row: &rusqlite::Row) -> Result<Self, server_core::db::FormSqliteRowError>
	where
		Self: Sized,
	{
		Ok(Self(take_or_err!(row, 0)))
	}
}

//__________________________________________________________________________________________________
