use plist;
use xcodeproj::objects::{PBXObject, PBXProj};

fn main() {
    let proj: PBXProj = plist::from_file("./examples/data/Recordings.xcodeproj/project.pbxproj")
        .expect("failed to read project.pbxproj");

    println!("Targets:");
    println!("========");
    let main_project = proj.root_project().expect("could not get main project");
    let targets = main_project
        .targets
        .iter()
        .filter_map(|target_ref| match proj.get(target_ref) {
            Some(PBXObject::PBXNativeTarget(target)) => Some(target),
            Some(_) => None,
            None => None,
        });

    for target in targets {
        println!(
            "- {}, {} - {}",
            target.product_name, target.name, target.product_type
        );
    }

    // Top Level group is C9EB56B41EAE33C1000EC5F4
    println!("Groups:");
    println!("=======");
    for group in proj.groups().iter() {
        println!("- {}", group);
    }

    println!("Done");
}
