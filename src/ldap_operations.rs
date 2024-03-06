use ldap3::{LdapConn, Scope, SearchEntry};

#[derive(Debug, thiserror::Error)]
pub enum LdapError {
    #[error("LDAP connection error: {0}")]
    ConnectionError(String),
    #[error("Invalid base DN construction: {0}")]
    BaseDNConstructionError(String),
    #[error("LDAP bind failed: {0}")]
    BindError(String),
    #[error("LDAP search failed: {0}")]
    SearchError(String),
    #[error("LDAP entry parsing error: {0}")]
    EntryParsingError(String),
}

pub struct LdapClient {
    ldap: LdapConn,
    base_dn: String,
}

impl LdapClient {
    pub fn new(
        server: &str,
        username: &str,
        password: &str,
        domain: &str,
    ) -> Result<Self, LdapError> {
        let server = format!("ldap://{}", server);
        println!("connecting to{}", server);
        let mut ldap =
            LdapConn::new(&server).map_err(|e| LdapError::ConnectionError(e.to_string()))?;
        let base_dn = Self::construct_base_dn(domain)?;
        let user_dn = format!("CN={},CN=Users,{}", username, &base_dn);
        println!("{}", &user_dn);
        ldap.simple_bind(&user_dn, password)
            .map_err(|e| LdapError::BindError(e.to_string()))?
            .success()
            .map_err(|e| LdapError::BindError(e.to_string()))?;

        Ok(LdapClient { ldap, base_dn })
    }
    fn construct_base_dn(domain: &str) -> Result<String, LdapError> {
        let components: Vec<&str> = domain.split('.').collect();
        let mut base_dn = String::new();

        for (index, component) in components.iter().enumerate() {
            if index == 0 {
                base_dn.push_str(&format!("DC={}", component.to_uppercase()));
            } else {
                base_dn.push_str(&format!(",DC={}", component.to_uppercase()));
            }
        }

        Ok(base_dn)
    }

    pub fn search_users(&mut self, filter: &str) -> Result<Vec<SearchEntry>, LdapError> {
        let (result, _) = self
            .ldap
            .search(
                &self.base_dn,
                Scope::Subtree,
                filter,
                vec!["cn", "description"], // Attributes to retrieve
            )
            .map_err(|e| LdapError::SearchError(e.to_string()))?
            .success()
            .map_err(|e| LdapError::SearchError(e.to_string()))?;

        let mut search_entries = Vec::new();

        for entry in result {
            let final_result = SearchEntry::construct(entry);
            search_entries.push(final_result)
        }
        Ok(search_entries)

        // Implement methods for user enumeration, group membership retrieval, etc.
    }
}
