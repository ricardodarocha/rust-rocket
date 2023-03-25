
// region Basic GET
#[test]
fn test_visitante() {
    let cli = rocket::local::blocking::Client::tracked(super::rocket()).unwrap();
    let response = cli.get("/api/vs").dispatch();
    assert_eq!(response.status(), rocket::http::Status::Ok);
}
// endregion

