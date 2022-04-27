// These tests are "integration" tests that exercise a workflow via the http service.

mod helpers;

use helpers::create_users;
use helpers::test_server::TestApp;

use itertools::Itertools;
use realworld_web::auth::encode_token;

#[tokio::test]
async fn profiles_api() {
    let mut server = TestApp::create("profiles_api").await;
    let users = create_users(&server.repository, 2)
        .await
        .into_iter()
        .map(|(u, _)| u)
        .collect_vec();
    let follower_user = users[0].clone();
    let followed_user = users[1].clone();

    let followed_profile = server
        .get_profile(&followed_user.profile.username, None)
        .await
        .unwrap();
    assert_eq!(
        followed_profile.profile.username,
        followed_user.profile.username
    );
    assert_eq!(followed_profile.profile.bio, followed_user.profile.bio);
    assert_eq!(followed_profile.profile.image, followed_user.profile.image);
    assert_eq!(followed_profile.profile.following, false);

    let follower_token = encode_token(follower_user.id);
    let followed_profile = server
        .follow_profile(&followed_user.profile.username, &follower_token)
        .await
        .unwrap();
    assert_eq!(followed_profile.profile.following, true);

    // If not logged in, following is still false
    let followed_profile = server
        .get_profile(&followed_user.profile.username, None)
        .await
        .unwrap();
    assert_eq!(followed_profile.profile.following, false);

    // If logged in, following is correctly valued
    let followed_profile = server
        .get_profile(&followed_user.profile.username, Some(&follower_token))
        .await
        .unwrap();
    assert_eq!(followed_profile.profile.following, true);

    let unfollowed_profile = server
        .unfollow_profile(&followed_user.profile.username, &follower_token)
        .await
        .unwrap();
    assert_eq!(unfollowed_profile.profile.following, false);

    // After unfollowing, following is now false
    let p = server
        .get_profile(&followed_user.profile.username, Some(&follower_token))
        .await
        .unwrap();
    assert_eq!(p.profile.following, false);
}
