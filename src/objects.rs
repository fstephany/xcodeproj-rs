use serde::Deserialize;
use std::collections::HashMap;

/// Represents a .pbxproj file
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PBXProj {
    archive_version: usize,
    object_version: usize,

    /// Root object references
    root_object: String,
    objects: HashMap<String, PBXObject>,
}

#[derive(Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
#[serde(tag = "isa")]
pub enum PBXObject {
    #[serde(rename_all = "camelCase")]
    PBXBuildFile {
        file_ref: String
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

    PBXProject,
    PBXTargetDependency,
    XCBuildConfiguration,
    XCConfigurationList,
}

// children 	List 	A list of element reference 	The objects are a reference to a PBXFileElement element.
// name 	String 	The filename. 	
// sourceTree 	String 	See the PBXSourceTree enumeration.

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PBXGroup {
    name: Option<String>,
    path: Option<String>,

    /// References to PBXFileElement elements
    children: Vec<String>,
    source_tree: PBXSourceTree,
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
    name: String,
    product_name: String,
    product_install_path: Option<String>,
    /// reference
    build_configuration_list: String, 
}