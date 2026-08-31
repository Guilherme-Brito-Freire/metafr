pub struct Param{
    param_name:String,
    value:String
}

impl Param {
    pub fn get_param(&self) -> String {
        return format!("{}='{}'",&self.param_name,&self.value); // this format the param to Html!
    }
}

// This function create the param in the memory and copy the rodata to RAM
pub fn create_param(param_name: &str, value: &str) -> Param {
    Param { param_name: param_name.to_string(), value: value.to_string() }
}

// This function create the param but recive data form the Heap, and don't process the string
pub fn create_param_from_string(param_name: String, value: String) -> Param {
    Param { param_name: param_name, value: value }
}

pub fn get_param_list(params:&Vec<Param>) -> String{
    
    let mut param_list: String = "".to_string();
    
    for item in params {
        param_list.push_str(&format!(" {} ", &item.get_param()));
    }

    return param_list;
}