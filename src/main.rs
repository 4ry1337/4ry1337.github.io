use ary1337::{
    config::get_config,
    error::SsgError,
    routes::{Routes, about::about, home::home},
};

fn main() -> Result<(), SsgError> {
    let config = get_config().expect("Failed to read configuration.");

    let routes = Routes::new(config)
        .route("/index", home())
        .route("/about", about());
    // let personal = Personal::from_file("content/1_personal.yaml")?;
    // let educations: Vec<Edcucation> = FromYaml::from_file("content/2_education.yaml")?;
    // let experences: Vec<Experience> = FromYaml::from_file("content/3_experience.yaml")?;
    // let projects: Vec<Project> = FromYaml::from_file("content/4_projects.yaml")?;
    // let skills = Skill::from_file("content/5_skills.yaml")?;
    // let contacts: Vec<Contact> = FromYaml::from_file("content/6_contacts.yaml")?;

    routes.build()?;

    Ok(())
}
