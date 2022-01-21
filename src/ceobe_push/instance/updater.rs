use std::collections::HashMap;

use dashmap::DashMap;

use crate::ceobe_push::dao::DataItem;
/// Updater 蹲饼器更新器
/// 内部使用`DashMap`保证 Sync+Send
/// # Usage
/// 
/// ---
/// 
/// ```rust [no test]
/// // 假设这是最新收到的蹲饼信息
/// let updater = Updater::default();
/// 
/// let newest: HashMap<String, Vec<DataItem>> = HashMap::default();
/// // new_dun 就是最新的蹲饼信息
/// let new_dun = updater.check_update(newest);
/// ```
///
#[derive(Default)]
pub struct Updater {
    last_id: DashMap<String, String>,
}

impl Updater {
    /// 检查更新
    /// 
    /// # Panic
    /// 
    /// ***对于每一`Vec<DataItem>`长度为0时将会panic***
    /// 
    /// # effect
    /// 
    /// 每次`check_update`将会更新内部的Updater
    /// 不提供内部可变
    /// 
    /// # return
    /// 
    /// 筛选后的更新的蹲饼消息
    pub fn check_update<'s>(
        &mut self,
        income: HashMap<String, Vec<DataItem>>,
    ) -> HashMap<String, Vec<DataItem>> {
        income
            .into_iter()
            .map(|(k, v)| (k.clone(), self.inner_checker(k, v)))
            .collect()
    }
    /// 内部检查更新，并更新 `last_id`
    fn inner_checker(&mut self, name: String, income: Vec<DataItem>) -> Vec<DataItem> {
        let new_id = income.get(0).unwrap().get_id().to_string();
        // 检擦是否为最新
        let res = if let Some(last_id) = self.last_id.get(&name) {
            Self::inner_check_update(&*last_id, income).unwrap_or_default()
        } else {
            income
        };
        // 更新
        self.last_id.insert(name, new_id);

        res
    }

    /// 内部检查更新，获取最新队列
    fn inner_check_update<T>(last_id: T, income: Vec<DataItem>) -> Option<Vec<DataItem>>
    where
        T: AsRef<str>,
    {
        if income.len() > 1 {
            let first = unsafe { income.get_unchecked(0) };
            if first.id != last_id.as_ref() {
                let mut res = Vec::with_capacity(income.len());
                let mut find_last = false;
                for inner in income {
                    if inner.id != last_id.as_ref() {
                        res.push(inner)
                    } else {
                        find_last = true;
                        break;
                    }
                }
                match find_last {
                    true => Some(res),
                    false => Some(vec![res.into_iter().next().unwrap()]),
                }
            } else {
                None
            }
        } else {
            unreachable!()
        }
    }
}

#[cfg(test)]
mod test_updater {

    use serde_json::Value;

    use super::Updater;

    fn init_value() -> Value {
        serde_json::json!(
            {
                "Mock":[
                {
                   "dataSource":"Mock",
                   "id":"Mock_id_0",
                   "timeForSort":0,
                   "timeForDisplay":"",
                   "content":"Mock",
                   "jumpUrl":"",
                   "imageList":[],
                   "imageHttpList":[]
                },
                {
                   "dataSource":"Mock",
                   "id":"Mock_id_1",
                   "timeForSort":0,
                   "timeForDisplay":"",
                   "content":"Mock",
                   "jumpUrl":"",
                   "imageList":[],
                   "imageHttpList":[]
                },
                {
                   "dataSource":"Mock",
                   "id":"Mock_id_2",
                   "timeForSort":0,
                   "timeForDisplay":"",
                   "content":"Mock",
                   "jumpUrl":"",
                   "imageList":[],
                   "imageHttpList":[]
                },
            ]}
        )
    }

    fn update_value_normal() -> Value {
        serde_json::json!(
            {
                "Mock":[
                {
                   "dataSource":"Mock",
                   "id":"Mock_id_-1",
                   "timeForSort":0,
                   "timeForDisplay":"",
                   "content":"Mock",
                   "jumpUrl":"",
                   "imageList":[],
                   "imageHttpList":[]
                },
                {
                    "dataSource":"Mock",
                    "id":"Mock_id_0",
                    "timeForSort":0,
                    "timeForDisplay":"",
                    "content":"Mock",
                    "jumpUrl":"",
                    "imageList":[],
                    "imageHttpList":[]
                 },
                 {
                    "dataSource":"Mock",
                    "id":"Mock_id_1",
                    "timeForSort":0,
                    "timeForDisplay":"",
                    "content":"Mock",
                    "jumpUrl":"",
                    "imageList":[],
                    "imageHttpList":[]
                 },
            ]}
        )
    }

    fn init(init: bool) -> Updater {
        let mut updater = Updater::default();
        if init {
            let mock_init = serde_json::from_value(init_value()).unwrap();
            updater.check_update(mock_init);
        }
        updater
    }

    #[test]
    fn test_no_init() {
        // 第一次启动，没有任何记录
        let mut updater = init(false);

        assert_eq!(updater.last_id.len(), 0);

        let res = updater.check_update(serde_json::from_value(init_value()).unwrap());

        assert_eq!(res.len(), 1);
        assert_eq!(res.get("Mock").unwrap().len(), 3);
        assert_eq!(
            updater
                .last_id
                .get("Mock")
                .and_then(|s| Some(s.to_string())),
            Some("Mock_id_0".to_string())
        );
    }

    #[test]
    fn test_normal_update() {
        // 第一次更新后
        let mut updater = init(true);
        assert_eq!(updater.last_id.len(), 1);

        let res = updater.check_update(serde_json::from_value(update_value_normal()).unwrap());

        assert_eq!(res.len(), 1);
        assert_eq!(res.get("Mock").unwrap().len(), 1);
        {
            let last = &updater.last_id;
            let new_id = last.get("Mock").unwrap().to_string();
            assert_eq!(new_id, "Mock_id_-1".to_string())
        }
    }
}
