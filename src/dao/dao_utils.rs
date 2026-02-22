/// 生成软删除过滤条件的宏
///
/// # 使用示例
/// ```
/// use crate::dao::soft_delete_filter;
/// use crate::entity::release_doc;
///
/// let filter = soft_delete_filter!(release_doc);
/// ```
#[macro_export]
macro_rules! soft_delete_filter {
    ($entity:ident) => {
        sea_orm::Condition::all().add($entity::Column::IsDelete.eq(false))
    };
}

pub use soft_delete_filter;
