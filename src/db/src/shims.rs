use realworld_domain::{Profile, User};

impl From<crate::entity::users::Model> for User {
    fn from(user: crate::entity::users::Model) -> Self {
        User {
            id: user.id,
            email: user.email,
            profile: Profile {
                username: user.username,
                bio: user.bio,
                image: user.image,
            },
        }
    }
}

pub fn to_article(
    a: crate::entity::articles::Model,
    u: realworld_domain::User,
    n_fav: u64,
) -> realworld_domain::Article {
    let metadata = realworld_domain::ArticleMetadata {
        created_at: a.created_at.into(),
        updated_at: a.updated_at.into(),
    };
    let content = realworld_domain::ArticleContent {
        title: a.title,
        description: a.description,
        body: a.body,
        // TODO Not supported now
        // tag_list: a.tag_list,
        tag_list: vec![],
    };
    realworld_domain::Article {
        content,
        slug: a.slug,
        author: u.profile,
        metadata,
        favorites_count: n_fav,
    }
}
impl From<crate::entity::users::Model> for Profile {
    fn from(u: crate::entity::users::Model) -> Self {
        Profile {
            username: u.username,
            bio: u.bio,
            image: u.image,
        }
    }
}
pub fn to_comment(c: crate::entity::comments::Model, u: User) -> realworld_domain::Comment {
    realworld_domain::Comment {
        id: c.id as u64,
        author: u.profile,
        body: c.body,
        created_at: c.created_at.into(),
        updated_at: c.updated_at.into(),
    }
}
