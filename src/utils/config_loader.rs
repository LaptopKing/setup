pub struct Service {
    pub command: String,
}

pub struct Project {
    pub name: String,
    pub working_directory: String,
    pub services: Vec<Service>,
}

pub fn projects_config_loader() -> Vec<Project> {
    vec![
        Project {
            name: String::from("test"),
            working_directory: String::from("test_dir"),
            services: Vec::new(),
        },
        Project {
            name: String::from("hello"),
            working_directory: String::from("hello_dir"),
            services: Vec::new(),
        },
        Project {
            name: String::from("ok"),
            working_directory: String::from("ok_dir"),
            services: Vec::new(),
        },
    ]
}
