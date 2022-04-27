use crate::{
    Article, ArticleContent, ArticleQuery, ArticleUpdate, ArticleView, Comment, CommentContent,
    DatabaseError, DeleteCommentError, FavoriteOutcome, FeedQuery, GetArticleError, GetUserError,
    LoginError, Profile, ProfileView, PublishArticleError, SignUp, SignUpError, UnfavoriteOutcome,
    User, UserUpdate,
};
use std::collections::HashSet;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait Repository: Send + Sync {
    async fn publish_article(
        &self,
        draft: ArticleContent,
        author: &User,
    ) -> Result<Article, PublishArticleError>;
    async fn get_article_by_slug(&self, slug: &str) -> Result<Article, GetArticleError>;
    async fn get_article_view(
        &self,
        viewer: &User,
        article: Article,
    ) -> Result<ArticleView, GetArticleError>;
    async fn get_articles_views(
        &self,
        viewer: &User,
        articles: Vec<Article>,
    ) -> Result<Vec<ArticleView>, DatabaseError>;
    async fn find_articles(&self, query: ArticleQuery) -> Result<Vec<Article>, DatabaseError>;
    async fn feed(&self, user: &User, query: FeedQuery) -> Result<Vec<ArticleView>, DatabaseError>;
    async fn delete_article(&self, article: &Article) -> Result<(), DatabaseError>;
    async fn comment_article(
        &self,
        user: &User,
        article: &Article,
        comment: CommentContent,
    ) -> Result<Comment, DatabaseError>;
    async fn get_comment(&self, comment_id: u64) -> Result<Comment, DeleteCommentError>;
    async fn get_comments(&self, article: &Article) -> Result<Vec<Comment>, DatabaseError>;
    async fn delete_comment(&self, comment_id: u64) -> Result<(), DeleteCommentError>;
    async fn update_article(
        &self,
        article: Article,
        update: ArticleUpdate,
    ) -> Result<Article, DatabaseError>;
    async fn favorite(
        &self,
        article: &Article,
        user: &User,
    ) -> Result<FavoriteOutcome, DatabaseError>;
    async fn unfavorite(
        &self,
        article: &Article,
        user: &User,
    ) -> Result<UnfavoriteOutcome, DatabaseError>;
    async fn sign_up(&self, sign_up: SignUp) -> Result<User, SignUpError>;
    async fn update_user(&self, user: User, update: UserUpdate) -> Result<User, DatabaseError>;
    async fn get_user_by_id(&self, user_id: Uuid) -> Result<User, GetUserError>;
    async fn get_user_by_email_and_password(
        &self,
        email: &str,
        password: &str,
    ) -> Result<User, LoginError>;
    async fn get_profile(&self, username: &str) -> Result<Profile, GetUserError>;
    async fn get_profile_view(
        &self,
        viewer: &User,
        username: &str,
    ) -> Result<ProfileView, GetUserError>;
    async fn follow(&self, follower: &User, to_be_followed: &Profile) -> Result<(), DatabaseError>;
    async fn unfollow(
        &self,
        follower: &User,
        to_be_unfollowed: &Profile,
    ) -> Result<(), DatabaseError>;
    async fn get_tags(&self) -> Result<HashSet<String>, DatabaseError>;
}
