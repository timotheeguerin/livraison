/// https://learn.microsoft.com/en-us/windows/win32/msi/standard-actions
const STANDARD_ACTIONS: &[&str] = &[
    "ADMIN",
    "ADVERTISE",
    "AllocateRegistrySpace",
    "AppSearch",
    "BindImage",
    "CCPSearch",
    "CostFinalize",
    "CostInitialize",
    "CreateFolders",
    "CreateShortcuts",
    "DeleteServices",
    "DuplicateFiles",
    "ExecuteAction",
    "FileCost",
    "FindRelatedProducts",
    "ForceReboot",
    "InstallAdminPackage",
    "InstallFiles",
    "InstallFinalize",
    "InstallInitialize",
    "InstallODBC",
    "InstallServices",
    "IsolateComponents",
    "LaunchConditions",
    "MigrateFeatureStates",
    "MoveFiles",
    "PatchFiles",
    "ProcessComponents",
    "PublishComponents",
    "PublishFeatures",
    "PublishProduct",
    "RegisterClassInfo",
    "RegisterComPlus",
    "RegisterFonts",
    "RegisterMIMEInfo",
    "RegisterProduct",
    "RegisterTypeLibraries",
    "RegisterUser",
    "RemoveDuplicateFiles",
    "RemoveEnvironmentStrings",
    "RemoveExistingProducts",
    "RemoveFiles",
    "RemoveFolders",
    "RemoveIniValues",
    "RemoveODBC",
    "RemoveRegistryValues",
    "RemoveShortcuts",
    "ResolveSource",
    "SelfRegModules",
    // cspell:ignore Unreg
    "SelfUnregModules",
    "SetODBCFolders",
    "StartServices",
    "StopServices",
    "SystemFolder",
    "SystemSearch",
    "UnpublishComponents",
    "UnpublishFeatures",
    "UnregisterClassInfo",
    "UnregisterComPlus",
    "UnregisterFonts",
    "UnregisterMIMEInfo",
    "UnregisterTypeLibraries",
    "ValidateProductID",
    "WriteEnvironmentStrings",
    "WriteIniValues",
    "WriteRegistryValues",
];

pub fn is_standard_action(action: &str) -> bool {
    STANDARD_ACTIONS.contains(&action)
}
