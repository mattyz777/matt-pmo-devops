
#[macro_export]
macro_rules! soft_delete_filter {
    ($entity:ident) => {
        sea_orm::Condition::all().add($entity::Column::IsDeleted.eq(false))
    };
}

pub use soft_delete_filter;
