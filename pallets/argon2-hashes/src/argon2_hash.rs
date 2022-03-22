
use argon2::{self, Config};
use std::hash::{Hash, Hasher};
use fasthash::MetroHasher;
use std::time::{SystemTime};


fn hash_func_id<T: Hash>(t: &T) -> u64 {
	let mut s: MetroHasher = Default::default();
	t.hash(&mut s);
	s.finish()
}


pub fn hash_pwd(pass: &str, salt: &str) -> (u64, String) {
	let config = Config::default();

	let hash = argon2::hash_encoded(pass.as_bytes(), salt.as_bytes(), &config).unwrap();

	let now = SystemTime::now();

	let id_hash = hash_func_id(&format!("{} {} {:?}", pass, hash, now));

	return (id_hash, hash)


}

pub fn verify_pass(hash: String, password: &str) -> bool {
	return argon2::verify_encoded(&hash, password.as_bytes()).unwrap()
}

