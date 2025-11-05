
//! A simple crate used for generating Authorization URLs with start.gg
//! Inspired (and basically translated) from 0xABADBABE-ops github repo "start.gg-oauth-full"
//! go star it! 
//! 
//! ## Usage and examples
//! 
//! ### Generating a Start.gg Oauth url with the default information
//! ```rust 
//!use startgg_oauth::{AuthUrl,ScopeBuilder};
//!
//!
//!// AuthUrl::default generates a url with a length of 64 as well 
//!let mut auth_url_builder = AuthUrl::new(64);
//!
//!let auth_url = auth_url_builder
//!     .set_client_id("my_client_id".to_string())
//!     .encode_redirect_uri_and_set("https://biggpiu.github.io".to_string())
//!     .generate_code_verifier_and_challenge()
//!     .build_url(&ScopeBuilder::default());
//!
//!// https://start.gg/oauth/authorize?scope=user.identity&response_type=code&client_id=my_client_id&redirect_uri=https%3A%2F%2Fbiggpiu.github.io&code_challenge=challenge&code_challenge_method=S256
//!println!("{}",auth_url);
//! ```
//! ### Generating a Start.gg Oauth Url with extra scopes
//! ```rust
//!use startgg_oauth::{AuthUrl,ScopeBuilder};
//!let mut auth_url_builder = AuthUrl::new(64);
//!let mut scope_builder = ScopeBuilder::new();
//!
//!let scopes = scope_builder
//!     .user_identity()
//!     .user_email()
//!     .tournament_manager()
//!     .tournament_reporter();
//!
//!let auth_url = auth_url_builder
//!     .set_client_id("my_client_id".to_string())
//!     .encode_redirect_uri_and_set("https://biggpiu.github.io".to_string())
//!     .generate_code_verifier_and_challenge()
//!     .build_url(&scopes);
//!
//!
//!println!("{}",auth_url);
//!
//! ```
//! ### Generate Start.gg Oauth Url and print PKCE code and challenge 
//! ```rust
//!use startgg_oauth::{AuthUrl,ScopeBuilder};
//!let mut auth_url_builder = AuthUrl::default();
//!
//!let auth_url = auth_url_builder
//!     .set_client_id("my_client_id".to_string())
//!     .encode_redirect_uri_and_set("https://biggpiu.github.io".to_string())
//!     .generate_code_verifier_and_challenge();
//!
//!
//!println!("PKCE code challenge: {}",auth_url.get_code_challenge());
//!println!("PKCE code verifier: {}",auth_url.get_code_verifier());
//!println!("url: {}",auth_url.build_url(&ScopeBuilder::default()));
//!
//! ```


mod pkce_implementation;
mod scope_builder;

pub use pkce_implementation::AuthUrl;
pub use scope_builder::ScopeBuilder;


#[cfg(test)]
mod tests {

    use super::*;
    use url::{Url};
    // use urlencoding::encode;


    #[test] 
    fn pkce_generates_right_length() {
        let len = 48;
        let mut x = pkce_implementation::AuthUrl::new(len);
        let y = x.generate_code_verifier_and_challenge();

        assert_eq!(y.get_code_verifier().len(),len);
    }

    #[test]
    fn cannot_exceed_pkce_length() {
        let len = 999;
        let mut x = pkce_implementation::AuthUrl::new(len);

        let y = x.generate_code_verifier_and_challenge();

        assert_ne!(y.get_code_verifier().len(),len);
    }

    #[test]
    fn link_has_all_parts() {
        let mut auth_url = AuthUrl::new(48);
        let redirect_uri = "https://biggpiu.github.io".to_string();
        // let encoded_redirect_uri = encode(&redirect_uri).into_owned();

        let auth = auth_url
            .encode_redirect_uri_and_set(redirect_uri)
            .set_client_id("fake".to_string())
            .generate_code_verifier_and_challenge()
            .build_url(&ScopeBuilder::default());


        let raw_url = Url::parse(&auth);

        if let Ok(deconstructed_url) = raw_url {
            assert_eq!(deconstructed_url.host_str(),Some("start.gg"));
            assert_eq!(deconstructed_url.path(),"/oauth/authorize");
            assert_eq!(deconstructed_url.query_pairs().count(), 6);

        }
        
    }
}