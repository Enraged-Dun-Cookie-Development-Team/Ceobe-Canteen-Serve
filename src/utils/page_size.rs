pub struct PageSize {
    pub page: u64,
    pub size: u64,
    pub total_count: usize,
    pub total_page: u64,
}

pub struct ListPage<T> {
    pub list: T,
    pub page_size: PageSize,
}


/// 用于分页接口，附带page，size信息
/// 
/// 返回示例：
/// ```
/// data: {
///     list: [],
///     page_size: {
///         page: 1,
///         size: 10,
///         total_count: 98,
///         total_page: 10
///     }
/// }
/// ```
pub fn resp_with_page_info<T>(list:T, page: u64, size:u64, total_count: usize) -> ListPage<T> {
    ListPage {
        list,
        page_size: PageSize {
            page,
            size,
            total_count,
            total_page: (total_count as f64 / size as f64).ceil() as u64,
        },
    }
}