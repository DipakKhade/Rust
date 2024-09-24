pub mod LoginMod {
    pub struct UserCrentional {
        pub email: String,
        pub password: String,
    }

    pub enum LoginState {
        success,
        failure,
    }

    pub fn login(cred: UserCrentional) -> LoginState {
        print!("control is here");
        LoginState::success
    }
}

pub mod SignInMod {
    use crate::LoginMod;

    pub fn sign_in(cred: LoginMod::UserCrentional) -> LoginMod::LoginState {
        LoginMod::LoginState::success
    }
}

pub mod db;
pub mod models;
pub mod parent;
pub mod utils;
