fn hidden_forum_inputs(page: Html) -> HashMap<String, String> {
    let mut inputs: HashMap<String, String> = HashMap::new();

    let mut selector = Selector::parse("input").unwrap();
    let elements = page.select(&selector);

    for element in elements {
        let e = element.value();
        let _type = element.value().attr("type").unwrap_or_default();

        if !_type.eq("hidden") && !_type.eq("submit") {
            continue;
        }

        let identifier = e.attr("name").or(e.id());

        let identifier = match identifier {
            Some(value) => {value},
            None => {continue;}
        };

        let value = e.attr("value").unwrap_or_default();

        inputs.insert(identifier.to_string(), value.to_string());
    }

    inputs
}

fn request(url: String, hidden_inputs: HashMap<String, String>,  f_req: HashMap<String, String>){
    let mut data = hidden_inputs.clone();
    data.insert("pstMsg".to_owned(),"1".to_owned());
    data.insert("checkConnection".to_owned(),"youtube".to_owned());
    data.insert("checkedDomains".to_owned(),"youtube".to_owned());
    data.insert("hl".to_owned(),"en".to_owned());
    data.insert("deviceinfo".to_owned(), "[null,null,null,[],null,\"US\",null,null,[], \"GlifWebSignIn\",null,[null,null,[]]]".to_owned());
    data.insert("f.req".to_owned(), hashmap_to_str(f_req));
    data.insert("flowName".to_owned(),"GlifWebSignIn".to_owned());
    data.insert("flowEntry".to_owned(),"ServiceLogin".to_owned());

    print!("{:?}", data);
}

fn hashmap_to_str(map: HashMap<String, String>) -> String {
    let mut res = String::from("{");
    
    for (k, v) in map {
        res.push_str(format!("\"{}\":\"{}\",", k, v).as_str());
    }

    res.pop();
    res.push_str("}");

    res
}

