

pub struct ScopeBuilder {
    scopes:Vec<&'static str>
}


impl Default for ScopeBuilder {
    /// gets the users user.identity by default
    fn default() -> Self {
        Self { scopes: vec!["user.identity"] }
    }
}


impl ScopeBuilder {
    
    pub fn new() -> Self {
        return ScopeBuilder { scopes: vec![] }
    }

    pub fn user_identity(&mut self) -> &mut ScopeBuilder{
        self.scopes.push("user.identity");
        self
    }

    pub fn user_email(&mut self) -> &mut ScopeBuilder {
        self.scopes.push("user.email");
        self
    }

    pub fn tournament_manager(&mut self) -> &mut ScopeBuilder {
        self.scopes.push("tournament.manager");
        self
    }

    pub fn tournament_reporter(&mut self) -> &mut ScopeBuilder {
        self.scopes.push("tournament.reporter");
        self
    }

    pub fn build(&self) -> String {
        self.scopes.join(" ")
    } 
    
}
