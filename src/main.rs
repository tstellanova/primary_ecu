


use serde_derive::{Serialize, Deserialize};
use std::env;


#[derive(Serialize, Clone, Debug)]
struct CustomEvent {
    vehicle_id: String,
    last_update: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct CustomOutput {
    resp_version: String,
    vehicle_id: String,
    message: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    //args[0] is the app name
    //args[1] is the service endpoint url, if any

    let service_url_opt: Option<String> =
        if args.len() > 1 {
            Some(args[1].clone())
        }
        else {
            let tmp = env::var("SERVICE_URL")?;
            Some(tmp.clone())
        };
    

    let request_event = CustomEvent {
        vehicle_id: "1234567890ABCDEF1234567890ABCDEF".into(),
        last_update: "2019-10-12T07:20:50.52Z".into()
    };

    let client = reqwest::blocking::Client::new();
    let resp = client.post(&service_url_opt.unwrap())
        .json(&request_event)
        .send()?;

    println!("{:#?}", resp);

    let resp_obj_res:Result<CustomOutput, reqwest::Error> = resp.json();
    if resp_obj_res.is_ok() {
        let resp_obj = resp_obj_res.unwrap();
        println!("resp_obj: {:?}", resp_obj);
    }
    else {
        println!("resp json error: {:?}", resp_obj_res);
    }

    Ok(())
}