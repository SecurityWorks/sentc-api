use sentc_crypto_common::{AppId, CustomerId, JwtKeyId, UserId};
use uuid::Uuid;

use crate::core::api_res::{ApiErrorCodes, HttpErr};
use crate::core::db::{exec, exec_transaction, query, query_first, TransactionData};
use crate::core::get_time;
use crate::customer_app::app_entities::{AppData, AppDataGeneral, AppExistsEntity, AppJwt, AppJwtData, AuthWithToken};
use crate::set_params;

/**
# Internal app data

cached in the app token middleware
*/
pub(crate) async fn get_app_data(hashed_token: &str) -> Result<AppData, HttpErr>
{
	//language=SQL
	let sql = r"
SELECT id as app_id, customer_id, hashed_secret_token, hashed_public_token, hash_alg 
FROM app 
WHERE hashed_public_token = ? OR hashed_secret_token = ? LIMIT 1";

	let app_data: Option<AppDataGeneral> = query_first(
		sql.to_string(),
		set_params!(hashed_token.to_string(), hashed_token.to_string()),
	)
	.await?;

	let app_data = match app_data {
		Some(d) => d,
		None => {
			return Err(HttpErr::new(
				401,
				ApiErrorCodes::AppTokenNotFound,
				"App token not found".to_string(),
				None,
			))
		},
	};

	//language=SQL
	let sql = "SELECT id, alg, time FROM app_jwt_keys WHERE app_id = ? ORDER BY time DESC LIMIT 10";

	let jwt_data: Vec<AppJwt> = query(sql.to_string(), set_params!(app_data.app_id.to_string())).await?;

	let auth_with_token = if hashed_token == app_data.hashed_public_token {
		AuthWithToken::Public
	} else if hashed_token == app_data.hashed_secret_token {
		AuthWithToken::Secret
	} else {
		return Err(HttpErr::new(
			401,
			ApiErrorCodes::AppTokenNotFound,
			"App token not found".to_string(),
			None,
		));
	};

	Ok(AppData {
		app_data,
		jwt_data,
		auth_with_token,
	})
}

/**
Get general app data like internal get app data

but this time with check on app id und customer id

only used internally
*/
pub(super) async fn get_app_general_data(customer_id: CustomerId, app_id: AppId) -> Result<AppDataGeneral, HttpErr>
{
	//language=SQL
	let sql = r"
SELECT id as app_id, customer_id, hashed_secret_token, hashed_public_token, hash_alg 
FROM app 
WHERE customer_id = ? AND id = ? LIMIT 1";

	let app_data: Option<AppDataGeneral> = query_first(sql.to_string(), set_params!(customer_id, app_id)).await?;

	match app_data {
		Some(d) => Ok(d),
		None => {
			return Err(HttpErr::new(
				401,
				ApiErrorCodes::AppTokenNotFound,
				"App token not found".to_string(),
				None,
			))
		},
	}
}

/**
Get jwt data like internal get app data

but this time check with customer and app id and not limited
*/
pub(super) async fn get_jwt_data(customer_id: CustomerId, app_id: AppId) -> Result<Vec<AppJwtData>, HttpErr>
{
	//language=SQL
	let sql = r"
SELECT ak.id, alg, ak.time, sign_key, verify_key 
FROM app a, app_jwt_keys ak 
WHERE 
    app_id = ? AND 
    customer_id = ? AND 
    app_id = a.id 
ORDER BY ak.time DESC";

	let jwt_data: Vec<AppJwtData> = query(sql.to_string(), set_params!(app_id, customer_id)).await?;

	Ok(jwt_data)
}

pub(super) async fn create_app(
	customer_id: &UserId,
	identifier: Option<String>,
	hashed_secret_token: String,
	hashed_public_token: String,
	alg: &str,
	first_jwt_sign_key: &str,
	first_jwt_verify_key: &str,
	first_jwt_alg: &str,
) -> Result<(AppId, JwtKeyId), HttpErr>
{
	let app_id = Uuid::new_v4().to_string();
	let time = get_time()?;

	//language=SQL
	let sql_app = r"
INSERT INTO app 
    (id, 
     customer_id, 
     identifier, 
     hashed_secret_token, 
     hashed_public_token, 
     hash_alg,
     time
     ) 
VALUES (?,?,?,?,?,?,?)";

	let identifier = match identifier {
		Some(i) => i,
		None => "".to_string(),
	};

	let params_app = set_params!(
		app_id.to_string(),
		customer_id.to_string(),
		identifier,
		hashed_secret_token.to_string(),
		hashed_public_token.to_string(),
		alg.to_string(),
		time.to_string()
	);

	let jwt_key_id = Uuid::new_v4().to_string();

	//language=SQL
	let sql_jwt = "INSERT INTO app_jwt_keys (id, app_id, sign_key, verify_key, alg, time) VALUES (?,?,?,?,?,?)";
	let params_jwt = set_params!(
		jwt_key_id.to_string(),
		app_id.to_string(),
		first_jwt_sign_key.to_string(),
		first_jwt_verify_key.to_string(),
		first_jwt_alg.to_string(),
		time.to_string()
	);

	exec_transaction(vec![
		TransactionData {
			sql: sql_app,
			params: params_app,
		},
		TransactionData {
			sql: sql_jwt,
			params: params_jwt,
		},
	])
	.await?;

	Ok((app_id, jwt_key_id))
}

pub(super) async fn token_renew(
	app_id: AppId,
	customer_id: CustomerId,
	hashed_secret_token: String,
	hashed_public_token: String,
	alg: &str,
) -> Result<(), HttpErr>
{
	//language=SQL
	let sql = "UPDATE app SET hashed_secret_token = ?, hashed_public_token = ?, hash_alg = ? WHERE id = ? AND customer_id = ?";

	exec(
		sql,
		set_params!(
			hashed_secret_token,
			hashed_public_token,
			alg.to_string(),
			app_id,
			customer_id
		),
	)
	.await?;

	Ok(())
}

pub(super) async fn add_jwt_keys(
	customer_id: CustomerId,
	app_id: AppId,
	new_jwt_sign_key: &str,
	new_jwt_verify_key: &str,
	new_jwt_alg: &str,
) -> Result<JwtKeyId, HttpErr>
{
	check_app_exists(customer_id, app_id.to_string()).await?;

	let time = get_time()?;
	let jwt_key_id = Uuid::new_v4().to_string();

	//language=SQL
	let sql = "INSERT INTO app_jwt_keys (id, app_id, sign_key, verify_key, alg, time) VALUES (?,?,?,?,?,?)";

	exec(
		sql,
		set_params!(
			jwt_key_id.to_string(),
			app_id.to_string(),
			new_jwt_sign_key.to_string(),
			new_jwt_verify_key.to_string(),
			new_jwt_alg.to_string(),
			time.to_string()
		),
	)
	.await?;

	Ok(jwt_key_id)
}

pub(super) async fn delete_jwt_keys(customer_id: CustomerId, app_id: AppId, jwt_key_id: JwtKeyId) -> Result<(), HttpErr>
{
	check_app_exists(customer_id, app_id.to_string()).await?;

	//language=SQL
	let sql = "DELETE FROM app_jwt_keys WHERE id = ? AND app_id = ?";

	exec(sql, set_params!(jwt_key_id, app_id)).await?;

	Ok(())
}

pub(super) async fn update(customer_id: CustomerId, app_id: AppId, identifier: Option<String>) -> Result<(), HttpErr>
{
	//language=SQL
	let sql = "UPDATE app SET identifier = ? WHERE customer_id = ? AND id = ?";

	let identifier = match identifier {
		Some(i) => i,
		None => "".to_string(),
	};

	exec(sql, set_params!(identifier, customer_id, app_id)).await?;

	Ok(())
}

pub(super) async fn delete(customer_id: CustomerId, app_id: AppId) -> Result<(), HttpErr>
{
	//use the double check with the customer id to check if this app really belongs to the customer!

	//language=SQL
	let sql = "DELETE FROM app WHERE customer_id = ? AND id = ?";

	exec(sql, set_params!(customer_id, app_id)).await?;

	Ok(())
}

async fn check_app_exists(customer_id: CustomerId, app_id: AppId) -> Result<(), HttpErr>
{
	//check if this app belongs to this customer
	//language=SQL
	let sql = "SELECT 1 FROM app WHERE id = ? AND customer_id = ?";
	let app_exists: Option<AppExistsEntity> = query_first(sql.to_string(), set_params!(app_id, customer_id)).await?;

	match app_exists {
		Some(_) => {},
		None => {
			return Err(HttpErr::new(
				400,
				ApiErrorCodes::AppNotFound,
				"App not found in this user space".to_string(),
				None,
			))
		},
	}

	Ok(())
}