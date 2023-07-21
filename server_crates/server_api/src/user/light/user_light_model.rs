use rustgram_server_util::db::id_handling::create_id;
use rustgram_server_util::db::{exec, exec_transaction, query_first, TransactionData};
use rustgram_server_util::error::{ServerCoreError, ServerErrorConstructor};
use rustgram_server_util::res::AppRes;
use rustgram_server_util::{get_time, set_params};
use sentc_crypto_common::user::{KeyDerivedData, KeyDerivedLightData, MasterKey};
use sentc_crypto_common::{AppId, DeviceId, UserId};

use crate::sentc_user_entities::UserLoginLightEntity;
use crate::user::user_model::{check_user_exists, prepare_register_device};
use crate::util::api_res::ApiErrorCodes;

pub(super) async fn register_light(
	app_id: impl Into<AppId>,
	device_identifier: String,
	master_key: MasterKey,
	derived: KeyDerivedLightData,
) -> AppRes<(UserId, DeviceId)>
{
	let app_id = app_id.into();

	//check first if the user identifier is available
	let check = check_user_exists(&app_id, &device_identifier).await?;

	if check {
		//check true == user exists
		return Err(ServerCoreError::new_msg(
			400,
			ApiErrorCodes::UserExists,
			"User already exists",
		));
	}

	//data for the user table
	//language=SQL
	let sql_user = "INSERT INTO sentc_user (id, app_id, user_group_id, time) VALUES (?,?,?,?)";
	let user_id = create_id();
	let time = get_time()?;

	//insert a fake group id for now, and update the user group id when user group was created
	let user_params = set_params!(user_id.clone(), app_id.clone(), "none".to_string(), time.to_string());

	let device_id = create_id();

	//data for the user key table
	let (sql_keys, key_params) = prepare_register_device(
		&device_id,
		&user_id,
		app_id,
		time,
		device_identifier,
		master_key,
		light_derived_to(derived),
		None,
	);

	exec_transaction(vec![
		TransactionData {
			sql: sql_user,
			params: user_params,
		},
		TransactionData {
			sql: sql_keys,
			params: key_params,
		},
	])
	.await?;

	Ok((user_id, device_id))
}

pub(super) async fn register_device_light(
	app_id: impl Into<AppId>,
	device_identifier: String,
	master_key: MasterKey,
	derived: KeyDerivedLightData,
	token: impl Into<String>,
) -> AppRes<DeviceId>
{
	let device_id = create_id();
	let time = get_time()?;

	let (sql_keys, key_params) = prepare_register_device(
		&device_id,
		"not_registered",
		app_id,
		time,
		device_identifier,
		master_key,
		light_derived_to(derived),
		Some(token.into()),
	);

	exec(sql_keys, key_params).await?;

	Ok(device_id)
}

pub(super) async fn get_done_login_light_data(app_id: impl Into<AppId>, user_identifier: impl Into<String>) -> AppRes<Option<UserLoginLightEntity>>
{
	//language=SQL
	let sql = r"
SELECT user_id, ud.id as device_id
FROM 
    sentc_user_device ud, 
    sentc_user u 
WHERE 
    device_identifier = ? AND 
    user_id = u.id AND 
    u.app_id = ?";

	let data: Option<UserLoginLightEntity> = query_first(sql, set_params!(user_identifier.into(), app_id.into())).await?;

	Ok(data)
}

//__________________________________________________________________________________________________

fn light_derived_to(derived: KeyDerivedLightData) -> KeyDerivedData
{
	KeyDerivedData {
		derived_alg: derived.derived_alg,
		client_random_value: derived.client_random_value,
		hashed_authentication_key: derived.hashed_authentication_key,
		public_key: derived.public_key,
		encrypted_private_key: derived.encrypted_private_key,
		keypair_encrypt_alg: derived.keypair_encrypt_alg,
		//no verify key
		verify_key: "".to_string(),
		encrypted_sign_key: "".to_string(),
		keypair_sign_alg: "".to_string(),
	}
}
