use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Display;
/// Represents a .pbxproj file
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PBXProj {
    pub archive_version: usize,
    pub object_version: usize,

    /// Root object reference
    root_object: String,
    objects: HashMap<String, PBXObject>,
}

impl PBXProj {
    pub fn groups(&self) -> Vec<&PBXGroup> {
        self.objects
            .iter()
            .filter_map(|(_id, object)| match object {
                PBXObject::PBXGroup(group) => Some(group),
                _ => None,
            })
            .collect()
    }

    /// All the targets contained in the pbxproj file.
    /// Beware that they might not be the ones from the root_project.
    pub fn targets(&self) -> Vec<&PBXNativeTarget> {
        self.objects
            .iter()
            .filter_map(|(_id, object)| match object {
                PBXObject::PBXNativeTarget(target) => Some(target),
                _ => None,
            })
            .collect()
    }

    pub fn get(&self, id: &str) -> Option<&PBXObject> {
        self.objects.get(id)
    }

    pub fn root_project(&self) -> Option<&PBXProject> {
        self.objects
            .get(&self.root_object)
            .and_then(|candidate| match candidate {
                PBXObject::PBXProject(project) => Some(project),
                _ => None,
            })
    }
}

#[derive(Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
#[serde(tag = "isa")]
pub enum PBXObject {
    #[serde(rename_all = "camelCase")]
    PBXBuildFile {
        file_ref: String,
    },
    PBXContainerItemProxy,

    PBXBuildPhase,
    PBXAppleScriptBuildPhase,
    PBXCopyFilesBuildPhase,
    PBXFrameworksBuildPhase,
    PBXHeadersBuildPhase,
    PBXResourcesBuildPhase,
    PBXShellScriptBuildPhase,
    PBXSourcesBuildPhase,

    PBXFileElement,
    PBXFileReference,
    PBXGroup(PBXGroup),
    PBXVariantGroup,

    // PBXTarget: abstract parent for specialized targets.
    PBXAggregateTarget,
    PBXLegacyTarget,
    PBXNativeTarget(PBXNativeTarget),

    PBXProject(PBXProject),
    PBXTargetDependency,
    XCBuildConfiguration,
    XCConfigurationList,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PBXProject {
    /// Reference to a XCConfigurationList element.
    pub build_configuration_list: String,
    /// Representation of the XcodeCompatibilityVersion.
    pub compatibility_version: String,
    /// The region of development.
    pub development_region: String,
    /// Whether file encodings have been scanned (0 or 1)
    pub has_scanned_for_encodings: usize,
    /// The known regions for localized files.
    pub known_regions: Vec<String>,
    /// Reference to a PBXGroup element.
    pub main_group: String,
    /// Reference to a PBXGroup element.
    pub product_ref_group: String,
    /// The relative path of the project.
    pub project_dir_path: String,
    /// String 	The relative root path of the project.
    pub project_root: String,
    /// References to PBXTargets.    
    pub targets: Vec<String>,
    // Array of map 	Each map in the array contains two keys: ProductGroup and ProjectRef.
    //pub project_references: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PBXGroup {
    pub name: Option<String>,
    pub path: Option<String>,

    /// References to PBXFileElement elements
    pub children: Vec<String>,
    pub source_tree: PBXSourceTree,
}

impl Display for PBXGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PBXGroup {{ ")?;

        if let Some(name) = &self.name {
            write!(f, "name: {} ", name)?;
        }

        if let Some(path) = &self.path {
            write!(f, "path: {} ", path)?;
        }

        write!(f, "sourceTree: {:?} ", self.source_tree)?;
        write!(f, "}}")
    }
}

#[derive(Deserialize, Debug)]
pub enum PBXSourceTree {
    #[serde(rename = ".")]
    None,

    #[serde(rename = "<absolute>")]
    Absolute,

    #[serde(rename = "<group>")]
    Group,

    #[serde(rename = "SOURCE_ROOT")]
    SourceRoot,

    #[serde(rename = "BUILT_PRODUCTS_DIR")]
    BuildProductsDir,

    #[serde(rename = "SDKROOT")]
    SdkRoot,

    #[serde(rename = "DEVELOPER_DIR")]
    DeveloperDir,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PBXNativeTarget {
    pub name: String,
    pub product_name: String,
    pub product_install_path: Option<String>,

    /// Examples:
    /// - "com.apple.product-type.application"
    /// - "com.apple.product-type.bundle.unit-test"
    pub product_type: String,
}
