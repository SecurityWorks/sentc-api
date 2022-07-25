use reqwest::StatusCode;
use sentc_crypto::KeyData;
use sentc_crypto_common::user::{
	DoneLoginServerKeysOutput,
	MasterKey,
	RegisterServerOutput,
	UserDeleteServerOutput,
	UserIdentifierAvailableServerInput,
	UserIdentifierAvailableServerOutput,
};
use sentc_crypto_common::ServerOutput;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use server_api::core::api_res::ApiErrorCodes;
use tokio::sync::{OnceCell, RwLock};

use crate::test_fn::{delete_user, get_url, register_user};

mod test_fn;

pub struct UserState
{
	pub username: String,
	pub pw: String,
	pub user_id: String,
	pub key_data: Option<KeyData>,
}

static USER_TEST_STATE: OnceCell<RwLock<UserState>> = OnceCell::const_new();

#[tokio::test]
async fn aaa_init_global_test()
{
	//this fn must be execute first!
	USER_TEST_STATE
		.get_or_init(|| {
			async {
				RwLock::new(UserState {
					username: "admin_test".to_string(),
					pw: "12345".to_string(),
					user_id: "".to_string(),
					key_data: None,
				})
			}
		})
		.await;
}

#[tokio::test]
async fn test_1_user_exists()
{
	let username = &USER_TEST_STATE.get().unwrap().read().await.username;

	//test if user exists
	let input = UserIdentifierAvailableServerInput {
		user_identifier: username.to_owned(),
	};

	let url = get_url("api/v1/exists".to_owned());

	let client = reqwest::Client::new();
	let res = client
		.post(url)
		.body(input.to_string().unwrap())
		.send()
		.await
		.unwrap();

	assert_eq!(res.status(), StatusCode::OK);

	let body = res.text().await.unwrap();

	let exists = ServerOutput::<UserIdentifierAvailableServerOutput>::from_string(body.as_str()).unwrap();

	assert_eq!(exists.status, true);
	assert_eq!(exists.err_code, None);

	let exists = match exists.result {
		Some(v) => v,
		None => panic!("exists is not here"),
	};

	assert_eq!(exists.user_identifier, username.to_string());
	assert_eq!(exists.available, false);
}

#[tokio::test]
async fn test_2_user_register()
{
	let mut user = USER_TEST_STATE.get().unwrap().write().await;

	let username = &user.username;
	let pw = &user.pw;

	let url = get_url("api/v1/register".to_owned());

	let input = sentc_crypto::user::register(username, pw).unwrap();

	let client = reqwest::Client::new();
	let res = client.post(url).body(input).send().await.unwrap();

	assert_eq!(res.status(), StatusCode::OK);

	let body = res.text().await.unwrap();

	//check it here (not like the client) to see if the server respond correctly
	let register_out = ServerOutput::<RegisterServerOutput>::from_string(body.as_str()).unwrap();

	assert_eq!(register_out.status, true);
	assert_eq!(register_out.err_code, None);

	let register_out = register_out.result.unwrap();
	assert_eq!(register_out.user_identifier, username.to_string());

	//get the user id like the client
	let user_id = sentc_crypto::user::done_register(body.as_str()).unwrap();

	assert_ne!(user_id, "".to_owned());

	//save the user id
	user.user_id = user_id;
}

#[tokio::test]
async fn test_3_user_check_after_register()
{
	let username = &USER_TEST_STATE.get().unwrap().read().await.username;

	//test if user exists
	let input = UserIdentifierAvailableServerInput {
		user_identifier: username.to_string(),
	};

	let url = get_url("api/v1/exists".to_owned());

	let client = reqwest::Client::new();
	let res = client
		.post(url)
		.body(input.to_string().unwrap())
		.send()
		.await
		.unwrap();

	assert_eq!(res.status(), StatusCode::OK);

	let body = res.text().await.unwrap();

	let exists = ServerOutput::<UserIdentifierAvailableServerOutput>::from_string(body.as_str()).unwrap();

	assert_eq!(exists.status, true);
	assert_eq!(exists.err_code, None);

	let exists = exists.result.unwrap();

	assert_eq!(exists.user_identifier, username.to_string());
	assert_eq!(exists.available, true);
}

#[tokio::test]
async fn test_4_user_register_failed_username_exists()
{
	let user = &USER_TEST_STATE.get().unwrap().read().await;
	let username = &user.username;
	let pw = &user.pw;

	let url = get_url("api/v1/register".to_owned());

	let input = sentc_crypto::user::register(username, pw).unwrap();

	let client = reqwest::Client::new();
	let res = client.post(url).body(input).send().await.unwrap();

	assert_eq!(res.status(), StatusCode::BAD_REQUEST);

	let body = res.text().await.unwrap();
	let error = ServerOutput::<RegisterServerOutput>::from_string(body.as_str()).unwrap();

	assert_eq!(error.status, false);
	assert_eq!(error.result.is_none(), true);
	assert_eq!(error.err_code.unwrap(), ApiErrorCodes::UserExists.get_int_code());

	//check err in sdk
	match sentc_crypto::user::done_register(body.as_str()) {
		Ok(_v) => {
			panic!("this should not be Ok")
		},
		Err(e) => {
			match e {
				sentc_crypto::SdkError::ServerErr(s, m) => {
					//this should be the right err
					//this are the same err as the backend
					assert_eq!(error.err_code.unwrap(), s);
					assert_eq!(error.err_msg.unwrap(), m);
				},
				_ => panic!("this should not be the right error code"),
			}
		},
	}
}

#[tokio::test]
async fn test_5_login()
{
	//make the pre req to the sever with the username
	let url = get_url("api/v1/prepare_login".to_owned());

	let mut user = USER_TEST_STATE.get().unwrap().write().await;
	let username = &user.username;
	let pw = &user.pw;

	let prep_server_input = sentc_crypto::user::prepare_login_start(username.as_str()).unwrap();

	let client = reqwest::Client::new();
	let res = client
		.post(url)
		.body(prep_server_input)
		.send()
		.await
		.unwrap();

	let body = res.text().await.unwrap();

	let (auth_key, derived_master_key) = sentc_crypto::user::prepare_login(username, pw, body.as_str()).unwrap();

	// //done login
	let url = get_url("api/v1/done_login".to_owned());

	let client = reqwest::Client::new();
	let res = client.post(url).body(auth_key).send().await.unwrap();

	let body = res.text().await.unwrap();

	let done_login = sentc_crypto::user::done_login(&derived_master_key, body.as_str()).unwrap();

	user.key_data = Some(done_login);
}

#[tokio::test]
async fn test_6_login_with_wrong_password()
{
	//make the pre req to the sever with the username
	let url = get_url("api/v1/prepare_login".to_owned());

	let user = &USER_TEST_STATE.get().unwrap().read().await;
	let username = &user.username;
	let pw = "wrong_password"; //the wording pw

	let prep_server_input = sentc_crypto::user::prepare_login_start(username.as_str()).unwrap();

	let client = reqwest::Client::new();
	let res = client
		.post(url)
		.body(prep_server_input)
		.send()
		.await
		.unwrap();

	let body = res.text().await.unwrap();

	let (auth_key, derived_master_key) = sentc_crypto::user::prepare_login(username, pw, body.as_str()).unwrap();

	// //done login
	let url = get_url("api/v1/done_login".to_owned());

	let client = reqwest::Client::new();
	let res = client.post(url).body(auth_key).send().await.unwrap();

	let body = res.text().await.unwrap();
	let login_output = ServerOutput::<DoneLoginServerKeysOutput>::from_string(body.as_str()).unwrap();

	assert_eq!(login_output.status, false);
	assert_eq!(login_output.result.is_none(), true);
	assert_eq!(login_output.err_code.unwrap(), ApiErrorCodes::Login.get_int_code());

	match sentc_crypto::user::done_login(&derived_master_key, body.as_str()) {
		Ok(_v) => {
			panic!("this should not be Ok")
		},
		Err(e) => {
			match e {
				sentc_crypto::SdkError::ServerErr(s, m) => {
					//this should be the right err
					//this are the same err as the backend
					assert_eq!(login_output.err_code.unwrap(), s);
					assert_eq!(login_output.err_msg.unwrap(), m);
				},
				_ => panic!("this should not be the right error code"),
			}
		},
	}
}

//do user tests before this one!

#[tokio::test]
async fn test_7_user_delete()
{
	let user_id = &USER_TEST_STATE.get().unwrap().read().await.user_id;

	let url = get_url("api/v1/user/".to_owned() + user_id);
	let client = reqwest::Client::new();
	let res = client.delete(url).send().await.unwrap();

	assert_eq!(res.status(), StatusCode::OK);

	let body = res.text().await.unwrap();
	let delete_output = ServerOutput::<UserDeleteServerOutput>::from_string(body.as_str()).unwrap();

	assert_eq!(delete_output.status, true);
	assert_eq!(delete_output.err_code, None);

	let delete_output = delete_output.result.unwrap();
	assert_eq!(delete_output.user_id, user_id.to_string());
	assert_eq!(delete_output.msg, "User deleted");

	//TODO validate it with the sdk done user delete
}

#[derive(Serialize, Deserialize)]
pub struct WrongRegisterData
{
	pub master_key: MasterKey,
}

impl WrongRegisterData
{
	pub fn from_string(v: &str) -> serde_json::Result<Self>
	{
		from_str::<Self>(v)
	}

	pub fn to_string(&self) -> serde_json::Result<String>
	{
		to_string(self)
	}
}

#[tokio::test]
async fn test_8_not_register_user_with_wrong_input()
{
	let url = get_url("api/v1/register".to_owned());

	let input = WrongRegisterData {
		master_key: MasterKey {
			master_key_alg: "123".to_string(),
			encrypted_master_key: "321".to_string(),
			encrypted_master_key_alg: "11".to_string(),
		},
	};

	let str = input.to_string().unwrap();

	let client = reqwest::Client::new();
	let res = client.post(url).body(str).send().await.unwrap();

	assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);

	let body = res.text().await.unwrap();
	let error = ServerOutput::<RegisterServerOutput>::from_string(body.as_str()).unwrap();

	assert_eq!(error.status, false);
	assert_eq!(error.result.is_none(), true);
	assert_eq!(error.err_code.unwrap(), ApiErrorCodes::JsonParse.get_int_code());

	//check err in sdk
	match sentc_crypto::user::done_register(body.as_str()) {
		Ok(_v) => {
			panic!("this should not be Ok")
		},
		Err(e) => {
			match e {
				sentc_crypto::SdkError::ServerErr(s, m) => {
					//this should be the right err
					//this are the same err as the backend
					assert_eq!(error.err_code.unwrap(), s);
					assert_eq!(error.err_msg.unwrap(), m);
				},
				_ => panic!("this should not be the right error code"),
			}
		},
	}
}

#[tokio::test]
async fn test_9_register_user_via_test_fn()
{
	let id = register_user("hello", "12345").await;

	delete_user(&id).await;
}
