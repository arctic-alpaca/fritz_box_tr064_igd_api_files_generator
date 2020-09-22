# FRITZ!Box-TR064-IGD-API-files-generator

A tool to generate rust source code files for interacting with the [AVM FritzBox TR-064 and IGD APIs](https://avm.de/service/schnittstellen/).
Please be aware that this isn't a library to interact with the FRITZ!Box. All interactions can be implemented on top of the generated files.

## What it does
AVM FRITZ!Boxes provide API descriptions at `http://fritz.box:49000/igddesc.xml` and `http://fritz.box:49000/tr64desc.xml`. This application parses and creates usable rust files out of them.

## Output
The resulting files can be found in `output/responses` and `output/requests`, the location can be modified in `main`.

### Request files
Each method inside the request files corresponds to an API call and can be called with the appropriate parameters to create the uri, header and body parts of a valid API request.
The `id` parameter is used to identify the service if there is more than one. By default this is `1` if `None` is supplied.

### Response files
The APIs return XML responses which can be deserialized with [serde-xml-rs](https://crates.io/crates/serde-xml-rs) and [serde](https://crates.io/crates/serde) into structs to easily work with.
`#![recursion_limit = "512"]` is probably required in any application using the generated response files since the `multi_use.rs` file contains a huge amount of `serde` macros to avoid having to create an `envelope` and `body` struct for every file. 

## FRITZ!Box and FRITZ!OS Version
This code has only been tested with the FRITZ!Box 6490 Cable and FRITZ!OS 7.20. Any FRITZ!Box should work as long as the API description format is unchanged.
I can't test with any other hardware, if you run into problems, please open an issue. 

## Usage
### Generation
 1. `git clone https://github.com/arctic-alpaca/fritz_box_tr064_igd_api_files_generator.git`
 2. modify `main` constants
 3. `cargo run`
 4. the generated files can be found in folder `output` in the current working directory
 
### Integration
An example how to use the generated files. Be aware, depending on the API call you want to perform, you might need to authenticate yourself.
```rust
// generate uri, header and body for the API call you want to perform, the method `generate_set_persistent_data_request` is generated for you
let (uri, header, body) = generate_get_persistent_data_request(None);
// send them to the fritzbox, you need to implement this
let response = send_request(uri, header, body);
// deserialize the response in the appropriate struct, the struct is generated for you
let desirialized_struct: Envelope<GetPersistentDataResponse> = serde_xml_rs::from_str(&*response)?;
```


## Examples of generated methods/structs
### Requests
```rust
pub fn generate_get_persistent_data_request( id: Option<&str>) -> (String, String, String){ 
	let id = id.unwrap_or("1");
	let uri = "/upnp/control/deviceconfig";
	let header = format!("urn:dslforum-org:service:DeviceConfig:{}#GetPersistentData", id);
	let body = format!("<?xml version=\"1.0\"?><s:Envelope xmlns:s=\"http://schemas.xmlsoap.org/soap/envelope/\"s:encodingStyle=\"http://schemas.xmlsoap.org/soap/encoding/\"><s:Body><u:GetPersistentData xmlns:u=\"urn:dslforum-org:service:DeviceConfig:{}\"></u:GetPersistentData></s:Body></s:Envelope>",id );
	(uri.to_string(), header, body) 
}
pub fn generate_set_persistent_data_request(new_persistent_data: &str, id: Option<&str>) -> (String, String, String){ 
	let id = id.unwrap_or("1");
	let uri = "/upnp/control/deviceconfig";
	let header = format!("urn:dslforum-org:service:DeviceConfig:{}#SetPersistentData", id);
	let body = format!("<?xml version=\"1.0\"?><s:Envelope xmlns:s=\"http://schemas.xmlsoap.org/soap/envelope/\"s:encodingStyle=\"http://schemas.xmlsoap.org/soap/encoding/\"><s:Body><u:SetPersistentData xmlns:u=\"urn:dslforum-org:service:DeviceConfig:{}\"><NewPersistentData>{}</NewPersistentData></u:SetPersistentData></s:Body></s:Envelope>",id ,new_persistent_data);
	(uri.to_string(), header, body) 
}
```

### Responses
```rust
#[derive(Deserialize, Debug)]
pub struct GetPersistentDataResponse{
	#[serde(rename = "NewPersistentData")]
	pub new_persistent_data: String,
}

#[derive(Deserialize, Debug)]
pub struct SetPersistentDataResponse{
}
```

## Questions
If you have and questions, find bugs or have a feature in mind please feel free to open an issue.
