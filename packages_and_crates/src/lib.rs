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
