use plist;
use xcodeproj::objects::PBXProj;

fn main() {
    let project: PBXProj = plist::from_file("./examples/data/Recordings.xcodeproj/project.pbxproj")
        .expect("failed to read project.pbxproj");

    println!("{:?}", project);
    println!("Done");
}