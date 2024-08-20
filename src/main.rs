use ureq;

const COURSES_LINK: &str = "https://sis.rutgers.edu/soc/api/courses.json?year=2024&term=1&campus=NB";

const COURSE_SEARCH_CODE: &str = "01:013:140";

fn get_populated_courses() -> serde_json::Value {
    let json_response = ureq::get(COURSES_LINK).call().unwrap().into_json().unwrap();
    json_response
}

fn find_relevant_course() -> serde_json::Value {
    let r = get_populated_courses();
    let r = r.as_array().unwrap();
    let mut result: Option<&serde_json::Value> = None;
    for v in r {
        if v["courseString"] == COURSE_SEARCH_CODE {
            let title = &v["title"];
            println!("{title}");
            result = Some(v);
        }
    }
    result.unwrap().clone()
}

fn find_open_sections() {
    let relevant_course = find_relevant_course();
    let sections = &relevant_course["sections"];
    let sections = sections.as_array().unwrap();
    for section in sections {
        if section["openStatusText"] == "OPEN" {
            let section_index = section["index"].as_str().unwrap();
            println!("{section_index}");
        }
    }
}

fn main() {
    println!("rutgers course sniper or whatever");
    find_open_sections();
}
