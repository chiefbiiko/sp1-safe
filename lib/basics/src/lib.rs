use serde::{Deserialize, Serialize};
use tiny_keccak::{Hasher, Keccak};

pub use primitive_types::H256;

pub const SAFE_SIGNED_MESSAGES_SLOT: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7,
];

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Inputs {
    pub safe_address: [u8; 20],      // Safe address
    pub msg_hash: [u8; 32],          // Custom msg hash
    pub state_root: [u8; 32],        // eth_getBlockBy*::response.stateRoot
    pub storage_root: [u8; 32],      // eth_getProof::response.storageHash
    pub state_trie_key: [u8; 32],    // keccak256(safe)
    pub storage_trie_key: [u8; 32],  // keccak256(msg_hash + uint256(7))
    pub account_proof: Vec<Vec<u8>>, // eth_getProof::response.accountProof
    pub storage_proof: Vec<Vec<u8>>, // eth_getProof::response.storageProof.proof
    pub header_rlp: Vec<u8>,         // RLP-encoded header
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sp1SafeParams {
    pub chain_id: u64,
    pub safe_address: String,
    pub message_hash: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sp1SafeResult {
    pub chain_id: u64,
    pub safe_address: String,
    pub message_hash: String,
    pub block_number: u64,
    pub block_hash: String,
    pub challenge: String,
    pub proof: String,
}

pub fn concat_bytes64(a: [u8; 32], b: [u8; 32]) -> [u8; 64] {
    // https://stackoverflow.com/a/76573243
    unsafe { core::mem::transmute::<[[u8; 32]; 2], [u8; 64]>([a, b]) }
}

pub fn lpad_bytes32(x: [u8; 20]) -> [u8; 32] {
    core::array::from_fn(|i| if i < 12 { 0u8 } else { x[i - 12] })
}

pub fn keccak256<T: AsRef<[u8]>>(input: T) -> [u8; 32] {
    let mut out = [0u8; 32];
    let mut k = Keccak::v256();
    k.update(input.as_ref());
    k.finalize(&mut out);
    out
}

//WIP FROM https://github.com/polytope-labs/hyperbridge/blob/v0.4.4/modules/trees/ethereum/src/lib.rs

extern crate alloc;

use core::marker::PhantomData;
use trie_db::TrieLayout;
// use primitive_types::H256;
use hash256_std_hasher::Hash256StdHasher;

mod node_codec;

/// Trie layout for EIP-1186 state proof nodes.
#[derive(Default, Clone)]
pub struct EIP1186Layout<H>(PhantomData<H>);

impl<H: hash_db::Hasher<Out = H256>> TrieLayout for EIP1186Layout<H> {
    const USE_EXTENSION: bool = true;
    const ALLOW_EMPTY: bool = false;
    const MAX_INLINE_VALUE: Option<u32> = None;
    type Hash = H;
    type Codec = node_codec::RlpNodeCodec<H>;
}

#[derive(Debug)]
pub struct KeccakHasher;

impl hash_db::Hasher for KeccakHasher {
    type Out = H256;
    type StdHasher = Hash256StdHasher;
    const LENGTH: usize = 32;

    fn hash(x: &[u8]) -> Self::Out {
        keccak256(x).into()
    }
}

//WIP FROM https://github.com/paritytech/trie/blob/master/trie-eip1186/src/eip1186.rs

// use crate::rstd::{result::Result, vec::Vec};
pub use alloc::vec;
pub use core::result;
pub trait Error {}
impl<T> Error for T {}

use hash_db::{HashDBRef, Hasher as HashDbHasher};
use trie_db::{
	node::{decode_hash, Node, NodeHandle, Value},
	recorder::Recorder,
	CError, DBValue, NibbleSlice, NodeCodec, Result as TrieResult, Trie, TrieDBBuilder, TrieHash,
	// TrieLayout,
};

/// Errors that may occur during proof verification. Most of the errors types simply indicate that
/// the proof is invalid with respect to the statement being verified, and the exact error type can
/// be used for debugging.
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum VerifyError<'a, HO, CE> {
	/// The proof does not contain any value for the given key
	/// the error carries the nibbles left after traversing the trie
	NonExistingValue(NibbleSlice<'a>),
	/// The proof contains a value for the given key
	/// while we were expecting to find a non-existence proof
	ExistingValue(Vec<u8>),
	/// The proof indicates that the trie contains a different value.
	/// the error carries the value contained in the trie
	ValueMismatch(Vec<u8>),
	/// The proof is missing trie nodes required to verify.
	IncompleteProof,
	/// The node hash computed from the proof is not matching.
	HashMismatch(HO),
	/// One of the proof nodes could not be decoded.
	DecodeError(CE),
	/// Error in converting a plain hash into a HO
	HashDecodeError(&'a [u8]),
}

/// Verify a compact proof for key-value pairs in a trie given a root hash.
pub fn verify_proof<'a, L>(
	root: &<L::Hash as HashDbHasher>::Out,
	proof: &'a [Vec<u8>],
	raw_key: &'a [u8],
	expected_value: Option<&[u8]>,
) -> Result<(), VerifyError<'a, TrieHash<L>, CError<L>>>
where
	L: TrieLayout,
{
	if proof.is_empty() {
		return Err(VerifyError::IncompleteProof)
	}
	let key = NibbleSlice::new(raw_key);
	process_node::<L>(Some(root), &proof[0], key, expected_value, &proof[1..])
}

fn process_node<'a, L>(
	expected_node_hash: Option<&<L::Hash as HashDbHasher>::Out>,
	encoded_node: &'a [u8],
	key: NibbleSlice<'a>,
	expected_value: Option<&[u8]>,
	proof: &'a [Vec<u8>],
) -> Result<(), VerifyError<'a, TrieHash<L>, CError<L>>>
where
	L: TrieLayout,
{
	if let Some(value) = expected_value {
		if encoded_node == value {
			return Ok(())
		}
	}
	if let Some(expected) = expected_node_hash {
		let calculated_node_hash = <L::Hash as HashDbHasher>::hash(encoded_node);
		if calculated_node_hash != *expected {
			return Err(VerifyError::HashMismatch(calculated_node_hash))
		}
	}
	let node = <L::Codec as NodeCodec>::decode(encoded_node).map_err(VerifyError::DecodeError)?;
	match node {
		Node::Empty => process_empty::<L>(key, expected_value, proof),
		Node::Leaf(nib, data) => process_leaf::<L>(nib, data, key, expected_value, proof),
		Node::Extension(nib, handle) =>
			process_extension::<L>(&nib, handle, key, expected_value, proof),
		Node::Branch(children, maybe_data) =>
			process_branch::<L>(children, maybe_data, key, expected_value, proof),
		Node::NibbledBranch(nib, children, maybe_data) =>
			process_nibbledbranch::<L>(nib, children, maybe_data, key, expected_value, proof),
	}
}

fn process_empty<'a, L>(
	key: NibbleSlice<'a>,
	expected_value: Option<&[u8]>,
	_: &[Vec<u8>],
) -> Result<(), VerifyError<'a, TrieHash<L>, CError<L>>>
where
	L: TrieLayout,
{
	if expected_value.is_none() {
		Ok(())
	} else {
		Err(VerifyError::NonExistingValue(key))
	}
}

fn process_leaf<'a, L>(
	nib: NibbleSlice,
	data: Value<'a>,
	key: NibbleSlice<'a>,
	expected_value: Option<&[u8]>,
	proof: &'a [Vec<u8>],
) -> Result<(), VerifyError<'a, TrieHash<L>, CError<L>>>
where
	L: TrieLayout,
{
	if key != nib && expected_value.is_none() {
		return Ok(())
	} else if key != nib {
		return Err(VerifyError::NonExistingValue(key))
	}
	match_value::<L>(Some(data), key, expected_value, proof)
}
fn process_extension<'a, L>(
	nib: &NibbleSlice,
	handle: NodeHandle<'a>,
	mut key: NibbleSlice<'a>,
	expected_value: Option<&[u8]>,
	proof: &'a [Vec<u8>],
) -> Result<(), VerifyError<'a, TrieHash<L>, CError<L>>>
where
	L: TrieLayout,
{
	if !key.starts_with(nib) && expected_value.is_none() {
		return Ok(())
	} else if !key.starts_with(nib) {
		return Err(VerifyError::NonExistingValue(key))
	}
	key.advance(nib.len());

	match handle {
		NodeHandle::Inline(encoded_node) =>
			process_node::<L>(None, encoded_node, key, expected_value, proof),
		NodeHandle::Hash(plain_hash) => {
			let new_root = decode_hash::<L::Hash>(plain_hash)
				.ok_or(VerifyError::HashDecodeError(plain_hash))?;
			process_node::<L>(Some(&new_root), &proof[0], key, expected_value, &proof[1..])
		},
	}
}

fn process_nibbledbranch<'a, L>(
	nib: NibbleSlice,
	children: [Option<NodeHandle<'a>>; 16],
	maybe_data: Option<Value<'a>>,
	mut key: NibbleSlice<'a>,
	expected_value: Option<&[u8]>,
	proof: &'a [Vec<u8>],
) -> Result<(), VerifyError<'a, TrieHash<L>, CError<L>>>
where
	L: TrieLayout,
{
	if !key.starts_with(&nib) && expected_value.is_none() {
		return Ok(())
	} else if !key.starts_with(&nib) && expected_value.is_some() {
		return Err(VerifyError::NonExistingValue(key))
	}
	key.advance(nib.len());

	if key.is_empty() {
		match_value::<L>(maybe_data, key, expected_value, proof)
	} else {
		match_children::<L>(children, key, expected_value, proof)
	}
}

fn process_branch<'a, L>(
	children: [Option<NodeHandle<'a>>; 16],
	maybe_data: Option<Value<'a>>,
	key: NibbleSlice<'a>,
	expected_value: Option<&[u8]>,
	proof: &'a [Vec<u8>],
) -> Result<(), VerifyError<'a, TrieHash<L>, CError<L>>>
where
	L: TrieLayout,
{
	if key.is_empty() {
		match_value::<L>(maybe_data, key, expected_value, proof)
	} else {
		match_children::<L>(children, key, expected_value, proof)
	}
}
fn match_children<'a, L>(
	children: [Option<NodeHandle<'a>>; 16],
	mut key: NibbleSlice<'a>,
	expected_value: Option<&[u8]>,
	proof: &'a [Vec<u8>],
) -> Result<(), VerifyError<'a, TrieHash<L>, CError<L>>>
where
	L: TrieLayout,
{
	match children.get(key.at(0) as usize) {
		Some(Some(NodeHandle::Hash(hash))) =>
			if proof.is_empty() {
				Err(VerifyError::IncompleteProof)
			} else {
				key.advance(1);
				let new_root =
					decode_hash::<L::Hash>(hash).ok_or(VerifyError::HashDecodeError(hash))?;
				process_node::<L>(Some(&new_root), &proof[0], key, expected_value, &proof[1..])
			},
		Some(Some(NodeHandle::Inline(encoded_node))) => {
			key.advance(1);
			process_node::<L>(None, encoded_node, key, expected_value, proof)
		},
		Some(None) =>
			if expected_value.is_none() {
				Ok(())
			} else {
				Err(VerifyError::NonExistingValue(key))
			},
		None => panic!("key index is out of range in children array"),
	}
}

fn match_value<'a, L>(
	maybe_data: Option<Value<'a>>,
	key: NibbleSlice<'a>,
	expected_value: Option<&[u8]>,
	proof: &'a [Vec<u8>],
) -> Result<(), VerifyError<'a, TrieHash<L>, CError<L>>>
where
	L: TrieLayout,
{
	match (maybe_data, proof.first(), expected_value) {
		(None, _, None) => Ok(()),
		(None, _, Some(_)) => Err(VerifyError::NonExistingValue(key)),
		(Some(Value::Inline(inline_data)), _, Some(value)) =>
			if inline_data == value {
				Ok(())
			} else {
				Err(VerifyError::ValueMismatch(inline_data.to_vec()))
			},
		(Some(Value::Inline(inline_data)), _, None) =>
			Err(VerifyError::ExistingValue(inline_data.to_vec())),
		(Some(Value::Node(plain_hash)), Some(next_proof_item), Some(value)) => {
			let value_hash = L::Hash::hash(value);
			let node_hash = decode_hash::<L::Hash>(plain_hash)
				.ok_or(VerifyError::HashDecodeError(plain_hash))?;
			if node_hash != value_hash {
				Err(VerifyError::HashMismatch(node_hash))
			} else if next_proof_item != value {
				Err(VerifyError::ValueMismatch(next_proof_item.to_vec()))
			} else {
				Ok(())
			}
		},
		(Some(Value::Node(_)), None, _) => Err(VerifyError::IncompleteProof),
		(Some(Value::Node(_)), Some(proof_item), None) =>
			Err(VerifyError::ExistingValue(proof_item.to_vec())),
	}
}