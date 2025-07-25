use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs,
    io::{self, Read, Seek, Write},
    path::{Path, PathBuf},
    vec,
};

use crate::{
    LivraisonResult,
    msi::features::environment_vars::{
        EnvironmentAction, EnvironmentActionKind, register_environment_vars,
    },
};
use msi::Language;
use msi_installer::{
    PropertiesBuilder, RequiredProperties,
    tables::{
        Binary, Component, ComponentAttributes, Directory, Entity, FeatureComponents, File,
        FileAttributes,
    },
};
use uuid::Uuid;

use super::{Context, dialogs::minimalist};

// Namespace to construct uuid v5
const UUID_NAMESPACE: Uuid = uuid::uuid!("3941a426-8f68-469a-a7c5-99944d6067d8");

// Don't add more files to a cabinet folder that already has this many bytes:
const CABINET_FOLDER_SIZE_LIMIT: u64 = 0x8000;
// The maximum number of resource files we'll put in one cabinet:
const CABINET_MAX_FILES: usize = 1000;
// The maximum number of data bytes we'll put in one cabinet:
const CABINET_MAX_SIZE: u64 = 0x1000_0000;

// The name of the installer package's sole Feature:
const MAIN_FEATURE_NAME: &str = "MainFeature";

#[derive(Default, Clone, Debug)]
pub struct MsiInstallerOptions {
    /// Unique name that should never change to generate the same UUID
    pub bundle_name: String,

    /// Application name
    pub name: String,

    /// Application version
    pub version: String,

    /// Unique name that should never change to generate the same UUID
    pub description: String,

    /// Author name
    pub author: String,

    /// Icon path
    pub icon: Option<String>,
    pub binaries: Option<Vec<BinaryFile>>,
}

#[derive(Default, Clone, Debug)]
pub struct BinaryFile {
    pub name: String,
    pub path: PathBuf,
}

pub struct MsiInstallerPacker<W: Read + Write + Seek> {
    package: msi::Package<W>,
    options: MsiInstallerOptions,
    context: Context,
}

pub fn pack(options: MsiInstallerOptions, dest: &Path) -> LivraisonResult<()> {
    let file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(dest)?;

    let mut packer = MsiInstallerPacker::new(file, options)?;

    packer.write()?;
    Ok(())
}

impl<W: Read + Write + Seek> MsiInstallerPacker<W> {
    pub fn new(out: W, options: MsiInstallerOptions) -> LivraisonResult<MsiInstallerPacker<W>> {
        let package = msi::Package::create(msi::PackageType::Installer, out)?;
        let product_code = compute_product_code(&options.bundle_name, &options.version);
        let upgrade_code = compute_upgrade_code(&options.bundle_name);

        Ok(MsiInstallerPacker {
            package,
            options,
            context: Context {
                product_code,
                upgrade_code,
            },
        })
    }

    pub fn write(&mut self) -> LivraisonResult<()> {
        self.set_summary_info();
        self.package.flush()?;
        self.create_property_table()?;

        // Copy resource files into package:
        let mut resources = self.collect_resource_info()?;
        let directories = self.collect_directory_info(&mut resources)?;
        let cabinets = self.divide_resources_into_cabinets(resources);
        self.generate_resource_cabinets(&cabinets)?;

        FeatureComponents::create_table(&mut self.package)?;
        Binary::create_table(&mut self.package)?;

        self.add_binary_data("ClassicImage", include_bytes!("./assets/classic_bg.jpg"))?;
        // Set up installer database tables:
        self.create_directory_table(&directories)?;
        self.create_feature_table()?;
        self.create_component_table(&directories)?;
        self.create_feature_components_table(&directories)?;
        self.create_media_table(&cabinets)?;
        self.create_file_table(&cabinets)?;
        self.create_install_execute_sequence_table(&cabinets)?;
        minimalist::create().insert(&mut self.package)?;

        register_environment_vars(
            &mut self.package,
            &self.context,
            &vec![
                EnvironmentAction {
                    id: "PATH".to_string(),
                    name: "PATH".to_string(),
                    value: "[INSTALLDIR]".to_string(),
                    kind: EnvironmentActionKind::Append,
                },
                // EnvironmentAction {
                //     id: "TEST_VAR".to_string(),
                //     name: "TEST_VAR".to_string(),
                //     value: "[INSTALLDIR]".to_string(),
                //     kind: EnvironmentActionKind::Set,
                // },
            ],
        )?;

        self.package.flush()?;
        Ok(())
    }

    // Populates the summary metadata for the package from the bundle settings.
    fn set_summary_info(&mut self) {
        let summary_info = self.package.summary_info_mut();
        summary_info.set_codepage(msi::CodePage::Iso88591);
        summary_info.set_creation_time_to_now();
        summary_info.set_arch("x64");
        summary_info.set_languages(&[msi::Language::from_tag("en-US")]);
        summary_info.set_subject(&self.options.name);
        summary_info.set_uuid(self.context.product_code);
        summary_info.set_comments(&self.options.description);
        summary_info.set_author(&self.options.author);
        summary_info.set_creating_application("livraison".to_string());
        summary_info.set_word_count(10);
    }

    // Creates and populates the `Property` database table for the package.
    fn create_property_table(&mut self) -> LivraisonResult<()> {
        PropertiesBuilder::new(RequiredProperties {
            product_code: self.context.product_code,
            product_language: Language::from_tag("en-US"),
            manufacturer: self.options.author.clone(),
            product_name: self.options.name.clone(),
            product_version: self.options.version.clone(),
        })
        .upgrade_code(&self.context.upgrade_code)
        .install_per_user()
        .default_ui_font("DefaultFont")
        .insert("Mode", "Install")
        .insert("Text_action", "installation")
        .insert("Text_agent", "installer")
        .insert("Text_Doing", "installing")
        .insert("Text_done", "installed")
        .create_table(&mut self.package)?;

        Ok(())
    }

    /// Add a binary file to use in the installer.
    fn add_binary_data(&mut self, name: &str, data: &[u8]) -> LivraisonResult<()> {
        let row = Binary {
            binary: name.to_string(),
        };

        let writer = self.package.write_stream(&row.stream_name());
        writer?.write_all(data)?;
        Binary::insert(&mut self.package, &[row])?;
        Ok(())
    }

    // Returns a list of `ResourceInfo` structs for the binary executable and all
    // the resource files that should be included in the package.
    fn collect_resource_info(&self) -> LivraisonResult<Vec<ResourceInfo>> {
        let mut resources = Vec::<ResourceInfo>::new();
        if let Some(binaries) = &self.options.binaries {
            for binary in binaries {
                let metadata = fs::metadata(&binary.path)?;
                resources.push(ResourceInfo {
                    source_path: PathBuf::from(&binary.path),
                    dest_path: PathBuf::from(&binary.name),
                    filename: binary.name.clone(),
                    size: metadata.len(),
                    component_key: String::new(),
                });
            }
        }

        // let root_rsrc_dir = PathBuf::from("Resources");
        // for source_path in settings.resource_files() {
        //     let source_path = source_path?;
        //     let metadata = source_path.metadata()?;
        //     let size = metadata.len();
        //     let dest_path = root_rsrc_dir.join(common::resource_relpath(&source_path));
        //     let filename = dest_path.file_name().unwrap().to_string_lossy().to_string();
        //     let info = ResourceInfo {
        //         source_path,
        //         dest_path,
        //         filename,
        //         size,
        //         component_key: String::new(),
        //     };
        //     resources.push(info);
        // }
        Ok(resources)
    }

    // Based on the list of all resource files to be bundled, returns a list of
    // all the directories that need to be created during installation.  Also,
    // modifies each `ResourceInfo` object to populate its `component_key` field
    // with the database key of the Component that the resource will be associated
    // with.
    fn collect_directory_info(
        &self,
        resources: &mut [ResourceInfo],
    ) -> LivraisonResult<Vec<DirectoryInfo>> {
        let mut dir_map = BTreeMap::<PathBuf, DirectoryInfo>::new();
        let mut dir_index: i32 = 0;
        dir_map.insert(
            PathBuf::new(),
            DirectoryInfo {
                key: "INSTALLDIR".to_string(),
                parent_key: "ProgramFilesFolder".to_string(),
                name: self.options.name.to_string(),
                files: Vec::new(),
            },
        );
        for resource in resources.iter_mut() {
            let mut dir_key = "INSTALLDIR".to_string();
            let mut dir_path = PathBuf::new();
            for component in resource.dest_path.parent().unwrap().components() {
                if let std::path::Component::Normal(name) = component {
                    dir_path.push(name);
                    if dir_map.contains_key(&dir_path) {
                        dir_key.clone_from(&dir_map.get(&dir_path).unwrap().key);
                    } else {
                        let new_key = format!("RDIR{dir_index:04}");
                        dir_map.insert(
                            dir_path.clone(),
                            DirectoryInfo {
                                key: new_key.clone(),
                                parent_key: dir_key.clone(),
                                name: name.to_string_lossy().to_string(),
                                files: Vec::new(),
                            },
                        );
                        dir_key = new_key;
                        dir_index += 1;
                    }
                }
            }
            let directory = dir_map.get_mut(&dir_path).unwrap();
            debug_assert_eq!(directory.key, dir_key);
            directory.files.push(resource.filename.clone());
            resource.component_key = resource.filename.to_string();
        }
        Ok(dir_map.into_values().collect())
    }

    // Divides up the list of resource into some number of cabinets, subject to a
    // few constraints: 1) no one cabinet will have two resources with the same
    // filename, 2) no one cabinet will have more than `CABINET_MAX_FILES` files
    // in it, and 3) no one cabinet will contain more than `CABINET_MAX_SIZE`
    // bytes of data (unless that cabinet consists of a single file that is
    // already bigger than that).
    fn divide_resources_into_cabinets(&self, mut resources: Vec<ResourceInfo>) -> Vec<CabinetInfo> {
        let mut cabinets = Vec::new();
        while !resources.is_empty() {
            let mut filenames = HashSet::<String>::new();
            let mut total_size = 0;
            let mut leftovers = Vec::<ResourceInfo>::new();
            let mut cabinet = CabinetInfo {
                name: format!("cab{}.cab", cabinets.len() + 1),
                resources: Vec::new(),
            };
            for resource in resources.into_iter() {
                if cabinet.resources.len() >= CABINET_MAX_FILES
                    || (!cabinet.resources.is_empty()
                        && total_size + resource.size > CABINET_MAX_SIZE)
                    || filenames.contains(&resource.filename)
                {
                    leftovers.push(resource);
                } else {
                    filenames.insert(resource.filename.clone());
                    total_size += resource.size;
                    cabinet.resources.push(resource);
                }
            }
            cabinets.push(cabinet);
            resources = leftovers;
        }
        cabinets
    }

    // Creates the CAB archives within the package that contain the binary
    // executable and all the resource files.
    fn generate_resource_cabinets(&mut self, cabinets: &[CabinetInfo]) -> LivraisonResult<()> {
        for cabinet_info in cabinets.iter() {
            let mut builder = cab::CabinetBuilder::new();
            let mut file_map = HashMap::<String, &Path>::new();
            let mut resource_index: usize = 0;
            while resource_index < cabinet_info.resources.len() {
                let folder = builder.add_folder(cab::CompressionType::MsZip);
                let mut folder_size: u64 = 0;
                while resource_index < cabinet_info.resources.len()
                    && folder_size < CABINET_FOLDER_SIZE_LIMIT
                {
                    let resource = &cabinet_info.resources[resource_index];
                    folder_size += resource.size;
                    folder.add_file(resource.filename.as_str());
                    debug_assert!(!file_map.contains_key(&resource.filename));
                    file_map.insert(resource.filename.clone(), &resource.source_path);
                    resource_index += 1;
                }
            }
            let stream = self.package.write_stream(cabinet_info.name.as_str())?;
            let mut cabinet_writer = builder.build(stream)?;
            while let Some(mut file_writer) = cabinet_writer.next_file()? {
                debug_assert!(file_map.contains_key(file_writer.file_name()));
                let file_path = file_map.get(file_writer.file_name()).unwrap();
                let mut file = fs::File::open(file_path)?;
                io::copy(&mut file, &mut file_writer)?;
            }
            cabinet_writer.finish()?;
        }
        Ok(())
    }

    // Creates and populates the `Directory` database table for the package.
    fn create_directory_table(&mut self, user_dirs: &[DirectoryInfo]) -> LivraisonResult<()> {
        Directory::create_table(&mut self.package)?;

        let mut dirs = vec![
            Directory {
                directory: "TARGETDIR".to_string(),
                parent: None,
                default_dir: "SourceDir".to_string(),
            },
            Directory {
                directory: "ProgramFilesFolder".to_string(),
                parent: Some("TARGETDIR".to_string()),
                default_dir: "Program Files".to_string(),
            },
        ];

        for dir in user_dirs.iter() {
            dirs.push(Directory {
                directory: dir.key.clone(),
                parent: Some(dir.parent_key.clone()),
                default_dir: dir.name.clone(),
            });
        }

        Directory::insert(&mut self.package, &dirs)?;

        Ok(())
    }

    // Creates and populates the `Feature` database table for the package.  The
    // package will have a single main feature that installs everything.
    fn create_feature_table(&mut self) -> LivraisonResult<()> {
        self.package.create_table(
            "Feature",
            vec![
                msi::Column::build("Feature").primary_key().id_string(38),
                msi::Column::build("Feature_Parent")
                    .nullable()
                    .foreign_key("Feature", 1)
                    .id_string(38),
                msi::Column::build("Title").nullable().text_string(64),
                msi::Column::build("Description")
                    .nullable()
                    .text_string(255),
                msi::Column::build("Display")
                    .nullable()
                    .range(0, 0x7fff)
                    .int16(),
                msi::Column::build("Level").range(0, 0x7fff).int16(),
                msi::Column::build("Directory_")
                    .nullable()
                    .foreign_key("Directory", 1)
                    .id_string(72),
                msi::Column::build("Attributes").int16(),
            ],
        )?;
        self.package
            .insert_rows(msi::Insert::into("Feature").row(vec![
                msi::Value::from(MAIN_FEATURE_NAME),
                msi::Value::Null,
                msi::Value::from(self.options.name.clone()),
                msi::Value::Null,
                msi::Value::Int(1),
                msi::Value::Int(1),
                msi::Value::from("INSTALLDIR"),
                msi::Value::Int(24),
            ]))?;
        Ok(())
    }

    // Creates and populates the `Component` database table for the package.  One
    // component is created for each subdirectory under in the install dir.
    fn create_component_table(&mut self, directories: &[DirectoryInfo]) -> LivraisonResult<()> {
        Component::create_table(&mut self.package)?;
        let mut rows = Vec::new();
        for directory in directories.iter() {
            for file in directory.files.iter() {
                let uuid = Uuid::new_v5(&self.context.product_code, file.as_bytes());
                rows.push(Component {
                    component: file.clone(),
                    id: Some(uuid),
                    directory: directory.key.clone(),
                    attributes: ComponentAttributes::empty(),
                    condition: None,
                    key_path: Some(file.clone()),
                });
            }
        }
        Component::insert(&mut self.package, &rows)?;
        Ok(())
    }

    // Creates and populates the `FeatureComponents` database table for the
    // package.  All components are added to the package's single main feature.
    fn create_feature_components_table(
        &mut self,
        directories: &[DirectoryInfo],
    ) -> LivraisonResult<()> {
        let mut rows = Vec::new();
        for directory in directories.iter() {
            for file in directory.files.iter() {
                rows.push(vec![
                    msi::Value::from(MAIN_FEATURE_NAME),
                    msi::Value::Str(file.clone()),
                ]);
            }
        }
        self.package
            .insert_rows(msi::Insert::into("FeatureComponents").rows(rows))?;
        Ok(())
    }

    // Creates and populates the `Media` database table for the package, with one
    // entry for each CAB archive within the package.
    fn create_media_table(&mut self, cabinets: &[CabinetInfo]) -> LivraisonResult<()> {
        self.package.create_table(
            "Media",
            vec![
                msi::Column::build("DiskId")
                    .primary_key()
                    .range(1, 0x7fff)
                    .int16(),
                msi::Column::build("LastSequence").range(0, 0x7fff).int16(),
                msi::Column::build("DiskPrompt").nullable().text_string(64),
                msi::Column::build("Cabinet")
                    .nullable()
                    .category(msi::Category::Cabinet)
                    .string(255),
                msi::Column::build("VolumeLabel").nullable().text_string(32),
                msi::Column::build("Source")
                    .nullable()
                    .category(msi::Category::Property)
                    .string(32),
            ],
        )?;
        let mut disk_id: i32 = 0;
        let mut last_seq: i32 = 0;
        let mut rows = Vec::new();
        for cabinet in cabinets.iter() {
            disk_id += 1;
            last_seq += cabinet.resources.len() as i32;
            rows.push(vec![
                msi::Value::Int(disk_id),
                msi::Value::Int(last_seq),
                msi::Value::Null,
                msi::Value::Str(format!("#{}", cabinet.name)),
                msi::Value::Null,
                msi::Value::Null,
            ]);
        }
        self.package
            .insert_rows(msi::Insert::into("Media").rows(rows))?;
        Ok(())
    }

    // Creates and populates the `File` database table for the package, with one
    // entry for each resource file to be installed (including the main
    // executable).
    fn create_file_table(&mut self, cabinets: &[CabinetInfo]) -> LivraisonResult<()> {
        File::create_table(&mut self.package)?;
        let mut rows = Vec::new();
        let mut sequence: i32 = 1;
        for cabinet in cabinets.iter() {
            for resource in cabinet.resources.iter() {
                rows.push(File {
                    file: resource.filename.clone(),
                    component: resource.component_key.clone(),
                    filename: resource.filename.clone(),
                    size: resource.size as i32,
                    sequence,
                    attributes: FileAttributes::Vital,
                    version: None,
                    language: None,
                });
                sequence += 1;
            }
        }

        File::insert(&mut self.package, &rows)?;
        Ok(())
    }

    fn create_install_execute_sequence_table(
        &mut self,
        _cabinets: &[CabinetInfo],
    ) -> LivraisonResult<()> {
        self.package.create_table(
            "InstallExecuteSequence",
            vec![
                msi::Column::build("Action").primary_key().id_string(72),
                msi::Column::build("Condition")
                    .nullable()
                    .category(msi::Category::Condition)
                    .string(255),
                msi::Column::build("Sequence")
                    .nullable()
                    .range(-4, 0x7fff)
                    .int16(),
            ],
        )?;
        let mut rows = Vec::new();
        let actions: [(&str, &str, i32); 28] = [
            //("LaunchConditions", "", 100), // Requires a LaunchCondition table
            //("FindRelatedProducts", "", 200), // Requires an Upgrade table
            //("AppSearch", "", 400), // Requires a Signature table
            //("CCPSearch", "NOT Installed", 500), // Requires a Signature or *Locator table
            //("RMCCPSearch", "NOT Installed", 600), // Requires the CCP_DRIVE property and a DrLocator table
            ("ValidateProductID", "", 700),
            ("CostInitialize", "", 800),
            ("FileCost", "", 900),
            ("CostFinalize", "", 1000),
            ("SetODBCFolders", "", 1100),
            //("MigrateFeatureStates", "", 1200),
            ("InstallValidate", "", 1400),
            ("InstallInitialize", "", 1500),
            ("AllocateRegistrySpace", "NOT Installed", 1550),
            ("ProcessComponents", "", 1600),
            ("UnpublishComponents", "", 1700),
            ("UnpublishFeatures", "", 1800),
            //("StopServices", "VersionNT", 1900), // Requires a ServiceControl table
            //("DeleteServices", "VersionNT", 2000), // Requires a ServiceControl table
            ("UnregisterComPlus", "", 2100),
            //("SelfUnregModules", "", 2200), // Requires a SelfReg table
            //("UnregisterTypeLibraries", "", 2300), // Requires a TypeLib table
            //("RemoveODBC", "", 2400), // Requires an ODBC* table
            //("UnregisterFonts", "", 2500), // Requires a Font table
            ("RemoveRegistryValues", "", 2600), // Requires a Registry table
            //("UnregisterClassInfo", "", 2700), // Requires a Class table
            //("UnregisterExtensionInfo", "", 2800), // Requires an Extension table
            //("UnregisterProgIdInfo", "", 2900), // Requires ProgId, Extension or Class table
            //("UnregisterMIMEInfo", "", 3000), // Requires a MIME table
            //("RemoveIniValues", "", 3100), // Requires an IniFile table
            //("RemoveShortcuts", "", 3200), // Requires a Shortcut table
            ("RemoveEnvironmentStrings", "", 3300), // Requires an Environment table
            //("RemoveDuplicateFiles", "", 3400), // Requires a DuplicateFile table
            ("RemoveFiles", "", 3500),
            ("RemoveFolders", "", 3600),
            ("CreateFolders", "", 3700),
            ("MoveFiles", "", 3800),
            ("InstallFiles", "", 4000),
            //("PatchFiles", "", 4090), // Requires a Patch table
            //("DuplicateFiles", "", 4210), // Requires a DuplicateFile table
            //("BindImage", "", 4300), // Requires a BindImage table
            //("CreateShortcuts", "", 4500), // Requires a Shortcut table
            //("RegisterClassInfo", "", 4600), // Requires a Class table
            //("RegisterExtensionInfo", "", 4700), // Requires an Extension table
            //("RegisterProgIdInfo", "", 4800), // Requires a ProgId table
            //("RegisterMIMEInfo", "", 4900), // Requires a MIME table
            ("WriteRegistryValues", "", 5000), // Requires a Registry table
            //("WriteIniValues", "", 5100), // Requires an IniFile table
            ("WriteEnvironmentStrings", "", 5200), // Requires an Environment table
            //("RegisterFonts", "", 5300), // Requires a Font table
            //("InstallODBC", "", 5400), // Requires an ODBC* table
            //("RegisterTypeLibraries", "", 5500), // Requires a TypeLib table
            //("SelfRegModules", "", 5600), // Requires a SelfReg table
            ("RegisterComPlus", "", 5700),
            //("InstallServices", "VersionNT", 5800), // Requires a ServiceInstall table
            //("StartServices", "VersionNT", 5900), // Requires a SelfReg ServiceControl
            ("RegisterUser", "", 6000),
            ("RegisterProduct", "", 6100),
            ("PublishComponents", "", 6200),
            ("PublishFeatures", "", 6300),
            ("PublishProduct", "", 6400),
            ("InstallFinalize", "", 6600),
            //("RemoveExistingProducts", "", 6700), // Requires an Upgrade table
        ];
        for action in actions {
            rows.push(vec![
                msi::Value::Str(action.0.to_string()),
                if !action.1.is_empty() {
                    msi::Value::Str(action.1.to_string())
                } else {
                    msi::Value::Null
                },
                msi::Value::Int(action.2),
            ]);
        }
        self.package
            .insert_rows(msi::Insert::into("InstallExecuteSequence").rows(rows))?;
        Ok(())
    }
}

fn compute_upgrade_code(bundle_name: &str) -> uuid::Uuid {
    Uuid::new_v5(&UUID_NAMESPACE, bundle_name.as_bytes())
}

fn compute_product_code(bundle_name: &str, version: &str) -> uuid::Uuid {
    let str = format!("{bundle_name}@{version}");
    Uuid::new_v5(&UUID_NAMESPACE, str.as_bytes())
}

// Info about a resource file (including the main executable) in the bundle.
struct ResourceInfo {
    // The path to the existing file that will be bundled as a resource.
    source_path: PathBuf,
    // Relative path from the install dir where this will be installed.
    dest_path: PathBuf,
    // The name of this resource file in the filesystem.
    filename: String,
    // The size of this resource file, in bytes.
    size: u64,
    // The database key for the Component that this resource is part of.
    component_key: String,
}

// Info about a directory that needs to be created during installation.
struct DirectoryInfo {
    // The database key for this directory.
    key: String,
    // The database key for this directory's parent.
    parent_key: String,
    // The name of this directory in the filesystem.
    name: String,
    // List of files in this directory, not counting subdirectories.
    files: Vec<String>,
}

// Info about a CAB archive within the installer package.
struct CabinetInfo {
    // The stream name for this cabinet.
    name: String,
    // The resource files that are in this cabinet.
    resources: Vec<ResourceInfo>,
}
