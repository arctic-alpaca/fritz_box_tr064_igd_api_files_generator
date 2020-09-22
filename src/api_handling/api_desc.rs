use crate::api_handling::api_desc_dir::{
    OutputFiles, ParameterAndType, RequestFile, RequestFunction, ResponseFile, SpecVersion,
};

///Struct to deserialize response from "fritz.box/xyzSCPD.xml" into.
#[derive(Deserialize, Debug, Default)]
pub struct ApiDesc {
    #[serde(rename = "specVersion")]
    pub spec_version: SpecVersion,
    #[serde(rename = "actionList")]
    pub action_list: ActionList,
    #[serde(rename = "serviceStateTable")]
    pub service_state_table: ServiceStateTable,
}
///Struct to deserialize the ActionList part of the response from "fritz.box/xyzSCPD.xml" into.
#[derive(Deserialize, Debug, Default)]
pub struct ActionList {
    #[serde(rename = "action")]
    #[serde(default)]
    pub action: Vec<Action>,
}
///Struct to deserialize the Action part of the response from "fritz.box/xyzSCPD.xml" into.
#[derive(Deserialize, Debug, Default)]
pub struct Action {
    pub name: String,
    #[serde(rename = "argumentList")]
    #[serde(default)]
    pub argument_list: ArgumentList,
}
///Struct to deserialize the ArgumentList part of the response from "fritz.box/xyzSCPD.xml" into.
#[derive(Deserialize, Debug, Default)]
pub struct ArgumentList {
    #[serde(rename = "argument")]
    pub argument: Vec<Argument>,
}

///Struct to deserialize the Argument part of the response from "fritz.box/xyzSCPD.xml" into.
#[derive(Deserialize, Debug, Default)]
pub struct Argument {
    pub name: String,
    pub direction: String,
    #[serde(rename = "relatedStateVariable")]
    pub related_state_variable: String,
}

///Struct to deserialize the ServiceStateTable part of the response from "fritz.box/xyzSCPD.xml" into.
#[derive(Deserialize, Debug, Default)]
pub struct ServiceStateTable {
    #[serde(rename = "stateVariable")]
    pub state_variable: Vec<StateVariable>,
}

///Struct to deserialize the StateVariable part of the response from "fritz.box/xyzSCPD.xml" into.
#[derive(Deserialize, Debug, Default)]
pub struct StateVariable {
    pub name: String,
    #[serde(rename = "dataType")]
    pub data_type: String,
    #[serde(rename = "defaultValue")]
    #[serde(default)]
    pub default_value: String,
    #[serde(rename = "allowedValueList")]
    #[serde(default)]
    pub allowed_value_list: AllowedValueList,
}

///Struct to deserialize the AllowedValueList part of the response from "fritz.box/xyzSCPD.xml" into.
#[derive(Deserialize, Debug, Default)]
pub struct AllowedValueList {
    #[serde(rename = "allowedValue")]
    pub allowed_value: Vec<String>,
}

impl ApiDesc {
    ///Takes  an `OutputFiles`, `name`, `control_url` and `service_type` and populates the `OutputFiles`
    pub fn fill_output_files(
        &self,
        output_files: &mut OutputFiles,
        name: &str,
        control_url: &str,
        service_type: &str,
    ) {
        let rusty_name = self.rustify_string(&name);
        let mut response_file = ResponseFile::new();
        let mut request_file = RequestFile::new();
        response_file.name = rusty_name.clone();
        request_file.name = rusty_name;
        for action in &self.action_list.action {
            let mut request_function = RequestFunction::new();
            request_function.name = action.name.clone();
            request_function.name_rusty = self.rustify_string(action.name.as_str());
            request_function.action_name = action.name.clone();
            request_function.control_url = control_url.to_string();
            request_function.service_type = service_type.to_string();

            response_file
                .content
                .push(String::from("#[derive(Deserialize, Debug)]\n"));
            response_file.content.push(format!(
                "pub struct {}Response{{\n",
                action.name.replace("-", "").replace("_", "")
            ));
            output_files
                .annotation_string
                .push(format!("\n\t#[serde(alias = \"{}Response\")]", action.name));

            for argument in &action.argument_list.argument {
                if argument.direction == "out" {
                    response_file
                        .content
                        .push(format!("\t#[serde(rename = \"{}\")]\n", argument.name));
                    let variable_type =
                        self.search_state_variable_type(argument.related_state_variable.as_str());
                    let variable_name: String = self.rustify_string(&argument.name);
                    response_file
                        .content
                        .push(format!("\tpub {}: {},\n", variable_name, variable_type));
                } else if argument.direction == "in" {
                    let mut param = ParameterAndType::new();
                    param.parameter_name = argument.name.clone();
                    param.parameter_name_rusty = self.rustify_string(&argument.name);
                    param.type_name =
                        self.search_state_variable_type(argument.related_state_variable.as_str());
                    request_function.parameter.push(param);
                }
            }
            response_file.content.push("}\n\n".to_string());
            request_file.request_functions.push(request_function);
        }
        output_files.request_files.push(request_file);
        output_files.response_files.push(response_file);
    }

    /// Searches for the requested variable and returns the corresponding type.
    /// If you encounter a panic here, please open a ticket with the output of `_ => print!("{}", variable.data_type.as_str()),`
    fn search_state_variable_type(&self, state_variable_name: &str) -> String {
        for variable in &self.service_state_table.state_variable {
            if state_variable_name.eq(&variable.name) {
                match variable.data_type.as_str() {
                    "boolean" => return String::from("bool"),
                    "ui1" => return String::from("u32"),
                    "ui2" => return String::from("u32"),
                    "ui4" => return String::from("u32"),
                    "i1" => return String::from("i32"),
                    "i2" => return String::from("i32"),
                    "i4" => return String::from("i32"),
                    "string" => return String::from("String"),
                    "uuid" => return String::from("String"),
                    "dateTime" => return String::from("String"),
                    _ => print!("{}", variable.data_type.as_str()),
                };
            }
        }

        panic!("variable Type not implemented, please open a ticket")
    }

    /// Modifies the supplied `input` to generate proper snake case.
    fn rustify_string(&self, input: &str) -> String {
        input
            .replace("NewX_AVM-DE_", "newXAvmDe")
            .replace("NewX_AVM_DE_", "newXAvmDe")
            .replace("X_AVM-DE_", "XAvmDe")
            .replace("X_", "x")
            .replace("_", "")
            .replace("NATRSIP", "NatRsip")
            .replace("NAT", "Nat")
            .replace("RSIP", "Rsip")
            .replace("FCS", "Fcs")
            .replace("ATM", "Atm")
            .replace("DAV", "Dav")
            .replace("PPP", "Ppp")
            .replace("WAN", "Wan")
            .replace("MAC", "Mac")
            .replace("AIN", "Ain")
            .replace("DDNS", "Ddns")
            .replace("DNS", "Dns")
            .replace("IPTVo", "IptvO")
            .replace("IPTV", "Iptv")
            .replace("US", "Us")
            .replace("VoIP", "Voip")
            .replace("AVM", "Avm")
            .replace("URL", "Url")
            .replace("ATUC", "Atuc")
            .replace("CHECK", "Check")
            .replace("DSL", "Dsl")
            .replace("DS", "Ds")
            .replace("SNRG", "Snrg_")
            .replace("SNRMT", "Snrmt_")
            .replace("SNR", "Snr_")
            .replace("LATN", "Latn_")
            .replace("HEC", "Hec")
            .replace("TAM", "Tam")
            .replace("OKZ", "Okz")
            .replace("LKZ", "Lkz")
            .replace("OKZ", "Okz")
            .replace("STUN", "Stun")
            .replace("UPnP", "Upnp")
            .replace("FTP", "Ftp")
            .replace("SSL", "Ssl")
            .replace("SMB", "Smb")
            .replace("CGI", "Cgi")
            .replace("NTP", "Ntp")
            .replace("TR069", "Tr069")
            .replace("BSSID", "Bssid")
            .replace("SSID", "Ssid")
            .replace("SID", "Sid")
            .replace("UUID", "Uuid")
            .replace("OUI", "Oui")
            .replace("ATUR", "Atur")
            .replace("FEC", "Fec")
            .replace("CRC", "Crc")
            .replace("PSK", "Psk")
            .replace("WEP", "Wep")
            .replace("WPA", "Wpa")
            .replace("WLAN", "Wlan")
            .replace("LAN", "Lan")
            .replace("AP", "Ap")
            .replace("WPS", "Wps")
            .replace("RX", "Rx")
            .replace("WOL", "Wol")
            .replace("DHCP", "Dhcp")
            .replace("ID", "Id")
            .replace("IP", "Ip")
            .chars()
            .enumerate()
            .map(|x| {
                if x.1.is_uppercase() {
                    if x.0 == 0 {
                        x.1.to_lowercase().to_string()
                    } else {
                        format!("_{}", x.1.to_lowercase())
                    }
                } else {
                    x.1.to_string()
                }
            })
            .collect()
    }
}
