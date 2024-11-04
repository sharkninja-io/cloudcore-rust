use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfile {
    uuid: String,
    username: String,
    email: String,
    firstname: String,
    lastname: String,
    approved: bool,
    confirmed: bool,
    city: Option<String>,
    company: Option<String>,
    confirmed_at: Option<String>,
    country: Option<String>,
    created_at: Option<String>,
    phone_country_code: Option<String>,
    phone: Option<String>,
    primary_contact: String,
    state: Option<String>,
    street: Option<String>,
    updated_at: Option<String>,
    zip: Option<String>,
    dealers: Option<Vec<String>>,
}

impl UserProfile {
    pub fn new(
        uuid: String,
        username: String,
        email: String,
        firstname: String,
        lastname: String,
        approved: bool,
        confirmed: bool,
        city: Option<String>,
        company: Option<String>,
        confirmed_at: Option<String>,
        country: Option<String>,
        created_at: Option<String>,
        phone_country_code: Option<String>,
        phone: Option<String>,
        primary_contact: String,
        state: Option<String>,
        street: Option<String>,
        updated_at: Option<String>,
        zip: Option<String>,
        dealers: Option<Vec<String>>,
    ) -> Self {
        Self {
            uuid,
            username,
            email,
            firstname,
            lastname,
            approved,
            confirmed,
            city,
            company,
            confirmed_at,
            country,
            created_at,
            phone_country_code,
            phone,
            primary_contact,
            state,
            street,
            updated_at,
            zip,
            dealers,
        }
    }

    /// Get a reference to the user profile's uuid.
    pub fn uuid(&self) -> &str {
        self.uuid.as_ref()
    }

    /// Get a reference to the user profile's username.
    pub fn username(&self) -> &str {
        self.username.as_ref()
    }

    /// Get a reference to the user profile's email.
    pub fn email(&self) -> &str {
        self.email.as_ref()
    }

    /// Set the user profile's email.
    pub fn set_email(&mut self, email: String) {
        self.email = email;
    }

    /// Get a reference to the user profile's firstname.
    pub fn firstname(&self) -> &str {
        self.firstname.as_ref()
    }

    /// Set the user profile's firstname.
    pub fn set_firstname(&mut self, firstname: String) {
        self.firstname = firstname;
    }

    /// Get a reference to the user profile's lastname.
    pub fn lastname(&self) -> &str {
        self.lastname.as_ref()
    }

    /// Set the user profile's lastname.
    pub fn set_lastname(&mut self, lastname: String) {
        self.lastname = lastname;
    }

    /// Get a reference to the user profile's approved.
    pub fn approved(&self) -> bool {
        self.approved
    }

    /// Get a reference to the user profile's confirmed.
    pub fn confirmed(&self) -> bool {
        self.confirmed
    }
}
