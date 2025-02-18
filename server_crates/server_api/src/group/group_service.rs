use std::collections::HashMap;

use rustgram_server_util::cache;
use rustgram_server_util::res::AppRes;
use sentc_crypto_common::group::{CreateData, GroupLightServerData, GroupUserAccessBy};
use sentc_crypto_common::{AppId, GroupId, SymKeyId, UserId};
use server_api_common::group::group_entities::InternalGroupDataComplete;
use server_api_common::util::get_group_cache_key;
use server_key_store::KeyStorage;

pub use self::group_model::{create_light as create_group_light, delete_user_group, get_first_level_children, get_group_hmac, get_group_sortable};
use crate::group::group_entities::GroupServerData;
use crate::group::group_model;
use crate::sentc_group_entities::GroupUserKeys;
use crate::sentc_user_entities::UserPublicKeyDataEntity;

#[inline(always)]
fn prepare_keys_for_extern_fetch(keys_to_fetch: &mut Vec<String>, key: &GroupUserKeys)
{
	if key.encrypted_private_group_key == "extern" {
		keys_to_fetch.push(format!("sk_{}", key.key_pair_id));
	}

	if key.public_group_key == "extern" {
		keys_to_fetch.push(format!("pk_{}", key.key_pair_id));
	}

	if let (Some(sign_key_id), Some(sign_key), Some(verify_key)) = (
		key.keypair_sign_id.as_ref(),
		key.encrypted_sign_key.as_ref(),
		key.verify_key.as_ref(),
	) {
		if sign_key == "extern" {
			keys_to_fetch.push(format!("sign_k_{sign_key_id}"));
		}

		if verify_key == "extern" {
			keys_to_fetch.push(format!("vk_{sign_key_id}"));
		}
	}
}

#[inline(always)]
fn get_keys_from_extern_result(key: &mut GroupUserKeys, fetched_keys: &mut HashMap<String, String>)
{
	if key.encrypted_private_group_key == "extern" {
		if let Some(fetched_key) = fetched_keys.remove(&format!("sk_{}", key.key_pair_id)) {
			key.encrypted_private_group_key = fetched_key
		}
	}

	if key.public_group_key == "extern" {
		if let Some(fetched_key) = fetched_keys.remove(&format!("pk_{}", key.key_pair_id)) {
			key.public_group_key = fetched_key
		}
	}

	if let (Some(sign_key_id), Some(sign_key), Some(verify_key)) = (
		key.keypair_sign_id.as_ref(),
		key.encrypted_sign_key.as_ref(),
		key.verify_key.as_ref(),
	) {
		if sign_key == "extern" {
			if let Some(fetched_key) = fetched_keys.remove(&format!("sign_k_{sign_key_id}")) {
				key.encrypted_sign_key = Some(fetched_key);
			}
		}

		if verify_key == "extern" {
			if let Some(fetched_key) = fetched_keys.remove(&format!("vk_{sign_key_id}")) {
				key.verify_key = Some(fetched_key);
			}
		}
	}
}

pub async fn get_user_group_keys(
	app_id: impl Into<AppId>,
	group_id: impl Into<GroupId>,
	user_id: impl Into<UserId>,
	last_fetched_time: u128,
	last_k_id: impl Into<SymKeyId>,
) -> AppRes<Vec<GroupUserKeys>>
{
	let mut out = group_model::get_user_group_keys(app_id, group_id, user_id, last_fetched_time, last_k_id).await?;

	//check if a key needs to be fetched from the storage
	let mut keys_to_fetch = vec![];

	for key in &out {
		prepare_keys_for_extern_fetch(&mut keys_to_fetch, key);
	}

	//fetch the keys
	let mut fetched_keys = server_key_store::get_keys(&keys_to_fetch).await?;

	for key in &mut out {
		get_keys_from_extern_result(key, &mut fetched_keys);
	}

	Ok(out)
}

pub async fn get_user_group_key(
	app_id: impl Into<AppId>,
	group_id: impl Into<GroupId>,
	user_id: impl Into<UserId>,
	key_id: impl Into<SymKeyId>,
) -> AppRes<GroupUserKeys>
{
	let mut key = group_model::get_user_group_key(app_id, group_id, user_id, key_id).await?;

	let mut keys_to_fetch = vec![];

	prepare_keys_for_extern_fetch(&mut keys_to_fetch, &key);

	let mut fetched_keys = server_key_store::get_keys(&keys_to_fetch).await?;

	get_keys_from_extern_result(&mut key, &mut fetched_keys);

	Ok(key)
}

pub async fn create_group(
	app_id: impl Into<AppId>,
	user_id: impl Into<UserId>,
	data: CreateData,
	group_type: i32,
	parent_group_id: Option<GroupId>,
	user_rank: Option<i32>,
	connected_group: Option<GroupId>,
	is_connected_group: bool,
) -> AppRes<(GroupId, SymKeyId)>
{
	let (encrypted_sign_key_for_model, encrypted_sign_key_for_storage, verify_key_for_model, verify_key_for_storage) =
		if let (Some(encrypted_sign_key), Some(verify_key)) = (data.encrypted_sign_key, data.verify_key) {
			(
				Some("extern".to_string()),
				Some(encrypted_sign_key),
				Some("extern".to_string()),
				Some(verify_key),
			)
		} else {
			(None, None, None, None)
		};

	let create_data = CreateData {
		encrypted_group_key: data.encrypted_group_key,
		group_key_alg: data.group_key_alg,
		encrypted_group_key_alg: data.encrypted_group_key_alg,
		encrypted_private_group_key: "extern".to_string(),
		public_group_key: "extern".to_string(),
		keypair_encrypt_alg: data.keypair_encrypt_alg,
		creator_public_key_id: data.creator_public_key_id,
		encrypted_hmac_key: data.encrypted_hmac_key,
		encrypted_hmac_alg: data.encrypted_hmac_alg,
		encrypted_sortable_key: data.encrypted_sortable_key,
		encrypted_sortable_alg: data.encrypted_sortable_alg,
		signed_by_user_id: data.signed_by_user_id,
		signed_by_user_sign_key_id: data.signed_by_user_sign_key_id,
		group_key_sig: data.group_key_sig,
		encrypted_sign_key: encrypted_sign_key_for_model,
		verify_key: verify_key_for_model,
		keypair_sign_alg: data.keypair_sign_alg,
		public_key_sig: data.public_key_sig,
	};

	let (group_id, key_id) = group_model::create(
		app_id,
		user_id,
		create_data,
		group_type,
		parent_group_id,
		user_rank,
		connected_group,
		is_connected_group,
	)
	.await?;

	let key_vec = if let (Some(sign_key), Some(verify_key)) = (encrypted_sign_key_for_storage, verify_key_for_storage) {
		vec![
			KeyStorage {
				key: data.public_group_key,
				id: format!("pk_{key_id}"),
			},
			KeyStorage {
				key: data.encrypted_private_group_key,
				id: format!("sk_{key_id}"),
			},
			KeyStorage {
				key: verify_key,
				id: format!("vk_{key_id}"),
			},
			KeyStorage {
				key: sign_key,
				id: format!("sign_k_{key_id}"),
			},
		]
	} else {
		vec![
			KeyStorage {
				key: data.public_group_key,
				id: format!("pk_{key_id}"),
			},
			KeyStorage {
				key: data.encrypted_private_group_key,
				id: format!("sk_{key_id}"),
			},
		]
	};

	server_key_store::upload_key(key_vec).await?;

	Ok((group_id, key_id))
}

pub async fn delete_group(app_id: &str, group_id: &str, user_rank: i32) -> AppRes<()>
{
	let children = group_model::delete(app_id, group_id, user_rank).await?;

	//children incl. the deleted group
	server_api_common::file::delete_file_for_group(app_id, group_id, children).await?;

	let key_group = get_group_cache_key(app_id, group_id);
	cache::delete(key_group.as_str()).await?;

	Ok(())
}

pub async fn stop_invite(app_id: &str, group_id: &str, user_rank: i32) -> AppRes<()>
{
	group_model::stop_invite(app_id, group_id, user_rank).await?;

	let key_group = get_group_cache_key(app_id, group_id);
	cache::delete(key_group.as_str()).await?;

	Ok(())
}

pub fn get_user_group_light_data(group_data: &InternalGroupDataComplete) -> GroupLightServerData
{
	let (parent, access_by) = extract_parent_and_access_by(group_data);

	GroupLightServerData {
		group_id: group_data.group_data.id.to_string(),
		parent_group_id: parent,
		rank: group_data.user_data.rank,
		created_time: group_data.group_data.time,
		joined_time: group_data.user_data.joined_time,
		access_by,
		is_connected_group: group_data.group_data.is_connected_group,
	}
}

pub async fn get_user_group_data(group_data: &InternalGroupDataComplete) -> AppRes<GroupServerData>
{
	let app_id = &group_data.group_data.app_id;
	let group_id = &group_data.group_data.id;
	let user_id = &group_data.user_data.user_id;

	let (keys, hmac_keys, sortable_keys, key_update) = tokio::try_join!(
		get_user_group_keys(app_id, group_id, user_id, 0, "",),
		get_group_hmac(app_id, group_id, 0, "",),
		get_group_sortable(app_id, group_id, 0, ""),
		group_model::check_for_key_update(app_id, user_id, group_id)
	)?;

	let (parent, access_by) = extract_parent_and_access_by(group_data);

	Ok(GroupServerData {
		group_id: group_id.to_string(),
		parent_group_id: parent,
		keys,
		hmac_keys,
		sortable_keys,
		key_update,
		rank: group_data.user_data.rank,
		created_time: group_data.group_data.time,
		joined_time: group_data.user_data.joined_time,
		access_by,
		is_connected_group: group_data.group_data.is_connected_group,
	})
}

fn extract_parent_and_access_by(group_data: &InternalGroupDataComplete) -> (Option<String>, GroupUserAccessBy)
{
	let parent = match &group_data.group_data.parent {
		Some(p) => Some(p.to_string()),
		None => None,
	};

	//tell the frontend how thi group as access
	let access_by = match (
		&group_data.user_data.get_values_from_group_as_member,
		&group_data.user_data.get_values_from_parent,
	) {
		//the user is in a group which is member in a parent group
		(Some(v_as_member), Some(v_as_parent)) => {
			GroupUserAccessBy::GroupAsUserAsParent {
				group_as_user: v_as_member.to_string(),
				parent: v_as_parent.to_string(),
			}
		},
		(Some(v_as_member), None) => GroupUserAccessBy::GroupAsUser(v_as_member.to_string()),
		(None, Some(v_as_parent)) => GroupUserAccessBy::Parent(v_as_parent.to_string()),
		(None, None) => GroupUserAccessBy::User,
	};

	(parent, access_by)
}

pub async fn get_public_key_data(app_id: impl Into<AppId>, group_id: impl Into<GroupId>) -> AppRes<UserPublicKeyDataEntity>
{
	let mut out = group_model::get_public_key_data(app_id, group_id).await?;

	if out.public_key == "extern" {
		let mut fetched_key = server_key_store::get_keys(&[format!("pk_{}", out.public_key_id)]).await?;

		if let Some(fetched_key) = fetched_key.remove(&format!("pk_{}", out.public_key_id)) {
			out.public_key = fetched_key
		}
	}

	Ok(out)
}
