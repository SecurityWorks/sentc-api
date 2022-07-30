use std::collections::HashMap;

use reqwest::header::AUTHORIZATION;
use reqwest::StatusCode;
use sentc_crypto::group::GroupKeyData;
use sentc_crypto::KeyData;
use sentc_crypto_common::group::{GroupCreateOutput, GroupDeleteServerOutput, GroupJoinReqList, GroupKeysForNewMemberServerInput, GroupServerData};
use sentc_crypto_common::server_default::ServerSuccessOutput;
use sentc_crypto_common::{GroupId, ServerOutput, UserId};
use server_api::core::api_res::ApiErrorCodes;
use server_api::AppRegisterOutput;
use tokio::sync::{OnceCell, RwLock};

use crate::test_fn::{auth_header, create_app, create_group, create_test_user, delete_app, delete_user, get_group, get_url};

mod test_fn;

pub struct UserState
{
	pub username: String,
	pub pw: String,
	pub user_id: UserId,
	pub key_data: KeyData,
}

pub struct GroupState
{
	pub group_id: GroupId,
	pub group_member: Vec<UserId>,
	pub decrypted_group_keys: HashMap<UserId, Vec<GroupKeyData>>,
}

static APP_TEST_STATE: OnceCell<RwLock<AppRegisterOutput>> = OnceCell::const_new();
static USERS_TEST_STATE: OnceCell<RwLock<Vec<UserState>>> = OnceCell::const_new();
static GROUP_TEST_STATE: OnceCell<RwLock<GroupState>> = OnceCell::const_new();

#[tokio::test]
async fn aaa_init_global_test()
{
	//create here an app
	let app_data = create_app().await;

	let secret_token = app_data.secret_token.to_string();
	let public_token = app_data.public_token.to_string();

	APP_TEST_STATE
		.get_or_init(|| async move { RwLock::new(app_data) })
		.await;

	let mut users = vec![];

	let user_pw = "12345";

	let secret_token_str = secret_token.as_str();
	let public_token_str = public_token.as_str();

	for i in 0..5 {
		let username = "hi".to_string() + i.to_string().as_str();

		let (user_id, key_data) = create_test_user(secret_token_str, public_token_str, username.as_str(), user_pw).await;

		let user = UserState {
			username,
			pw: user_pw.to_string(),
			user_id,
			key_data,
		};

		users.push(user);
	}

	USERS_TEST_STATE
		.get_or_init(|| async move { RwLock::new(users) })
		.await;

	GROUP_TEST_STATE
		.get_or_init(|| {
			async move {
				RwLock::new(GroupState {
					group_id: "".to_string(),
					group_member: vec![],
					decrypted_group_keys: HashMap::new(),
				})
			}
		})
		.await;
}

//__________________________________________________________________________________________________
//tests start

#[tokio::test]
async fn test_10_create_group()
{
	let secret_token = &APP_TEST_STATE.get().unwrap().read().await.secret_token;
	let mut group = GROUP_TEST_STATE.get().unwrap().write().await;

	let creator = USERS_TEST_STATE.get().unwrap().read().await;
	let creator = &creator[0];

	let group_input = sentc_crypto::group::prepare_create(&creator.key_data.public_key, None).unwrap();

	let url = get_url("api/v1/group".to_owned());
	let client = reqwest::Client::new();
	let res = client
		.post(url)
		.header(AUTHORIZATION, auth_header(creator.key_data.jwt.as_str()))
		.header("x-sentc-app-token", secret_token)
		.body(group_input)
		.send()
		.await
		.unwrap();

	assert_eq!(res.status(), StatusCode::OK);

	let body = res.text().await.unwrap();
	let out = ServerOutput::<GroupCreateOutput>::from_string(body.as_str()).unwrap();

	assert_eq!(out.status, true);
	assert_eq!(out.err_code, None);

	let out = out.result.unwrap();

	group.group_id = out.group_id;
}

#[tokio::test]
async fn test_11_get_group_data()
{
	let secret_token = &APP_TEST_STATE.get().unwrap().read().await.secret_token;
	let mut group = GROUP_TEST_STATE.get().unwrap().write().await;

	let creator = USERS_TEST_STATE.get().unwrap().read().await;
	let creator = &creator[0];

	let url = get_url("api/v1/group/".to_owned() + group.group_id.as_str());
	let client = reqwest::Client::new();
	let res = client
		.get(url)
		.header(AUTHORIZATION, auth_header(creator.key_data.jwt.as_str()))
		.header("x-sentc-app-token", secret_token)
		.send()
		.await
		.unwrap();

	assert_eq!(res.status(), StatusCode::OK);

	let body = res.text().await.unwrap();

	let out = ServerOutput::<GroupServerData>::from_string(body.as_str()).unwrap();

	assert_eq!(out.status, true);
	assert_eq!(out.err_code, None);

	//check if result is there
	let _out = out.result.unwrap();

	let data = sentc_crypto::group::get_group_data(&creator.key_data.private_key, body.as_str()).unwrap();

	//user is the creator
	assert_eq!(data.rank, 0);

	group
		.decrypted_group_keys
		.insert(creator.user_id.to_string(), data.keys);
}

#[tokio::test]
async fn test_12_create_child_group()
{
	let secret_token = &APP_TEST_STATE.get().unwrap().read().await.secret_token;
	let group = GROUP_TEST_STATE.get().unwrap().read().await;

	let creator = USERS_TEST_STATE.get().unwrap().read().await;
	let creator = &creator[0];

	let child_id = create_group(
		secret_token,
		&creator.key_data.public_key,
		Some(group.group_id.to_string()),
		creator.key_data.jwt.as_str(),
	)
	.await;

	let data = get_group(
		secret_token,
		creator.key_data.jwt.as_str(),
		child_id.as_str(),
		&creator.key_data.private_key,
	)
	.await;

	assert_eq!(data.rank, 0);
	assert_eq!(data.group_id, child_id);
	assert_eq!(data.parent_group_id.unwrap(), group.group_id.to_string());

	//don't delete the child group to test if parent group delete deletes all. delete the child
}

#[tokio::test]
async fn test_13_invite_user()
{
	let secret_token = &APP_TEST_STATE.get().unwrap().read().await.secret_token;
	let group = GROUP_TEST_STATE.get().unwrap().read().await;

	let users = USERS_TEST_STATE.get().unwrap().read().await;
	let creator = &users[0];

	let user_to_invite = &users[1];

	let mut group_keys_ref = vec![];

	let user_keys = group
		.decrypted_group_keys
		.get(creator.user_id.as_str())
		.unwrap();

	for decrypted_group_key in user_keys {
		group_keys_ref.push(&decrypted_group_key.group_key);
	}

	let invite = sentc_crypto::group::prepare_group_keys_for_new_member(&user_to_invite.key_data.exported_public_key, &group_keys_ref).unwrap();

	let url = get_url("api/v1/group/".to_owned() + group.group_id.as_str() + "/invite/" + user_to_invite.user_id.as_str());

	let client = reqwest::Client::new();
	let res = client
		.put(url)
		.header(AUTHORIZATION, auth_header(creator.key_data.jwt.as_str()))
		.header("x-sentc-app-token", secret_token)
		.body(invite)
		.send()
		.await
		.unwrap();

	let body = res.text().await.unwrap();
	let out = ServerOutput::<ServerSuccessOutput>::from_string(body.as_str()).unwrap();

	assert_eq!(out.status, true);
	assert_eq!(out.err_code, None);
}

#[tokio::test]
async fn test_14_not_invite_user_without_keys()
{
	let secret_token = &APP_TEST_STATE.get().unwrap().read().await.secret_token;
	let group = GROUP_TEST_STATE.get().unwrap().read().await;

	let users = USERS_TEST_STATE.get().unwrap().read().await;
	let creator = &users[0];

	let user_to_invite = &users[1];

	let mut group_keys_ref = vec![];

	let user_keys = group
		.decrypted_group_keys
		.get(creator.user_id.as_str())
		.unwrap();

	for decrypted_group_key in user_keys {
		group_keys_ref.push(&decrypted_group_key.group_key);
	}

	//no keys -> must be an error
	let input = GroupKeysForNewMemberServerInput(Vec::new());

	let url = get_url("api/v1/group/".to_owned() + group.group_id.as_str() + "/invite/" + user_to_invite.user_id.as_str());

	let client = reqwest::Client::new();
	let res = client
		.put(url)
		.header(AUTHORIZATION, auth_header(creator.key_data.jwt.as_str()))
		.header("x-sentc-app-token", secret_token)
		.body(input.to_string().unwrap())
		.send()
		.await
		.unwrap();

	let body = res.text().await.unwrap();
	let out = ServerOutput::<ServerSuccessOutput>::from_string(body.as_str()).unwrap();

	assert_eq!(out.status, false);
	assert_eq!(out.err_code, Some(303));
}

#[tokio::test]
async fn test_15_accept_invite()
{
	let secret_token = &APP_TEST_STATE.get().unwrap().read().await.secret_token;
	let mut group = GROUP_TEST_STATE.get().unwrap().write().await;

	let users = USERS_TEST_STATE.get().unwrap().read().await;

	let user_to_invite = &users[1];

	let url = get_url("api/v1/group/".to_owned() + group.group_id.as_str() + "/invite");

	let client = reqwest::Client::new();
	let res = client
		.patch(url)
		.header(AUTHORIZATION, auth_header(user_to_invite.key_data.jwt.as_str()))
		.header("x-sentc-app-token", secret_token)
		.send()
		.await
		.unwrap();

	let body = res.text().await.unwrap();

	let out = ServerOutput::<ServerSuccessOutput>::from_string(body.as_str()).unwrap();

	assert_eq!(out.status, true);
	assert_eq!(out.err_code, None);

	//test get group as new user
	let data = get_group(
		secret_token,
		user_to_invite.key_data.jwt.as_str(),
		group.group_id.as_str(),
		&user_to_invite.key_data.private_key,
	)
	.await;

	//should be normal user rank
	assert_eq!(data.rank, 4);

	group
		.decrypted_group_keys
		.insert(user_to_invite.user_id.to_string(), data.keys);

	group.group_member.push(user_to_invite.user_id.to_string());
}

#[tokio::test]
async fn test_16_invite_user_an_reject_invite()
{
	let secret_token = &APP_TEST_STATE.get().unwrap().read().await.secret_token;
	let group = GROUP_TEST_STATE.get().unwrap().read().await;

	let users = USERS_TEST_STATE.get().unwrap().read().await;
	let creator = &users[0];

	let user_to_invite = &users[2];

	let mut group_keys_ref = vec![];

	let user_keys = group
		.decrypted_group_keys
		.get(creator.user_id.as_str())
		.unwrap();

	for decrypted_group_key in user_keys {
		group_keys_ref.push(&decrypted_group_key.group_key);
	}

	let invite = sentc_crypto::group::prepare_group_keys_for_new_member(&user_to_invite.key_data.exported_public_key, &group_keys_ref).unwrap();

	let url = get_url("api/v1/group/".to_owned() + group.group_id.as_str() + "/invite/" + user_to_invite.user_id.as_str());

	let client = reqwest::Client::new();
	let res = client
		.put(url)
		.header(AUTHORIZATION, auth_header(creator.key_data.jwt.as_str()))
		.header("x-sentc-app-token", secret_token)
		.body(invite)
		.send()
		.await
		.unwrap();

	let body = res.text().await.unwrap();
	let out = ServerOutput::<ServerSuccessOutput>::from_string(body.as_str()).unwrap();

	assert_eq!(out.status, true);
	assert_eq!(out.err_code, None);

	//______________________________________________________________________________________________
	//no reject the invite

	let url = get_url("api/v1/group/".to_owned() + group.group_id.as_str() + "/invite");

	let client = reqwest::Client::new();
	let res = client
		.delete(url)
		.header(AUTHORIZATION, auth_header(user_to_invite.key_data.jwt.as_str()))
		.header("x-sentc-app-token", secret_token)
		.send()
		.await
		.unwrap();

	let body = res.text().await.unwrap();

	let out = ServerOutput::<ServerSuccessOutput>::from_string(body.as_str()).unwrap();

	assert_eq!(out.status, true);
	assert_eq!(out.err_code, None);

	//______________________________________________________________________________________________
	//the rejected user should not get the group data

	let url = get_url("api/v1/group/".to_owned() + group.group_id.as_str());
	let client = reqwest::Client::new();
	let res = client
		.get(url)
		.header(AUTHORIZATION, auth_header(user_to_invite.key_data.jwt.as_str()))
		.header("x-sentc-app-token", secret_token)
		.send()
		.await
		.unwrap();

	assert_eq!(res.status(), StatusCode::BAD_REQUEST);

	let body = res.text().await.unwrap();

	let out = ServerOutput::<GroupServerData>::from_string(body.as_str()).unwrap();

	assert_eq!(out.status, false);
	assert_eq!(out.err_code.unwrap(), ApiErrorCodes::GroupUserNotFound.get_int_code());
}

#[tokio::test]
async fn test_17_not_leave_group_when_user_is_the_only_admin()
{
	let secret_token = &APP_TEST_STATE.get().unwrap().read().await.secret_token;
	let group = GROUP_TEST_STATE.get().unwrap().read().await;

	let users = USERS_TEST_STATE.get().unwrap().read().await;
	let creator = &users[0];

	let url = get_url("api/v1/group/".to_owned() + group.group_id.as_str() + "/leave");
	let client = reqwest::Client::new();
	let res = client
		.delete(url)
		.header(AUTHORIZATION, auth_header(creator.key_data.jwt.as_str()))
		.header("x-sentc-app-token", secret_token)
		.send()
		.await
		.unwrap();

	assert_eq!(res.status(), StatusCode::BAD_REQUEST);

	let body = res.text().await.unwrap();

	let out = ServerOutput::<ServerSuccessOutput>::from_string(body.as_str()).unwrap();
	assert_eq!(out.status, false);

	//should get the data without error
	let _data = get_group(
		secret_token,
		creator.key_data.jwt.as_str(),
		group.group_id.as_str(),
		&creator.key_data.private_key,
	)
	.await;
}

#[tokio::test]
async fn test_18_leave_group()
{
	let secret_token = &APP_TEST_STATE.get().unwrap().read().await.secret_token;
	let group = GROUP_TEST_STATE.get().unwrap().read().await;

	let users = USERS_TEST_STATE.get().unwrap().read().await;
	let user = &users[1];

	let url = get_url("api/v1/group/".to_owned() + group.group_id.as_str() + "/leave");
	let client = reqwest::Client::new();
	let res = client
		.delete(url)
		.header(AUTHORIZATION, auth_header(user.key_data.jwt.as_str()))
		.header("x-sentc-app-token", secret_token)
		.send()
		.await
		.unwrap();

	assert_eq!(res.status(), StatusCode::OK);

	let body = res.text().await.unwrap();

	let out = ServerOutput::<ServerSuccessOutput>::from_string(body.as_str()).unwrap();

	assert_eq!(out.status, true);
	assert_eq!(out.err_code, None);

	//this user should not get the group data
	let url = get_url("api/v1/group/".to_owned() + group.group_id.as_str());
	let client = reqwest::Client::new();
	let res = client
		.get(url)
		.header(AUTHORIZATION, auth_header(user.key_data.jwt.as_str()))
		.header("x-sentc-app-token", secret_token)
		.send()
		.await
		.unwrap();

	assert_eq!(res.status(), StatusCode::BAD_REQUEST);

	let body = res.text().await.unwrap();

	let out = ServerOutput::<GroupServerData>::from_string(body.as_str()).unwrap();

	assert_eq!(out.status, false);
	assert_eq!(out.err_code.unwrap(), ApiErrorCodes::GroupUserNotFound.get_int_code());
}

#[tokio::test]
async fn test_19_join_req()
{
	let secret_token = &APP_TEST_STATE.get().unwrap().read().await.secret_token;
	let group = GROUP_TEST_STATE.get().unwrap().read().await;

	let users = USERS_TEST_STATE.get().unwrap().read().await;
	let user = &users[1];

	let url = get_url("api/v1/group/".to_owned() + group.group_id.as_str() + "/join_req");
	let client = reqwest::Client::new();
	let res = client
		.patch(url)
		.header(AUTHORIZATION, auth_header(user.key_data.jwt.as_str()))
		.header("x-sentc-app-token", secret_token)
		.send()
		.await
		.unwrap();

	assert_eq!(res.status(), StatusCode::OK);

	let body = res.text().await.unwrap();

	let out = ServerOutput::<ServerSuccessOutput>::from_string(body.as_str()).unwrap();

	assert_eq!(out.status, true);
	assert_eq!(out.err_code, None);
}

#[tokio::test]
async fn test_20_get_join_req()
{
	let group = GROUP_TEST_STATE.get().unwrap().read().await;

	let users = USERS_TEST_STATE.get().unwrap().read().await;
	let creator = &users[0];
	let secret_token = &APP_TEST_STATE.get().unwrap().read().await.secret_token;

	//get the first page
	let url = get_url("api/v1/group/".to_owned() + group.group_id.as_str() + "/join_req/" + "0");
	let client = reqwest::Client::new();
	let res = client
		.get(url)
		.header(AUTHORIZATION, auth_header(creator.key_data.jwt.as_str()))
		.header("x-sentc-app-token", secret_token)
		.send()
		.await
		.unwrap();

	assert_eq!(res.status(), StatusCode::OK);

	let body = res.text().await.unwrap();

	let out = ServerOutput::<Vec<GroupJoinReqList>>::from_string(body.as_str()).unwrap();

	assert_eq!(out.status, true);
	assert_eq!(out.err_code, None);

	let out = out.result.unwrap();

	assert_eq!(out.len(), 1);
	assert_eq!(out[0].user_id.to_string(), users[1].user_id.to_string());
}

#[tokio::test]
async fn test_21_send_join_req_aging()
{
	//this should not err because of insert ignore

	let secret_token = &APP_TEST_STATE.get().unwrap().read().await.secret_token;
	let group = GROUP_TEST_STATE.get().unwrap().read().await;

	let users = USERS_TEST_STATE.get().unwrap().read().await;
	let user = &users[1];

	let url = get_url("api/v1/group/".to_owned() + group.group_id.as_str() + "/join_req");
	let client = reqwest::Client::new();
	let res = client
		.patch(url)
		.header(AUTHORIZATION, auth_header(user.key_data.jwt.as_str()))
		.header("x-sentc-app-token", secret_token)
		.send()
		.await
		.unwrap();

	assert_eq!(res.status(), StatusCode::OK);

	let body = res.text().await.unwrap();

	let out = ServerOutput::<ServerSuccessOutput>::from_string(body.as_str()).unwrap();

	assert_eq!(out.status, true);
	assert_eq!(out.err_code, None);

	//______________________________________________________________________________________________
	let creator = &users[0];

	//should still be this one join req
	//get the first page
	let url = get_url("api/v1/group/".to_owned() + group.group_id.as_str() + "/join_req/" + "0");
	let client = reqwest::Client::new();
	let res = client
		.get(url)
		.header(AUTHORIZATION, auth_header(creator.key_data.jwt.as_str()))
		.header("x-sentc-app-token", secret_token)
		.send()
		.await
		.unwrap();

	assert_eq!(res.status(), StatusCode::OK);

	let body = res.text().await.unwrap();

	let out = ServerOutput::<Vec<GroupJoinReqList>>::from_string(body.as_str()).unwrap();

	assert_eq!(out.status, true);
	assert_eq!(out.err_code, None);

	let out = out.result.unwrap();

	assert_eq!(out.len(), 1);
	assert_eq!(out[0].user_id.to_string(), users[1].user_id.to_string());
}

#[tokio::test]
async fn test_22_reject_join_req()
{
	let group = GROUP_TEST_STATE.get().unwrap().read().await;

	let users = USERS_TEST_STATE.get().unwrap().read().await;
	let creator = &users[0];
	let secret_token = &APP_TEST_STATE.get().unwrap().read().await.secret_token;

	//get the first page
	let url = get_url("api/v1/group/".to_owned() + group.group_id.as_str() + "/join_req/" + users[1].user_id.as_str());
	let client = reqwest::Client::new();
	let res = client
		.delete(url)
		.header(AUTHORIZATION, auth_header(creator.key_data.jwt.as_str()))
		.header("x-sentc-app-token", secret_token)
		.send()
		.await
		.unwrap();

	assert_eq!(res.status(), StatusCode::OK);

	let body = res.text().await.unwrap();
	let out = ServerOutput::<ServerSuccessOutput>::from_string(body.as_str()).unwrap();

	assert_eq!(out.status, true);
	assert_eq!(out.err_code, None);
}

#[tokio::test]
async fn test_23_get_not_join_req_after_reject()
{
	let group = GROUP_TEST_STATE.get().unwrap().read().await;

	let users = USERS_TEST_STATE.get().unwrap().read().await;
	let creator = &users[0];
	let secret_token = &APP_TEST_STATE.get().unwrap().read().await.secret_token;

	//get the first page
	let url = get_url("api/v1/group/".to_owned() + group.group_id.as_str() + "/join_req/" + "0");
	let client = reqwest::Client::new();
	let res = client
		.get(url)
		.header(AUTHORIZATION, auth_header(creator.key_data.jwt.as_str()))
		.header("x-sentc-app-token", secret_token)
		.send()
		.await
		.unwrap();

	assert_eq!(res.status(), StatusCode::OK);

	let body = res.text().await.unwrap();

	let out = ServerOutput::<Vec<GroupJoinReqList>>::from_string(body.as_str()).unwrap();

	assert_eq!(out.status, true);
	assert_eq!(out.err_code, None);

	let out = out.result.unwrap();

	assert_eq!(out.len(), 0);
}

#[tokio::test]
async fn test_24_accept_join_req()
{
	//1. send the join req again, because we were rejecting the last one
	let secret_token = &APP_TEST_STATE.get().unwrap().read().await.secret_token;
	let group = GROUP_TEST_STATE.get().unwrap().read().await;

	let users = USERS_TEST_STATE.get().unwrap().read().await;
	let user = &users[1];

	let url = get_url("api/v1/group/".to_owned() + group.group_id.as_str() + "/join_req");
	let client = reqwest::Client::new();
	let res = client
		.patch(url)
		.header(AUTHORIZATION, auth_header(user.key_data.jwt.as_str()))
		.header("x-sentc-app-token", secret_token)
		.send()
		.await
		.unwrap();

	assert_eq!(res.status(), StatusCode::OK);

	let body = res.text().await.unwrap();

	let out = ServerOutput::<ServerSuccessOutput>::from_string(body.as_str()).unwrap();

	assert_eq!(out.status, true);
	assert_eq!(out.err_code, None);

	//______________________________________________________________________________________________
	//2. accept this join req
	let creator = &users[0];

	let user_to_accept = &users[1];

	let mut group_keys_ref = vec![];

	let user_keys = group
		.decrypted_group_keys
		.get(creator.user_id.as_str())
		.unwrap();

	for decrypted_group_key in user_keys {
		group_keys_ref.push(&decrypted_group_key.group_key);
	}

	let join = sentc_crypto::group::prepare_group_keys_for_new_member(&user_to_accept.key_data.exported_public_key, &group_keys_ref).unwrap();

	let url = get_url("api/v1/group/".to_owned() + group.group_id.as_str() + "/join_req/" + user_to_accept.user_id.as_str());

	let client = reqwest::Client::new();
	let res = client
		.put(url)
		.header(AUTHORIZATION, auth_header(creator.key_data.jwt.as_str()))
		.header("x-sentc-app-token", secret_token)
		.body(join)
		.send()
		.await
		.unwrap();

	let body = res.text().await.unwrap();
	let out = ServerOutput::<ServerSuccessOutput>::from_string(body.as_str()).unwrap();

	assert_eq!(out.status, true);
	assert_eq!(out.err_code, None);

	//3. should get the group data
	let _data = get_group(
		secret_token,
		user.key_data.jwt.as_str(),
		group.group_id.as_str(),
		&user.key_data.private_key,
	);
}

#[tokio::test]
async fn test_30_delete_group()
{
	let group = GROUP_TEST_STATE.get().unwrap().read().await;

	let creator = USERS_TEST_STATE.get().unwrap().read().await;
	let creator = &creator[0];
	let secret_token = &APP_TEST_STATE.get().unwrap().read().await.secret_token;

	let url = get_url("api/v1/group/".to_owned() + group.group_id.as_str());
	let client = reqwest::Client::new();
	let res = client
		.delete(url)
		.header(AUTHORIZATION, auth_header(creator.key_data.jwt.as_str()))
		.header("x-sentc-app-token", secret_token)
		.send()
		.await
		.unwrap();

	assert_eq!(res.status(), StatusCode::OK);

	let body = res.text().await.unwrap();
	let out = ServerOutput::<GroupDeleteServerOutput>::from_string(body.as_str()).unwrap();

	assert_eq!(out.status, true);
	assert_eq!(out.err_code, None);

	let out = out.result.unwrap();

	assert_eq!(out.group_id, group.group_id);
}

//__________________________________________________________________________________________________
//clean up

#[tokio::test]
async fn zzz_clean_up()
{
	let app = APP_TEST_STATE.get().unwrap().read().await;

	let users = USERS_TEST_STATE.get().unwrap().read().await;

	let secret_token = &APP_TEST_STATE.get().unwrap().read().await.secret_token;

	for user in users.iter() {
		delete_user(secret_token, user.key_data.jwt.as_str()).await;
	}

	delete_app(app.app_id.as_str()).await;
}