use serde::Serialize;


pub struct PageInfo {
    page: usize,
    size: usize,
    total_count: usize,
    total_page: usize,
}

/// 列表与分页信息
pub struct ListWithPageInfo<T: Serialize + IntoIterator> {
    list: T,
    page_size: PageInfo
}

pub trait GenerateListWithPageInfo {
    type ListType: Serialize + IntoIterator;
    fn generate_list_with_page_info(self, page: usize, size: usize, count: usize) -> ListWithPageInfo<Self::ListType>;
}


impl<T:Serialize + IntoIterator> GenerateListWithPageInfo for T {
    type ListType = T;
    /// 将列表，与分页信息存入一个结构体
    fn generate_list_with_page_info(self, page: usize, size: usize, count: usize) -> ListWithPageInfo<Self::ListType> {
        ListWithPageInfo {
            list: self,
            page_size: PageInfo {
                page,
                size,
                total_count: count,
                total_page: (count as f64 / size as f64).ceil() as usize,
            }
            
        }
    }
}
