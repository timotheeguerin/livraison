/// https://learn.microsoft.com/en-us/windows/win32/msi/standard-actions
const STANDARD_ACTIONS: &[&str] = &[
    "ADMIN",                 // https://learn.microsoft.com/en-us/windows/win32/msi/admin-action
    "ADVERTISE",             // https://learn.microsoft.com/en-us/windows/win32/msi/advertise-action
    "AllocateRegistrySpace", // https://learn.microsoft.com/en-us/windows/win32/msi/allocateregistryspace-action
    "AppSearch",             // https://learn.microsoft.com/en-us/windows/win32/msi/appsearch-action
    "BindImage",             // https://learn.microsoft.com/en-us/windows/win32/msi/bindimage-action
    "CCPSearch",             // https://learn.microsoft.com/en-us/windows/win32/msi/ccpsearch-action
    "CostFinalize", // https://learn.microsoft.com/en-us/windows/win32/msi/costfinalize-action
    "CostInitialize", // https://learn.microsoft.com/en-us/windows/win32/msi/costinitialize-action
    "CreateFolders", // https://learn.microsoft.com/en-us/windows/win32/msi/createfolders-action
    "CreateShortcuts", // https://learn.microsoft.com/en-us/windows/win32/msi/createshortcuts-action
    "DeleteServices", // https://learn.microsoft.com/en-us/windows/win32/msi/deleteservices-action
    "DuplicateFiles", // https://learn.microsoft.com/en-us/windows/win32/msi/duplicatefiles-action
    "ExecuteAction", // https://learn.microsoft.com/en-us/windows/win32/msi/executeaction-action
    "FileCost",     // https://learn.microsoft.com/en-us/windows/win32/msi/filecost-action
    "FindRelatedProducts", // https://learn.microsoft.com/en-us/windows/win32/msi/findrelatedproducts-action
    "ForceReboot",         // https://learn.microsoft.com/en-us/windows/win32/msi/forcereboot-action
    "InstallAdminPackage", // https://learn.microsoft.com/en-us/windows/win32/msi/installadminpackage-action
    "InstallFiles", // https://learn.microsoft.com/en-us/windows/win32/msi/installfiles-action
    "InstallFinalize", // https://learn.microsoft.com/en-us/windows/win32/msi/installfinalize-action
    "InstallInitialize", // https://learn.microsoft.com/en-us/windows/win32/msi/installinitialize-action
    "InstallODBC",       // https://learn.microsoft.com/en-us/windows/win32/msi/installodbc-action
    "InstallServices", // https://learn.microsoft.com/en-us/windows/win32/msi/installservices-action
    "IsolateComponents", // https://learn.microsoft.com/en-us/windows/win32/msi/isolatecomponents-action
    "LaunchConditions", // https://learn.microsoft.com/en-us/windows/win32/msi/launchconditions-action
    "MigrateFeatureStates", // https://learn.microsoft.com/en-us/windows/win32/msi/migratefeaturestates-action
    "MoveFiles",            // https://learn.microsoft.com/en-us/windows/win32/msi/movefiles-action
    "PatchFiles",           // https://learn.microsoft.com/en-us/windows/win32/msi/patchfiles-action
    "ProcessComponents", // https://learn.microsoft.com/en-us/windows/win32/msi/processcomponents-action
    "PublishComponents", // https://learn.microsoft.com/en-us/windows/win32/msi/publishcomponents-action
    "PublishFeatures", // https://learn.microsoft.com/en-us/windows/win32/msi/publishfeatures-action
    "PublishProduct",  // https://learn.microsoft.com/en-us/windows/win32/msi/publishproduct-action
    "RegisterClassInfo", // https://learn.microsoft.com/en-us/windows/win32/msi/registerclassinfo-action
    "RegisterComPlus", // https://learn.microsoft.com/en-us/windows/win32/msi/registercomplus-action
    "RegisterFonts",   // https://learn.microsoft.com/en-us/windows/win32/msi/registerfonts-action
    "RegisterMIMEInfo", // https://learn.microsoft.com/en-us/windows/win32/msi/registermimeinfo-action
    "RegisterProduct", // https://learn.microsoft.com/en-us/windows/win32/msi/registerproduct-action
    "RegisterTypeLibraries", // https://learn.microsoft.com/en-us/windows/win32/msi/registertypelibraries-action
    "RegisterUser", // https://learn.microsoft.com/en-us/windows/win32/msi/registeruser-action
    "RemoveDuplicateFiles", // https://learn.microsoft.com/en-us/windows/win32/msi/removeduplicatefiles-action
    "RemoveEnvironmentStrings", // https://learn.microsoft.com/en-us/windows/win32/msi/removeenvironmentstrings-action
    "RemoveExistingProducts", // https://learn.microsoft.com/en-us/windows/win32/msi/removeexistingproducts-action
    "RemoveFiles", // https://learn.microsoft.com/en-us/windows/win32/msi/removefiles-action
    "RemoveFolders", // https://learn.microsoft.com/en-us/windows/win32/msi/removefolders-action
    "RemoveIniValues", // https://learn.microsoft.com/en-us/windows/win32/msi/removeinivalues-action
    "RemoveODBC",  // https://learn.microsoft.com/en-us/windows/win32/msi/removeodbc-action
    "RemoveRegistryValues", // https://learn.microsoft.com/en-us/windows/win32/msi/removeregistryvalues-action
    "RemoveShortcuts", // https://learn.microsoft.com/en-us/windows/win32/msi/removeshortcuts-action
    "ResolveSource",   // https://learn.microsoft.com/en-us/windows/win32/msi/resolvesource-action
    "SelfRegModules",  // https://learn.microsoft.com/en-us/windows/win32/msi/selfregmodules-action
    // cspell:ignore Unreg
    "SelfUnregModules", // https://learn.microsoft.com/en-us/windows/win32/msi/selfunregmodules-action
    "SetODBCFolders",   // https://learn.microsoft.com/en-us/windows/win32/msi/setodbcfolders-action
    "StartServices",    // https://learn.microsoft.com/en-us/windows/win32/msi/startservices-action
    "StopServices",     // https://learn.microsoft.com/en-us/windows/win32/msi/stopservices-action
    "SystemFolder",     // https://learn.microsoft.com/en-us/windows/win32/msi/systemfolder-action
    "SystemSearch",     // https://learn.microsoft.com/en-us/windows/win32/msi/systemsearch-action
    "UnpublishComponents", // https://learn.microsoft.com/en-us/windows/win32/msi/unpublishcomponents-action
    "UnpublishFeatures", // https://learn.microsoft.com/en-us/windows/win32/msi/unpublishfeatures-action
    "UnregisterClassInfo", // https://learn.microsoft.com/en-us/windows/win32/msi/unregisterclassinfo-action
    "UnregisterComPlus", // https://learn.microsoft.com/en-us/windows/win32/msi/unregistercomplus-action
    "UnregisterFonts", // https://learn.microsoft.com/en-us/windows/win32/msi/unregisterfonts-action
    "UnregisterMIMEInfo", // https://learn.microsoft.com/en-us/windows/win32/msi/unregistermimeinfo-action
    "UnregisterTypeLibraries", // https://learn.microsoft.com/en-us/windows/win32/msi/unregistertypelibraries-action
    "ValidateProductID", // https://learn.microsoft.com/en-us/windows/win32/msi/validateproductid-action
    "WriteEnvironmentStrings", // https://learn.microsoft.com/en-us/windows/win32/msi/writeenvironmentstrings-action
    "WriteIniValues", // https://learn.microsoft.com/en-us/windows/win32/msi/writeinivalues-action
    "WriteRegistryValues", // https://learn.microsoft.com/en-us/windows/win32/msi/writeregistryvalues-action
];

pub fn is_standard_action(action: &str) -> bool {
    STANDARD_ACTIONS.contains(&action)
}
