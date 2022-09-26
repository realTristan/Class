// Library Usages
use std::collections::HashMap;
use std::sync::Mutex;

// The SUPER_SECRET_CODE is what's used to prevent
// users trying to abuse the api from being able
// to generate their own auth tokens
static SUPER_SECRET_CODE: &str = "super_secret_code";

// The TOKEN_STORAGE is used to store previously used
// tokens so that abusers can't access the api using
// a previous token.
lazy_static::lazy_static! {
    static ref TOKEN_STORAGE: Mutex<HashMap<String, Vec<String>>> = {
        Mutex::new(HashMap::new())
    };    
}

// BEARER TOKEN IS SHA256 ENCODE [ (user_hash):(provided auth token):(firebase_token) ]
// The verify_bearer() function is used to verify whether
// the provided bearer token is valid. If the bearer is valid
// then we can proceed with whatever 'secure' function it is we need to do
pub fn verify_bearer(
    user_hash: &str, access_token: &str, bearer_token: &str, firebase_token: &str
) -> bool {
    // Generate a new bearer token format using the provided
    // data which will be compared to the provided bearer
    let gen: String = format!("{}:{}:{}", user_hash, access_token, firebase_token);
    // SHA256 Encode the generated format above
    let gen_bearer: String = format!("Bearer {}", sha256::digest(gen));
    // Return whether the provided bearer token is
    // equal to the generated one
    return bearer_token.to_string() == gen_bearer
}

// The verify() function is used to check whether the
// provided auth token is valid. It does this by
// checking whether the token has been created within
// the past 8 seconds. If so, return true, else, return false.
pub fn verify(user_hash: &str, access_token: &str) -> bool {
    // Get the system time since epoch. This value
    // is used to check how long ago the auth token was
    // generated. Doing this prevents users from consecutively
    // using a single auth token if trying to abuse the api
    let time: std::time::Duration = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH).unwrap();
    // Convert the time to milliseconds
    let time: u64 = time.as_secs();

    // Execute the storage handler
    // If the function returns false, then the provided
    // auth token has already been used within the past 8 seconds.
    if !storage_handler(user_hash, access_token, &time) { return false };

    // Check whether the auth token was generated
    // within the past 10 seconds
    for i in 0..8 {
        let gen: String = format!("{}:{}:{}", user_hash, time-i, SUPER_SECRET_CODE);
        // If the provided auth token is equal to the
        // generated auth token, return true
        if access_token == sha256::digest(gen) {
            return true;
        }
    }
    return false;
}

// The storage_handler() function is used to check whether
// the provided auth token has already been used
// within the past 8 seconds. This is function is
// necessary to prevent abusers from using the same
// token more than once.
fn storage_handler(user_hash: &str, access_token: &str, time: &u64) -> bool {
    let mut token_storage = TOKEN_STORAGE.lock().unwrap();

    // Convert the token storage into a mutable variable.
    // This is required so that we can append the access_token
    // to the users token storage, or so that we can clear
    // the token storage if full.
    let mut_storage: Option<&mut Vec<String>> = token_storage.get_mut(user_hash);

    // If the user doesn't already exist within the
    // token storage.. Insert a new key:value that
    // contains the users hash and the array containing the
    // current time (which will be used to determine the last wipe)
    // and the provided auth token.
    if mut_storage.is_none() {
        // Insert the user into the token storage
        // along with the current time and auth token
        token_storage.insert(
            user_hash.to_string(), 
            [time.to_string(), access_token.to_string()].to_vec()
        );
        // Return true as the token did not
        // previously exist in the token storage
        return true;
    }
    // Unwrap the previous mutable storage
    let mut_storage: &mut Vec<String> = mut_storage.unwrap();

    // Get the last storage wipe time
    let last_wipe_time: u64 = mut_storage[0].parse().unwrap();

    // If the last wipe happened over 8 seconds ago,
    // wipe the users token storage to prevent an
    // overflow. If the user has too many tokens and
    // the cache isn't eventually cleared.. you already
    // know what'll happen lmao.
    if time > &(last_wipe_time+8) {
        mut_storage.clear();
        mut_storage[0] = time.to_string();
    }
    
    // After the users current token storage has or hasn't been
    // cleared, check whether the access_token is already existant
    // in the token storage. If it is, return false, thus the
    // user is using an unauthorized token. Else, append the
    // token to the user's token storage and return true.
    if !mut_storage.contains(&access_token.to_string()) {
        mut_storage.push(access_token.to_string());
        return true;
    }
    return false;
}
