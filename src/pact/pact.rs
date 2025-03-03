// use pact_consumer::prelude::*;

#[tokio::test]
async fn a_service_consumer_side_of_a_pact_goes_a_little_something_like_this() {
   let alice_service = PactBuilder::new("Consumer", "Alice Service")
       // Start a new interaction. We can add as many interactions as we want.
       .interaction("a retrieve Mallory request", "", |mut i| {
           // Defines a provider state. It is optional.
           i.given("there is some good mallory");
           // Define the request, a GET (default) request to '/mallory'.
           i.request.path("/mallory");
           // Define the response we want returned. We assume a 200 OK
           // response by default.
           i.response
               .content_type("text/plain")
               .body("That is some good Mallory.");
           // Return the interaction builder back to the pact framework
           i
       }).start_mock_server(None, None);
  
   // You would use your actual client code here.
   let mallory_url = alice_service.path("/mallory");
   let response = reqwest::get(mallory_url).await.expect("could not fetch URL")
     .text().await.expect("Could not read response body");
   assert_eq!(response, "That is some good Mallory.");
  
   // When `alice_service` goes out of scope, your pact will be validated,
   // and the test will fail if the mock server didn't receive matching
   // requests.
}