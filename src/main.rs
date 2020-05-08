

use std::env;
use ruptane_common::{VehicleUpdateRequest, VehicleUpdateResponse, SignedPayload, Signature};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    //args[0] is the app name
    //args[1] is the service endpoint url, if any
    let service_url =
        if args.len() > 1 {  args[1].clone()  }
        else {  env::var("SERVICE_URL")?.clone()  };

    let request_event = VehicleUpdateRequest {
        vehicle_id: "1234567890ABCDEF1234567890ABCDEF".into(),
        last_update: "2019-10-12T07:20:50.52Z".into()
    };
    //TODO generate non-canned signatures
    let sig = Signature {
        keyid: "9a406d99e362e7c93e7acfe1e4d6585221315be817f350c026bbee84ada260da".into(),
        method: "ed25519".into(),
        sig: "335272f77357dc0e9f1b74d72eb500e4ff0f443f824b83405e2b21264778d1610e0a5f2663b90eda8ab05a28b5b64fc15514020985d8a93576fe33b287e1380f".into()
    };
    let sign_wrapper = SignedPayload::<VehicleUpdateRequest> {
        signatures: vec![sig],
        signed: request_event
    };

    let client = reqwest::blocking::Client::new();
    let resp = client.post(&service_url)
        .json(&sign_wrapper)
        .send()?;

    println!("{:#?}", resp);

    let resp_obj_res:Result<SignedPayload<VehicleUpdateResponse>, reqwest::Error> = resp.json();
    if resp_obj_res.is_ok() {
        let resp_obj = resp_obj_res.unwrap();
        println!("resp_obj: {:?}", resp_obj);
    }
    else {
        println!("resp json error: {:?}", resp_obj_res);
    }

    Ok(())
}