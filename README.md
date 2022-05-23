# se_ms_api
 SolarEdge Monitoring Server API

This library provides access to the SolarEdge Monitoring API.  The API is defined [here](https://www.solaredge.com/sites/default/files/se_monitoring_api.pdf),
released January 2022.  I am in no way associated with SolarEdge.  And it follows that this software is not associated with SolarEdge.

You can use this library to get data collected by your SolarEdge inverter.  I use it to get the amount of self consumption for a billing period.
I then use this data to track how close I am to breaking even on my solar investment.

I've implemented the API as defined, except for the bulk requests. Since I only have one inverter, I cannot test bulk.

I did this as an exercise to learn Rust.  There are other similar libraries out there that are more sophisticated. 

Due to the restrictions that SolarEdge imposes on this API, this library does not try to be performant. For example, it makes blocking HTTP requests.

The basic use case is:

1. Create a SolarEdge struct that contains the site id and api key that will be used for the requests. (The se_monitoring_api.pdf linked above has instructions for getting your site id and api key.
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
