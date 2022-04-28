use std::collections::HashMap;
use std::collections::HashSet;

use crate::json::Json;
use crate::shims::to_article;
use crate::shims::to_comment;
use chrono::Utc;
use realworld_domain::{Article, FavoriteOutcome};
use sea_orm::sea_query::Expr;
use sea_orm::DbBackend;
use sea_orm::DeleteResult;
use sea_orm::JoinType;
use sea_orm::Statement;

use sea_orm::FromQueryResult;
use sea_orm::ModelTrait;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Database, DatabaseConnection, EntityTrait,
    QueryFilter, QuerySelect,
};
use uuid::Uuid;

#[derive(Clone)]
pub struct Repository(DatabaseConnection);

impl Repository {
    pub async fn create(url: String) -> anyhow::Result<Repository> {
        let pool = Database::connect(url).await?;
        Ok(Repository(pool))
    }
    pub fn with_connection(pool: DatabaseConnection) -> Repository {
        Repository(pool)
    }

    pub fn pool(&self) -> DatabaseConnection {
        self.0.clone()
    }
}

#[async_trait::async_trait]
impl realworld_domain::repositories::Repository for Repository {
    async fn publish_article(
        &self,
        draft: realworld_domain::ArticleContent,
        author: &realworld_domain::User,
    ) -> Result<realworld_domain::Article, realworld_domain::PublishArticleError> {
        use crate::entity::articles;

        let article = articles::ActiveModel {
            title: ActiveValue::Set(draft.title.clone()),
            body: ActiveValue::Set(draft.body.clone()),
            description: ActiveValue::Set(draft.description.clone()),
            created_at: ActiveValue::Set(Utc::now().into()),
            updated_at: ActiveValue::Set(Utc::now().into()),
            user_id: ActiveValue::Set(author.id),
            slug: ActiveValue::Set(draft.slug()),
            tag_list: ActiveValue::Set(Json(draft.tag_list)),
        };
        article
            .insert(&self.0)
            .await
            .map(|article| to_article(article, author.clone(), 0))
            .map_err(to_db_error)
            .map(Ok)?
    }

    async fn get_article_by_slug(
        &self,
        slug: &str,
    ) -> Result<realworld_domain::Article, realworld_domain::GetArticleError> {
        use crate::entity::articles::Entity as Article;
        use crate::entity::users;
        let mut article = Article::find_by_id(slug.to_string())
            .find_also_related(users::Entity)
            .one(&self.0)
            .await
            .map_err(to_db_error)?
            .and_then(|(article, user)| user.map(|u| to_article(article, u.into(), 0)))
            .ok_or_else(|| {
                realworld_domain::DatabaseError::from(anyhow::anyhow!("Article not found"))
            })?;

        let n_fav = self.n_favorites(&article).await?;
        article.favorites_count = n_fav as u64;

        Ok(article)
    }

    async fn get_article_view(
        &self,
        viewer: &realworld_domain::User,
        article: realworld_domain::Article,
    ) -> Result<realworld_domain::ArticleView, realworld_domain::GetArticleError> {
        let author_view = self
            .get_profile_view(viewer, &article.author.username)
            .await?;
        let is_favorite = self.is_favorite(&article, viewer).await?;
        let article_view = realworld_domain::ArticleView {
            content: article.content,
            slug: article.slug,
            author: author_view,
            metadata: article.metadata,
            favorited: is_favorite,
            favorites_count: article.favorites_count,
            viewer: viewer.id,
        };
        Ok(article_view)
    }

    async fn get_articles_views(
        &self,
        viewer: &realworld_domain::User,
        articles: Vec<realworld_domain::Article>,
    ) -> Result<Vec<realworld_domain::ArticleView>, realworld_domain::DatabaseError> {
        let are_favorites = self.are_favorites(&articles, viewer).await?;
        let mut articles_view = Vec::with_capacity(are_favorites.len());
        for article in articles.iter() {
            let author_view = self
                .get_profile_view(viewer, &article.author.username)
                .await?;

            let favorited = are_favorites
                .get(article.slug.as_str())
                .cloned()
                .unwrap_or_default();
            let article_view = realworld_domain::ArticleView {
                content: article.content.clone(),
                slug: article.slug.clone(),
                author: author_view,
                metadata: article.metadata.clone(),
                favorited,
                favorites_count: article.favorites_count,
                viewer: viewer.id,
            };
            articles_view.push(article_view);
        }
        Ok(articles_view)
    }

    async fn find_articles(
        &self,
        query: realworld_domain::ArticleQuery,
    ) -> Result<Vec<realworld_domain::Article>, realworld_domain::DatabaseError> {
        use crate::entity::articles::Entity as Article;
        use crate::entity::users;

        let mut q = Article::find().find_also_related(users::Entity);

        if let Some(username) = query.author {
            q = q.filter(users::Column::Username.eq(username));
        }

        if let Some(tag) = query.tag {
            q = q.filter(Expr::cust(&format!("tag_list @> '\"{}\"'", tag)));
        }

        let mut articles: Vec<realworld_domain::Article> = q
            .all(&self.0)
            .await
            .map_err(to_db_error)?
            .into_iter()
            .filter_map(|(article, user)| user.map(|u| (article, realworld_domain::User::from(u))))
            .map(|(article, user)| to_article(article, user, 0))
            .collect();

        // TODO Optimize
        for article in articles.iter_mut() {
            let n_fav = self.n_favorites(article).await?;
            article.favorites_count = n_fav as u64;
        }
        Ok(articles)
    }

    async fn feed(
        &self,
        user: &realworld_domain::User,
        _query: realworld_domain::FeedQuery,
    ) -> Result<Vec<realworld_domain::ArticleView>, realworld_domain::DatabaseError> {
        use crate::entity::articles::Entity as Articles;
        use crate::entity::followers;
        use crate::entity::users;
        use sea_orm::RelationTrait;

        let mut articles: Vec<realworld_domain::Article> = Articles::find()
            .find_also_related(users::Entity)
            .join_rev(JoinType::InnerJoin, followers::Relation::Users1.def())
            .filter(followers::Column::FollowerId.eq(user.id))
            .all(&self.0)
            .await
            .map_err(to_db_error)?
            .into_iter()
            .filter_map(|(article, user)| user.map(|u| (article, realworld_domain::User::from(u))))
            .map(|(article, user)| to_article(article, user, 0))
            .collect();

        for article in articles.iter_mut() {
            let n_fav = self.n_favorites(article).await?;
            article.favorites_count = n_fav as u64;
        }

        self.get_articles_views(user, articles).await
    }

    async fn delete_article(
        &self,
        article: &realworld_domain::Article,
    ) -> Result<(), realworld_domain::DatabaseError> {
        use crate::entity::articles::Entity as Article;

        let article = Article::find_by_id(article.slug.clone())
            .one(&self.0)
            .await
            .map_err(to_db_error)?
            .ok_or_else(|| {
                realworld_domain::DatabaseError::from(anyhow::anyhow!("Article not found"))
            })?;

        article.delete(&self.0).await.map_err(to_db_error)?;

        Ok(())
    }

    async fn comment_article(
        &self,
        user: &realworld_domain::User,
        article: &realworld_domain::Article,
        comment: realworld_domain::CommentContent,
    ) -> Result<realworld_domain::Comment, realworld_domain::DatabaseError> {
        use crate::entity::comments::ActiveModel;
        let new_comment = ActiveModel {
            body: ActiveValue::Set(comment.0),
            article_id: ActiveValue::Set(article.slug.clone()),
            author_id: ActiveValue::Set(user.id),
            created_at: ActiveValue::Set(Utc::now().into()),
            updated_at: ActiveValue::Set(Utc::now().into()),
            ..Default::default()
        };

        let raw_comment = new_comment.insert(&self.0).await.map_err(to_db_error)?;
        let comment = realworld_domain::Comment {
            id: raw_comment.id as u64,
            author: user.profile.clone(),
            body: raw_comment.body,
            created_at: raw_comment.created_at.into(),
            updated_at: raw_comment.updated_at.into(),
        };
        Ok(comment)
    }

    async fn get_comment(
        &self,
        comment_id: u64,
    ) -> Result<realworld_domain::Comment, realworld_domain::DeleteCommentError> {
        use crate::entity::comments::Entity as Comments;

        let comment = Comments::find_by_id(comment_id as i64)
            .one(&self.0)
            .await
            .map_err(to_db_error)?
            .ok_or_else(|| realworld_domain::DeleteCommentError::CommentNotFound {
                comment_id,
                source: anyhow::anyhow!("Comment not found").into(),
            })?;

        let user = self.get_user_by_id(comment.author_id).await?;

        Ok(to_comment(comment, user))
    }

    async fn get_comments(
        &self,
        article: &realworld_domain::Article,
    ) -> Result<Vec<realworld_domain::Comment>, realworld_domain::DatabaseError> {
        use crate::entity::comments::{self, Entity as Comments};
        use crate::entity::users;

        Ok(Comments::find()
            .filter(comments::Column::ArticleId.eq(article.slug.clone()))
            .find_also_related(users::Entity)
            .all(&self.0)
            .await
            .map_err(to_db_error)?
            .into_iter()
            .filter_map(|(comment, user)| user.map(|u| to_comment(comment, u.into())))
            .collect())
    }

    async fn delete_comment(
        &self,
        comment_id: u64,
    ) -> Result<(), realworld_domain::DeleteCommentError> {
        use crate::entity::comments::Entity as Comments;
        Comments::delete_by_id(comment_id as i64)
            .exec(&self.0)
            .await
            .map_err(to_db_error)?;
        Ok(())
    }

    async fn update_article(
        &self,
        article: realworld_domain::Article,
        update: realworld_domain::ArticleUpdate,
    ) -> Result<realworld_domain::Article, realworld_domain::DatabaseError> {
        use crate::entity::articles::Entity as Article;
        use sea_orm::IntoActiveModel;
        let slug = article.slug.clone();

        let article = Article::find_by_id(slug.clone())
            .one(&self.0)
            .await
            .map_err(to_db_error)?
            .ok_or_else(|| {
                realworld_domain::DatabaseError::from(anyhow::anyhow!("Article not found"))
            })?;

        let mut model = article.into_active_model();

        if let Some(title) = update.title {
            model.title = ActiveValue::Set(title);
        }
        if let Some(description) = update.description {
            model.description = ActiveValue::Set(description);
        }
        if let Some(body) = update.body {
            model.body = ActiveValue::Set(body);
        }

        model.update(&self.0).await.map_err(to_db_error)?;

        let article = self.get_article_by_slug(&slug).await?;

        Ok(article)
    }

    async fn favorite(
        &self,
        article: &realworld_domain::Article,
        user: &realworld_domain::User,
    ) -> Result<realworld_domain::FavoriteOutcome, realworld_domain::DatabaseError> {
        use crate::entity::favorites::{self, ActiveModel, Entity as Favorites};
        let favorite = Favorites::find()
            .filter(favorites::Column::ArticleId.eq(article.slug.clone()))
            .filter(favorites::Column::UserId.eq(user.id))
            .one(&self.0)
            .await
            .map_err(to_db_error)?;

        match favorite {
            Some(_) => Ok(FavoriteOutcome::AlreadyAFavorite),
            None => {
                let model = ActiveModel {
                    article_id: ActiveValue::Set(article.slug.clone()),
                    user_id: ActiveValue::Set(user.id),
                };

                model.insert(&self.0).await.map_err(to_db_error)?;
                Ok(FavoriteOutcome::NewFavorite)
            }
        }
    }

    async fn unfavorite(
        &self,
        article: &realworld_domain::Article,
        user: &realworld_domain::User,
    ) -> Result<realworld_domain::UnfavoriteOutcome, realworld_domain::DatabaseError> {
        use crate::entity::favorites::{self, Entity as Favorites};
        let result: DeleteResult = Favorites::delete_many()
            .filter(favorites::Column::ArticleId.eq(article.slug.clone()))
            .filter(favorites::Column::UserId.eq(user.id))
            .exec(&self.0)
            .await
            .map_err(to_db_error)?;

        if result.rows_affected > 0 {
            Ok(realworld_domain::UnfavoriteOutcome::WasAFavorite)
        } else {
            Ok(realworld_domain::UnfavoriteOutcome::WasNotAFavorite)
        }
    }

    async fn sign_up(
        &self,
        sign_up: realworld_domain::SignUp,
    ) -> Result<realworld_domain::User, realworld_domain::SignUpError> {
        use crate::entity::users;
        let user = users::ActiveModel {
            id: ActiveValue::Set(Uuid::new_v4()),
            username: ActiveValue::Set(sign_up.username),
            email: ActiveValue::Set(sign_up.email),
            password: ActiveValue::Set(sign_up.password.hash().to_string()),
            created_at: ActiveValue::Set(Utc::now().into()),
            updated_at: ActiveValue::Set(Utc::now().into()),
            ..Default::default()
        };

        let user = user
            .insert(&self.0)
            .await
            .map(Into::into)
            .map_err(to_db_error)?;

        self.follow(&user, &user.profile).await?;
        Ok(user)
    }

    async fn update_user(
        &self,
        user: realworld_domain::User,
        realworld_domain::UserUpdate {
            email,
            username,
            password,
            image,
            bio,
        }: realworld_domain::UserUpdate,
    ) -> Result<realworld_domain::User, realworld_domain::DatabaseError> {
        use crate::entity::users::{ActiveModel, Entity as User};
        let mut user: ActiveModel = User::find_by_id(user.id)
            .one(&self.0)
            .await
            .map_err(to_db_error)?
            .ok_or_else(|| realworld_domain::GetUserError::NotFound {
                user_id: user.id,
                source: anyhow::anyhow!("User not found").into(),
            })?
            .into();

        if let Some(email) = email {
            user.email = ActiveValue::Set(email);
        }
        if let Some(username) = username {
            user.username = ActiveValue::Set(username);
        }
        if let Some(password) = password {
            user.password = ActiveValue::Set(password.hash().to_string());
        }

        user.bio = ActiveValue::Set(bio);
        user.image = ActiveValue::Set(image);

        user.update(&self.0)
            .await
            .map(Into::into)
            .map_err(to_db_error)
            .map(Ok)?
    }

    async fn get_user_by_id(
        &self,
        user_id: Uuid,
    ) -> Result<realworld_domain::User, realworld_domain::GetUserError> {
        use crate::entity::users::Entity as User;

        User::find_by_id(user_id)
            .one(&self.0)
            .await
            .map_err(to_db_error)?
            .ok_or_else(|| realworld_domain::GetUserError::NotFound {
                user_id,
                source: anyhow::anyhow!("User not found").into(),
            })
            .map(Into::into)
    }

    async fn get_user_by_email_and_password(
        &self,
        email: &str,
        password: &str,
    ) -> Result<realworld_domain::User, realworld_domain::LoginError> {
        use crate::entity::users::{self, Entity as User};

        let user = User::find()
            .filter(users::Column::Email.eq(email))
            .one(&self.0)
            .await
            .map_err(to_db_error)?
            .ok_or(realworld_domain::LoginError::NotFound)?;
        let stored_password = realworld_domain::Password::from_hash(user.password.to_owned());
        if !stored_password.verify(password)? {
            return Err(realworld_domain::LoginError::NotFound);
        }
        Ok(user.into())
    }

    async fn get_profile(
        &self,
        username: &str,
    ) -> Result<realworld_domain::Profile, realworld_domain::GetUserError> {
        use crate::entity::users::{self, Entity as User};

        let user = User::find()
            .filter(users::Column::Username.eq(username))
            .one(&self.0)
            .await
            .map_err(to_db_error)?
            .ok_or_else(|| realworld_domain::GetUserError::NotFoundByUsername {
                username: username.to_string(),
                source: anyhow::anyhow!("User not found").into(),
            })?;

        Ok(user.into())
    }

    async fn get_profile_view(
        &self,
        viewer: &realworld_domain::User,
        username: &str,
    ) -> Result<realworld_domain::ProfileView, realworld_domain::GetUserError> {
        let viewed_user = self.get_user_by_username(username).await?;

        let following = self.is_following(viewer, &viewed_user).await?;
        let view = realworld_domain::ProfileView {
            profile: viewed_user.profile,
            following,
            viewer: viewer.id,
        };
        Ok(view)
    }

    async fn follow(
        &self,
        follower: &realworld_domain::User,
        to_be_followed: &realworld_domain::Profile,
    ) -> Result<(), realworld_domain::DatabaseError> {
        use crate::entity::followers::{self, ActiveModel, Entity as Followers};

        let user_to_be_followed = self.get_user_by_username(&to_be_followed.username).await?;
        let following = Followers::find()
            .filter(followers::Column::FollowerId.eq(follower.id))
            .filter(followers::Column::FollowedId.eq(user_to_be_followed.id))
            .one(&self.0)
            .await
            .map_err(to_db_error)?;

        match following {
            Some(_) => Ok(()),
            None => {
                let model = ActiveModel {
                    follower_id: ActiveValue::Set(follower.id),
                    followed_id: ActiveValue::Set(user_to_be_followed.id),
                };

                model.insert(&self.0).await.map_err(to_db_error)?;
                Ok(())
            }
        }
    }

    async fn unfollow(
        &self,
        follower: &realworld_domain::User,
        to_be_unfollowed: &realworld_domain::Profile,
    ) -> Result<(), realworld_domain::DatabaseError> {
        use crate::entity::followers::{self, Entity as Followers};
        let user_to_be_unfollowed = self
            .get_user_by_username(&to_be_unfollowed.username)
            .await?;

        Followers::delete_many()
            .filter(followers::Column::FollowerId.eq(follower.id))
            .filter(followers::Column::FollowedId.eq(user_to_be_unfollowed.id))
            .exec(&self.0)
            .await
            .map_err(to_db_error)?;

        Ok(())
    }

    async fn get_tags(
        &self,
    ) -> Result<std::collections::HashSet<String>, realworld_domain::DatabaseError> {
        #[derive(Debug, FromQueryResult)]
        pub struct UniqueTag {
            tag: String,
        }
        let stmt = Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"select DISTINCT tag.tag from (select jsonb_array_elements_text(tag_list) as tag from articles) as tag"#,
            vec![],
        );
        let tags = UniqueTag::find_by_statement(stmt)
            .all(&self.0)
            .await
            .map_err(to_db_error)?
            .into_iter()
            .map(|tag| tag.tag)
            .collect::<HashSet<String>>();
        Ok(tags)
    }
}

/// Helper function to cast a diesel::Error into a domain Database Error.
/// This requires casting the diesel::Error into anyhow::Error first.
pub fn to_db_error(e: sea_orm::DbErr) -> realworld_domain::DatabaseError {
    realworld_domain::DatabaseError::from(anyhow::Error::from(e))
}

#[derive(FromQueryResult)]
struct Count {
    count: i64,
}
impl Repository {
    pub async fn n_favorites(
        &self,
        article: &Article,
    ) -> Result<i64, realworld_domain::DatabaseError> {
        use crate::entity::favorites::{self, Entity as Favorites};
        Favorites::find()
            .select_only()
            .column_as(Expr::cust("count(*)"), "count")
            .filter(favorites::Column::ArticleId.eq(article.slug.clone()))
            .into_model::<Count>()
            .one(&self.0)
            .await
            .map(|row| row.map(|r| r.count).unwrap_or_default())
            .map_err(to_db_error)
    }
    pub async fn is_favorite(
        &self,
        article: &realworld_domain::Article,
        user: &realworld_domain::User,
    ) -> Result<bool, realworld_domain::DatabaseError> {
        use crate::entity::favorites::{self, Entity as Favorites};
        Favorites::find()
            .filter(favorites::Column::ArticleId.eq(article.slug.clone()))
            .filter(favorites::Column::UserId.eq(user.id))
            .one(&self.0)
            .await
            .map(|row| row.is_some())
            .map_err(to_db_error)
    }
    pub async fn are_favorites<'a>(
        &self,
        articles: &'a [realworld_domain::Article],
        user: &realworld_domain::User,
    ) -> Result<HashMap<&'a str, bool>, realworld_domain::DatabaseError> {
        use crate::entity::favorites::{self, Entity as Favorites};

        let favs = Favorites::find()
            .filter(
                favorites::Column::ArticleId.is_in(
                    articles
                        .iter()
                        .map(|article| article.slug.as_str())
                        .collect::<Vec<&str>>(),
                ),
            )
            .filter(favorites::Column::UserId.eq(user.id))
            .all(&self.0)
            .await
            .map_err(to_db_error)?;

        let favs_slugs = favs
            .iter()
            .map(|favorite| favorite.article_id.as_str())
            .collect::<HashSet<&str>>();

        let mut favorited = HashMap::new();
        for article in articles.iter() {
            favorited.insert(
                article.slug.as_str(),
                favs_slugs.contains(article.slug.as_str()),
            );
        }

        Ok(favorited)
    }
    pub async fn is_following(
        &self,
        viewer: &realworld_domain::User,
        viewed: &realworld_domain::User,
    ) -> Result<bool, realworld_domain::DatabaseError> {
        use crate::entity::followers::{self, Entity as Followers};
        Followers::find()
            .filter(followers::Column::FollowerId.eq(viewer.id))
            .filter(followers::Column::FollowedId.eq(viewed.id))
            .one(&self.0)
            .await
            .map(|row| row.is_some())
            .map_err(to_db_error)
    }

    async fn get_user_by_username(
        &self,
        username: &str,
    ) -> Result<realworld_domain::User, realworld_domain::GetUserError> {
        use crate::entity::users::{self, Entity as User};

        User::find()
            .filter(users::Column::Username.eq(username))
            .one(&self.0)
            .await
            .map_err(to_db_error)?
            .ok_or_else(|| realworld_domain::GetUserError::NotFoundByUsername {
                username: username.to_string(),
                source: anyhow::anyhow!("User not found").into(),
            })
            .map(Into::into)
    }
}
