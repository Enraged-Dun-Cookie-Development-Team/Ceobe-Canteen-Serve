
crate::quick_struct!{
    pub PlatformAndDatasourceArray {
        platform_list: Vec<String>
        datasource_list: Vec<String>
    }

    pub DatasourceListFilterCond {
        platform: Option<String>
        datasource: Option<String>
    }
}