mod helpers;

use helpers::generate;
use helpers::test_server::TestApp;

use realworld_web::users::responses::UserResponse;
use realworld_web::users::update::UpdateUserRequest;

#[tokio::test]
async fn register_and_login() {
    let mut server = TestApp::create("register_and_login").await;
    let (user, password) = generate::new_user();

    server.register_user(&user, &password).await.unwrap();
    let token = server
        .login_user(&user.email, &password)
        .await
        .unwrap()
        .user
        .token;
    let user_details = server.get_current_user(&token).await.unwrap();

    assert_eq!(user_details.user.username, user.username);
    assert_eq!(user_details.user.email, user.email);
}

#[tokio::test]
async fn update_and_retrieve_user_details() {
    let mut server = TestApp::create("update_and_retrieve_user_details").await;
    let (user, password) = generate::new_user();

    let stored_user = server.register_user(&user, &password).await.unwrap();
    let token = server
        .login_user(&user.email, &password)
        .await
        .unwrap()
        .user
        .token;

    assert_eq!(stored_user.user.bio, None);
    assert_eq!(stored_user.user.image, None);

    let new_details = realworld_web::users::update::Request {
        user: UpdateUserRequest {
            bio: Some("I like to code.".to_string()),
            image: Some("https://www.rust-lang.org/static/images/rust-logo-blk.svg".to_string()),
            email: None,
            password: None,
            username: None,
        },
    };
    let updated_user: UserResponse = server
        .update_user_details(&new_details, &token)
        .await
        .unwrap();
    assert_eq!(updated_user.user.bio, new_details.user.bio);
    assert_eq!(updated_user.user.image, new_details.user.image);

    let current_user: UserResponse = server.get_current_user(&token).await.unwrap();
    assert_eq!(current_user.user.bio, new_details.user.bio);
    assert_eq!(current_user.user.image, new_details.user.image);
}
