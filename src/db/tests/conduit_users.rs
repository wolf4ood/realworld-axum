use helpers::create_user;
use realworld_domain::{repositories::Repository, UserUpdate};
mod helpers;
use fake::fake;
use realworld_tests::db::test_db;

#[tokio::test]
async fn test_create_user() {
    let db = test_db("test_create_user").await;
    let user = create_user(&db).await.0;
    let results = db.0.get_user_by_id(user.id).await;
    assert!(results.is_ok());
}

#[tokio::test]
async fn test_authenticate_user() {
    let db = test_db("test_authenticate_user").await;

    // Create a new user
    let (user, password) = create_user(&db).await;

    let results =
        db.0.get_user_by_email_and_password(&user.email, &password)
            .await;
    assert!(results.is_ok());
}

#[tokio::test]
async fn test_update_user() {
    let db = test_db("test_update_user").await;
    let user = create_user(&db).await.0;

    let bio = fake!(Lorem.paragraph(3, 5)).to_string();
    let image = fake!(Internet.domain_suffix).to_string();
    let email = fake!(Internet.free_email).to_string();

    let new_details = UserUpdate {
        bio: Some(bio.clone()),
        image: Some(image.clone()),
        email: Some(email.clone()),
        username: None,
        password: None,
    };

    let user_id = user.id;
    db.0.update_user(user, new_details)
        .await
        .expect("Failed to update user");

    let updated_user =
        db.0.get_user_by_id(user_id)
            .await
            .expect("Failed to get user");

    assert_eq!(updated_user.profile.bio, Some(bio));
    assert_eq!(updated_user.profile.image, Some(image));
    assert_eq!(updated_user.email, email);
}
