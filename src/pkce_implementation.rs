use std::cmp::min;

// use crate::structs::StartGGAuthUrl;
use crate::scope_builder::ScopeBuilder;
use url_builder::{self, URLBuilder};


pub struct AuthUrl {
    code_length:usize,
    client_id:String,
    redirect_uri:String,
    code_verifier:String,
    code_challenge:String,

}

impl Default for AuthUrl {
    fn default() -> Self {
        Self {
            code_length: 64_usize,
            client_id:String::from(""),
            redirect_uri: String::from(""),
            code_verifier: String::from(""),
            code_challenge: String::from(""),
        }
    }
}



impl AuthUrl {
    
    pub fn new(code_length:usize) -> Self {
        return AuthUrl {
            code_length:min(128, code_length),
            client_id: String::from(""),
            redirect_uri: String::from(""), 
            code_verifier: String::from(""),
            code_challenge: String::from(""),
        }
    }

    pub fn set_client_id(&mut self, client_id:String) -> &mut AuthUrl {
        // I think the funniest part about this is that I could 
        // set it to an u8 and nobody would ever notice
        self.client_id = client_id;
        self
    }

    /// Set the redirect URI without encoding it 
    pub fn set_redirect_uri(&mut self, redirect_uri:String) -> &mut AuthUrl {
        self.redirect_uri = redirect_uri;
        self
    }

    ///Encode the redirect URI and set it as the redirect URI
    pub fn encode_redirect_uri_and_set(&mut self, redirect_uri:String) -> &mut AuthUrl {
        self.redirect_uri = urlencoding::encode(&redirect_uri).into();

        self
    }

    pub fn generate_code_verifier_and_challenge(&mut self) -> &mut AuthUrl {
        // if this goes wrong shit is not going right
        let unencrypted_code_verifier = pkce::code_verifier(self.code_length);
        self.code_verifier = String::from_utf8(unencrypted_code_verifier.clone()).unwrap();
        self.code_challenge = pkce::code_challenge(&unencrypted_code_verifier);

        self
    }

    pub fn build_url(&self ,scope_builder:&ScopeBuilder) -> String {
        let mut url = URLBuilder::new();

        url
        .set_protocol("https")
        .set_host("start.gg")
        .add_route("oauth")
        .add_route("authorize")
        .add_param("response_type", "code")
        .add_param("client_id", &self.client_id)
        .add_param("scope", &scope_builder.build())
        .add_param("redirect_uri", &self.redirect_uri)
        .add_param("code_challenge", &self.code_challenge)
        .add_param("code_challenge_method", "S256");

        return url.build();
    }
    
    pub fn get_code_verifier(&self) -> String {
        return self.code_verifier.clone();
    }

    pub fn get_code_challenge(&self) -> String {
        return self.code_challenge.clone();
    }
}


