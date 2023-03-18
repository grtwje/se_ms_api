# se_ms_api
 SolarEdge Monitoring Server API

This library provides access to the SolarEdge Monitoring API. The API is defined [here](https://www.solaredge.com/sites/default/files/se_monitoring_api.pdf), released January 2022. I am in no way associated with SolarEdge. And it follows that this software is not associated with SolarEdge.

You can use this library to get data collected by your SolarEdge inverter. I use it to get the amount of self consumption for a billing period. I then use this data to track how close I am to breaking even on my solar investment.

I've implemented most of the API.
* The bulk requests have not been implemented. Since I only have one inverter, I cannot test bulk.
* The site storage data request is only partially tested since I don't have a battery (yet).
* The site image and installer logo image requests are not implemented. I did not want to deal with handling the images.
* The get sensor data request is not implemented since I do not have any sensors to test against.

Note that the API document provided by SolarEdge is not complete. And based on actual testing, in some cases it is incorrect. I have tested against my site's data. For other site's I expect there are cases where se_ms_api will be surprised by the response returned from the SolarEdge server. Open an issue for these cases.

I did this as an exercise to learn Rust.  There are other similar libraries out there that are more sophisticated. 

Due to the restrictions that SolarEdge imposes on this API, this library does not try to be performant. For example, it makes blocking HTTP requests.

The basic use case is:

1. Create a SolarEdge struct that contains the site id and api key that will be used for the requests. (The se_monitoring_api.pdf linked above has instructions for getting your site id and api key.)
2. Create a request for the information that you want.
3. Send the request using the SolarEdge struct.
4. Read the response to get the information.

```rust
extern crate se_ms_api;
use se_ms_api::{SendReq, SiteDetailsReq, SolaredgeCredentials};

let site_id = "my_site_id";
let api_key = "my_api_key";

let cred = SolaredgeCredentials::new(&site_id, &api_key); // (1)
let req = SiteDetailsReq::new();                          // (2)
let resp = req.send(&cred);                               // (3)

match resp {                                              // (4)
   Ok(r) => {
       println!("My site's status is {}.", r.details.status);
   }
   Err(e) => {
       panic!("Unexpected SiteDetails response: {:?}", e);
   }
}
```

To include the latest stable release, add this to your Cargo.toml file. (If there's interest, and it matures a bit, I'll put it on crates.io later.)

```toml
[dependencies]
se_monitoring_server_api = {git = "https://github.com/grtwje/se_ms_api", tag = "0.1.0-alpha.4"}
```
