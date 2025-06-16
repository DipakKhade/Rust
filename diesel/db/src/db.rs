use crate::config::Config;
use diesel::prelude::*;

use crate::config;

pub struct Db {
    connection: Connection
}

impl Db {
    pub fn new() -> Result<Self, ConnectionError> {
        let config_url = Config::default();
        let connection = MysqlConnection::establish(&config_url);

        Ok(Self {
            connection
        })

    }
}