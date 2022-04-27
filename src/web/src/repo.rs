use domain::repositories::Repository;
use std::sync::Arc;

#[derive(Clone)]
pub struct ArcRepo(Arc<dyn Repository>);

impl ArcRepo {
    pub fn new(repo: impl Repository + 'static) -> Self {
        Self(Arc::new(repo))
    }
}

#[async_trait::async_trait]
impl Repository for ArcRepo {
    async fn publish_article(
        &self,
        draft: domain::ArticleContent,
        author: &domain::User,
    ) -> Result<domain::Article, domain::PublishArticleError> {
        self.0.publish_article(draft, author).await
    }

    async fn get_article_by_slug(
        &self,
        slug: &str,
    ) -> Result<domain::Article, domain::GetArticleError> {
        self.0.get_article_by_slug(slug).await
    }

    async fn get_article_view(
        &self,
        viewer: &domain::User,
        article: domain::Article,
    ) -> Result<domain::ArticleView, domain::GetArticleError> {
        self.0.get_article_view(viewer, article).await
    }

    async fn get_articles_views(
        &self,
        viewer: &domain::User,
        articles: Vec<domain::Article>,
    ) -> Result<Vec<domain::ArticleView>, domain::DatabaseError> {
        self.0.get_articles_views(viewer, articles).await
    }

    async fn find_articles(
        &self,
        query: domain::ArticleQuery,
    ) -> Result<Vec<domain::Article>, domain::DatabaseError> {
        self.0.find_articles(query).await
    }

    async fn feed(
        &self,
        user: &domain::User,
        query: domain::FeedQuery,
    ) -> Result<Vec<domain::ArticleView>, domain::DatabaseError> {
        self.0.feed(user, query).await
    }

    async fn delete_article(&self, article: &domain::Article) -> Result<(), domain::DatabaseError> {
        self.0.delete_article(article).await
    }

    async fn comment_article(
        &self,
        user: &domain::User,
        article: &domain::Article,
        comment: domain::CommentContent,
    ) -> Result<domain::Comment, domain::DatabaseError> {
        self.0.comment_article(user, article, comment).await
    }

    async fn get_comment(
        &self,
        comment_id: u64,
    ) -> Result<domain::Comment, domain::DeleteCommentError> {
        self.0.get_comment(comment_id).await
    }

    async fn get_comments(
        &self,
        article: &domain::Article,
    ) -> Result<Vec<domain::Comment>, domain::DatabaseError> {
        self.0.get_comments(article).await
    }

    async fn delete_comment(&self, comment_id: u64) -> Result<(), domain::DeleteCommentError> {
        self.0.delete_comment(comment_id).await
    }

    async fn update_article(
        &self,
        article: domain::Article,
        update: domain::ArticleUpdate,
    ) -> Result<domain::Article, domain::DatabaseError> {
        self.0.update_article(article, update).await
    }

    async fn favorite(
        &self,
        article: &domain::Article,
        user: &domain::User,
    ) -> Result<domain::FavoriteOutcome, domain::DatabaseError> {
        self.0.favorite(article, user).await
    }

    async fn unfavorite(
        &self,
        article: &domain::Article,
        user: &domain::User,
    ) -> Result<domain::UnfavoriteOutcome, domain::DatabaseError> {
        self.0.unfavorite(article, user).await
    }

    async fn sign_up(&self, sign_up: domain::SignUp) -> Result<domain::User, domain::SignUpError> {
        self.0.sign_up(sign_up).await
    }

    async fn update_user(
        &self,
        user: domain::User,
        update: domain::UserUpdate,
    ) -> Result<domain::User, domain::DatabaseError> {
        self.0.update_user(user, update).await
    }

    async fn get_user_by_id(
        &self,
        user_id: uuid::Uuid,
    ) -> Result<domain::User, domain::GetUserError> {
        self.0.get_user_by_id(user_id).await
    }

    async fn get_user_by_email_and_password(
        &self,
        email: &str,
        password: &str,
    ) -> Result<domain::User, domain::LoginError> {
        self.0.get_user_by_email_and_password(email, password).await
    }

    async fn get_profile(&self, username: &str) -> Result<domain::Profile, domain::GetUserError> {
        self.0.get_profile(username).await
    }

    async fn get_profile_view(
        &self,
        viewer: &domain::User,
        username: &str,
    ) -> Result<domain::ProfileView, domain::GetUserError> {
        self.0.get_profile_view(viewer, username).await
    }

    async fn follow(
        &self,
        follower: &domain::User,
        to_be_followed: &domain::Profile,
    ) -> Result<(), domain::DatabaseError> {
        self.0.follow(follower, to_be_followed).await
    }

    async fn unfollow(
        &self,
        follower: &domain::User,
        to_be_unfollowed: &domain::Profile,
    ) -> Result<(), domain::DatabaseError> {
        self.0.unfollow(follower, to_be_unfollowed).await
    }

    async fn get_tags(&self) -> Result<std::collections::HashSet<String>, domain::DatabaseError> {
        self.0.get_tags().await
    }
}
