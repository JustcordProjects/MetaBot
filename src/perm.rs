use serenity::all::{Message, User, RoleId};
use crate::config::Config;

pub fn is_allowed_user(cfg: &Config, user: &User) -> bool {
    cfg.permissions.users.contains(&user.id)
}

pub fn has_allowed_role(cfg: &Config, roles: &[RoleId]) -> bool {
    for role in &cfg.permissions.roles {
        if roles.contains(&role) {
            return true;
        }
    }

    return false;
}

pub fn can_execute(cfg: &Config, msg: &Message) -> bool {
    if is_allowed_user(cfg, &msg.author) {
        return true;
    }

    if let Some(member) = &msg.member {
        has_allowed_role(cfg, &member.roles)
    } else {
        false
    }
}
