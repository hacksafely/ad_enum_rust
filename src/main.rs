// src/main.rs
mod ldap_operations;
use prettytable::*;

use ldap_operations::{LdapClient, LdapError};
use std::env;

fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 5 {
        eprintln!(
            "Usage: {} <username> <password> <domain_controller_ip> <domain_name>",
            args[0]
        );
        std::process::exit(1);
    }

    let username = &args[1];
    let password = &args[2];
    let server = &args[3];
    let domain = &args[4];

    // Connect to LDAP server
    let mut ldap_client = match LdapClient::new(server, username, password, domain) {
        Ok(ldap_client) => {
            // Use ldap_client to perform LDAP operations
            println!("Connected to LDAP server: {}", server);
            ldap_client
        }
        Err(err) => {
            eprintln!("Error connecting to LDAP server: {}", err);
            std::process::exit(1);
        }
    };

    // Perform LDAP search
    let filter = "(objectClass=user)"; // Define your LDAP filter here

    match ldap_client.search_users(filter) {
        Ok(users) => {
            println!("Users found:");

            // Create a table for nice formatting
            let mut table = Table::new();
            table.add_row(row!["User", "Description"]);

            for entry in users {
                // Get CN (or handle its absence)
                let cn_value = entry
                    .attrs
                    .get("cn")
                    .map(|val| val.join(", "))
                    .unwrap_or_else(|| "CN not found in entry".to_string());

                // Get Description (or handle its absence)
                let desc_value = entry
                    .attrs
                    .get("description")
                    .map(|val| val.join(", "))
                    .unwrap_or_else(|| "Description not found in entry".to_string());

                // Add a row to the table
                table.add_row(row![cn_value, desc_value]);
            }

            // Print the table
            table.printstd();
        }
        Err(err) => {
            eprintln!("Error searching users: {}", err);
            std::process::exit(1);
        }
    }
}
