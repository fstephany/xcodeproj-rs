use plist;
use xcodeproj::objects::{PBXGroup, PBXObject, PBXProj};

fn print(level: u32, msg: &str) {
    let total_len = level as usize + msg.len();
    let mut to_print = String::with_capacity(total_len);

    for _i in 0..=level {
        to_print.push_str(" ");
    }
    to_print.push_str(msg);

    println!("{}", to_print);
}

fn print_group(proj: &PBXProj, group: &PBXGroup, indent: u32) {
    print(indent, &format!("{}", group));
    let child_indent = indent + 1;

    let mut iter = group.children.iter();
    while let Some(child_id) = iter.next() {
        match proj.get(child_id) {
            Some(PBXObject::PBXGroup(group)) => print_group(proj, group, child_indent),
            Some(something) => print(child_indent, &format!("{:?}", something)),
            None => print(child_indent, "Could not find child reference"),
        }
    }
}

fn main() {
    let proj: PBXProj = plist::from_file("./examples/data/Recordings.xcodeproj/project.pbxproj")
        .expect("failed to read project.pbxproj");

    // Walks through the main project and epxlore each group.
    let main_project = proj.root_project().expect("could not get main project");

    // Main Group ID: C9EB56B41EAE33C1000EC5F4
    let main_group = proj.get_group(&main_project.main_group).unwrap();
    println!("Main group children.len(): {}", main_group.children.len());
    print_group(&proj, main_group, 0);
}
