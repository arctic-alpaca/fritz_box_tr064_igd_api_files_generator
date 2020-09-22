use crate::api_handling::api_desc::ApiDesc;
use std::fs;
use std::fs::File;
use std::io::Write;

///Struct to deserialize the response from "fritz.box/tr64desc.xml" into.
#[derive(Deserialize, Debug, Default)]
pub struct ApiDescDir {
    #[serde(rename = "specVersion")]
    pub spec_version: SpecVersion,
    #[serde(rename = "systemVersion")]
    #[serde(default)]
    pub system_version: SystemVersion,
    pub device: Device,
}
///Struct to deserialize the device part of the response from "fritz.box/tr64desc.xml" into.
#[derive(Deserialize, Debug, Default)]
pub struct Device {
    #[serde(rename = "deviceType")]
    pub device_type: String,
    #[serde(rename = "friendlyName")]
    pub friendly_name: String,
    #[serde(rename = "manufacturer")]
    pub manufacturer: String,
    #[serde(rename = "manufacturerURL")]
    pub manufacturer_url: String,
    #[serde(rename = "modelDescription")]
    pub model_description: String,
    #[serde(rename = "modelName")]
    pub model_name: String,
    #[serde(rename = "modelNumber")]
    pub model_number: String,
    #[serde(rename = "modelURL")]
    pub model_url: String,
    #[serde(rename = "UDN")]
    pub udn: String,
    #[serde(rename = "UPC")]
    #[serde(default)]
    pub upc: String,
    #[serde(rename = "iconList")]
    #[serde(default)]
    pub icon_list: IconList,
    #[serde(rename = "serviceList")]
    pub service_list: ServiceList,
    #[serde(rename = "deviceList")]
    #[serde(default)]
    pub device_list: DeviceList,
    #[serde(rename = "presentationURL")]
    #[serde(default)]
    pub presentation_url: String,
}
///Struct to deserialize the icon list part of the response from "fritz.box/tr64desc.xml" into.
#[derive(Deserialize, Debug, Default)]
pub struct IconList {
    #[serde(rename = "icon")]
    pub icon: Vec<Icon>,
}

///Struct to deserialize the device list part of the response from "fritz.box/tr64desc.xml" into.
#[derive(Deserialize, Debug, Default)]
pub struct DeviceList {
    #[serde(rename = "device")]
    pub device: Vec<Device>,
}

///Struct to deserialize the icon part of the response from "fritz.box/tr64desc.xml" into.
#[derive(Deserialize, Debug, Default)]
pub struct Icon {
    #[serde(default)]
    pub mimetype: String,
    #[serde(default)]
    pub width: u32,
    #[serde(default)]
    pub height: u32,
    #[serde(default)]
    pub depth: u32,
    #[serde(default)]
    pub url: String,
}
///Struct to deserialize the service list part of the response from "fritz.box/tr64desc.xml" into.
#[derive(Deserialize, Debug, Default)]
pub struct ServiceList {
    #[serde(rename = "service")]
    pub service: Vec<Service>,
}
///Struct to deserialize the service part of the response from "fritz.box/tr64desc.xml" into.
#[derive(Deserialize, Debug, Default)]
pub struct Service {
    #[serde(rename = "serviceType")]
    pub service_type: String,
    #[serde(rename = "serviceId")]
    pub service_id: String,
    #[serde(rename = "controlURL")]
    pub control_url: String,
    #[serde(rename = "eventSubURL")]
    pub event_sub_url: String,
    #[serde(rename = "SCPDURL")]
    pub scpd_url: String,
}
///Struct to deserialize the system version part of the response from "fritz.box/tr64desc.xml" into.
#[derive(Deserialize, Debug, Default)]
pub struct SystemVersion {
    #[serde(rename = "HW")]
    pub hw: u32,
    #[serde(rename = "Major")]
    pub major: u32,
    #[serde(rename = "Minor")]
    pub minor: u32,
    #[serde(rename = "Patch")]
    pub patch: u32,
    #[serde(rename = "Buildnumber")]
    pub buildnumber: u32,
    #[serde(rename = "Display")]
    pub display: String,
}
///Struct to deserialize the spec version part of the response from "fritz.box/tr64desc.xml" into.
#[derive(Deserialize, Debug, Default)]
pub struct SpecVersion {
    pub major: u32,
    pub minor: u32,
}

/// Struct to build response files. File parts get pushed into `content` and assembled later.
#[derive(Debug)]
pub struct ResponseFile {
    pub name: String,
    pub content: Vec<String>,
}

impl ResponseFile {
    pub fn new() -> Self {
        ResponseFile {
            name: "".to_string(),
            content: vec![],
        }
    }
}

/// Parameter with it's type, part of `RequestFunction`.
#[derive(Debug)]
pub struct ParameterAndType {
    pub parameter_name: String,
    pub parameter_name_rusty: String,
    pub type_name: String,
}

impl ParameterAndType {
    pub fn new() -> Self {
        ParameterAndType {
            parameter_name: "".to_string(),
            parameter_name_rusty: "".to_string(),
            type_name: "".to_string(),
        }
    }
}

/// Struct to build response files. `RequestFunction`s get pushed into `request_functions` and assembled later.
#[derive(Debug)]
pub struct RequestFile {
    pub name: String,
    pub request_functions: Vec<RequestFunction>,
}

impl RequestFile {
    pub fn new() -> Self {
        RequestFile {
            name: "".to_string(),
            request_functions: vec![],
        }
    }
}

/// Represents a request function, `name` is taken directly from the API, `name_rusty` is the same name in proper snake case.
/// `service_type`, `action_name` and `control_type` are directly taken from the API.
#[derive(Debug)]
pub struct RequestFunction {
    pub name: String,
    pub name_rusty: String,
    pub parameter: Vec<ParameterAndType>,
    pub service_type: String,
    pub action_name: String,
    pub control_url: String,
}

impl RequestFunction {
    pub fn new() -> Self {
        RequestFunction {
            name: "".to_string(),
            name_rusty: "".to_string(),
            parameter: vec![],
            service_type: "".to_string(),
            action_name: "".to_string(),
            control_url: "".to_string(),
        }
    }

    /// Returns the String representation of a request function.
    pub fn create_function(&self) -> String {
        let service_type_parts: Vec<&str> = self.service_type.split(':').collect();

        let function_print = format!(
            "pub fn generate_{}_request({} id: Option<&str>) -> (String, String, String)\
             {{ \n\tlet id = id.unwrap_or(\"1\");\n",
            self.name_rusty,
            self.parameter_for_function()
        );

        let uri_print = format!("\tlet uri = \"{}\";\n", self.control_url);

        let header = format!(
            "\tlet header = format!(\"{}:{}:{}:{}:{{}}#{}\", id);\n",
            service_type_parts[0],
            service_type_parts[1],
            service_type_parts[2],
            service_type_parts[3],
            self.action_name
        );

        let body_print = format!(
            "\tlet body = format!(\"<?xml version=\\\"1.0\\\"?><s:Envelope \
            xmlns:s=\\\"http://schemas.xmlsoap.org/soap/envelope/\\\"s:encodingStyle=\
            \\\"http://schemas.xmlsoap.org/soap/encoding/\\\"><s:Body><u:{} xmlns:u=\
            \\\"{}:{}:{}:{}:{{}}\\\">{}</u:{}></s:Body></s:Envelope>\",id {});\n",
            self.action_name,
            service_type_parts[0],
            service_type_parts[1],
            service_type_parts[2],
            service_type_parts[3],
            self.parameter_for_body(),
            self.action_name,
            self.parameter_for_code()
        );
        format!(
            "{}{}{}{}\t(uri.to_string(), header, body) \n}}\n",
            function_print, uri_print, header, body_print
        )
    }

    /// Helper function for `create_function`. Creates a String containing the parameter for the request function signature.
    pub fn parameter_for_function(&self) -> String {
        if self.parameter.is_empty() {
            return "".to_string();
        }
        let mut result = String::new();
        for parameter in &self.parameter {
            if parameter.type_name == "String" {
                result.push_str(format!("{}: &str,", parameter.parameter_name_rusty).as_str());
            } else {
                result.push_str(
                    format!(
                        "{}: &{},",
                        parameter.parameter_name_rusty, parameter.type_name
                    )
                    .as_str(),
                );
            }
        }
        result
    }

    /// Helper function for `create_function`. Creates a String containing the parameter for the soap request body.
    pub fn parameter_for_body(&self) -> String {
        let mut result = String::new();
        for parameter in &self.parameter {
            result.push_str(
                format!(
                    "<{}>{{}}</{}>",
                    parameter.parameter_name, parameter.parameter_name
                )
                .as_str(),
            );
        }
        result
    }

    /// Helper function for `create_function`. Creates a String containing the parameter for insertion (format!...) part of the function.
    pub fn parameter_for_code(&self) -> String {
        let mut result = String::new();
        for parameter in &self.parameter {
            result.push_str(format!(",{}", parameter.parameter_name_rusty).as_str());
        }
        result
    }
}

/// Struct to collect file names, content and the annotation for the Body deserialization struct.
#[derive(Debug)]
pub struct OutputFiles {
    pub annotation_string: Vec<String>,
    pub response_files: Vec<ResponseFile>,
    pub request_files: Vec<RequestFile>,
    pub response_output_folder: String,
    pub request_output_folder: String,
    pub prefix: String,
}
impl OutputFiles {
    pub fn new() -> Self {
        OutputFiles {
            annotation_string: vec![],
            response_files: vec![],
            request_files: vec![],
            response_output_folder: "response_output".to_string(),
            request_output_folder: "request_output".to_string(),
            prefix: "".to_string(),
        }
    }

    /// Calls all functions needed to actually create the output files.
    fn create_files(&self) {
        self.create_output_folders();
        self.write_annotation_file();
        self.write_mod_files();
        self.write_requests_files();
        self.write_responses_files();
    }
    /// Creates output folders. If they are already present, do nothing. On every other error, panic.
    fn create_output_folders(&self) {
        if let Err(e) = fs::create_dir_all(&self.response_output_folder) {
            if e.kind() != std::io::ErrorKind::AlreadyExists {
                println!("{}", e.to_string());
                panic!();
            }
        };
        if let Err(e) = fs::create_dir_all(&self.request_output_folder) {
            if e.kind() != std::io::ErrorKind::AlreadyExists {
                println!("{}", e.to_string());
                panic!();
            }
        };
    }

    /// Writes the annotation file to the disk.
    fn write_annotation_file(&self) {
        let mut file = File::create(format!(
            "{}/{}multi_use.rs",
            &self.response_output_folder, self.prefix
        ))
        .unwrap();
        file.write_all(
            format!(
                "#[derive(Deserialize, Debug)]\npub struct Envelope<T> {{\n\t#[serde(rename = \
                \"Body\")]\n\tpub body: Body<T>,\n}}\n\n#[derive(Deserialize, Debug)]\npub struct \
                Body<T> {{ {}\n\tpub response: T,\n}}",
                self.annotation_string.join("")
            )
            .as_bytes(),
        )
        .unwrap();
    }

    /// Writes all the response files to the disk.
    fn write_responses_files(&self) {
        for response_file in &self.response_files {
            let mut file = File::create(format!(
                "{}/{}{}.rs",
                &self.response_output_folder, self.prefix, response_file.name
            ))
            .unwrap();
            file.write_all(response_file.content.join("").as_bytes())
                .unwrap();
        }
    }

    /// Writes the mod.rs files into the two folders.
    fn write_mod_files(&self) {
        let mut file = File::create(format!("{}/mod.rs", &self.response_output_folder)).unwrap();
        let mut file_name_vec = vec![format!("pub mod {}multi_use;\n", self.prefix)];
        for response_file in &self.response_files {
            file_name_vec.push(format!("pub mod {}{};\n", self.prefix, response_file.name));
        }
        file_name_vec.sort();
        file_name_vec.dedup();
        file.write_all(file_name_vec.join("").as_bytes()).unwrap();

        file_name_vec.clear();
        let mut file = File::create(format!("{}/mod.rs", &self.request_output_folder)).unwrap();
        for request_file in &self.request_files {
            file_name_vec.push(format!("pub mod {}{};\n", self.prefix, request_file.name));
        }
        file_name_vec.sort();
        file_name_vec.dedup();
        file.write_all(file_name_vec.join("").as_bytes()).unwrap();
    }

    /// Writes all request files to disk.
    fn write_requests_files(&self) {
        for request_file in &self.request_files {
            let mut file = File::create(format!(
                "{}/{}{}.rs",
                &self.request_output_folder, self.prefix, request_file.name
            ))
            .unwrap();
            for function in &request_file.request_functions {
                file.write_all(function.create_function().as_bytes())
                    .unwrap();
            }
        }
    }
}

impl ApiDescDir {
    /// Generates request and response files from the FritzBox TR-064 API.
    pub fn generate_files(
        &self,
        address: &str,
        responses_output_folder: String,
        request_output_folder: String,
        prefix: Option<String>,
    ) {
        let mut output_files = OutputFiles::new();
        let prefix = if prefix.is_some() {
            format!("{}_", prefix.unwrap())
        } else {
            String::from("")
        };
        output_files.response_output_folder =
            format!("output/{}{}", prefix, responses_output_folder);
        output_files.request_output_folder = format!("output/{}{}", prefix, request_output_folder);
        output_files.prefix = prefix;

        self.handle_device(&self.device, address, &mut output_files);

        output_files.create_files();
    }

    /// Handles all services of a device and all contained devices.
    fn handle_device(&self, device: &Device, address: &str, mut output_files: &mut OutputFiles) {
        for service in &device.service_list.service {
            let resp = reqwest::blocking::get(format!("{}{}", address, service.scpd_url).as_str())
                .unwrap()
                .text()
                .unwrap();
            let scdp: ApiDesc = serde_xml_rs::from_str(&*resp).unwrap();
            service.service_type.split(':').nth(3).unwrap();
            scdp.fill_output_files(
                &mut output_files,
                service.service_type.split(':').nth(3).unwrap(),
                &service.control_url,
                &service.service_type,
            );
        }
        for local_device in &device.device_list.device {
            self.handle_device(local_device, address, &mut output_files);
        }
    }
}
