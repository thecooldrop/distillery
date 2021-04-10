use diesel::prelude::*;
use diesel::associations::HasTable;
use diesel::query_builder::{AsQuery, AsChangeset};
use diesel::query_dsl::filter_dsl::FilterDsl;
use diesel::sql_types::SingleValue;
use diesel::expression::AsExpression;
use diesel::dsl::And;

/// Trait indicating that the struct has a column, which serves as version
pub trait HasVersion {
    type Version: Column + ExpressionMethods;
    fn version() -> Self::Version;
}

/// Trait to indicate that struct can be versioned
pub trait Versionable: HasVersion {
    type VersionValue: AsExpression<<<Self as HasVersion>::Version as Expression>::SqlType>;
    fn version_value(self) -> Self::VersionValue;
}

pub trait OptimisticLocked: HasVersion {
    type Output;
    fn update<T: Connection>(&self, conn: &T) -> Self::Output;
}

type PrimaryKeySqlType<T> = <<<T as HasTable>::Table as diesel::Table>::PrimaryKey as diesel::Expression>::SqlType;
type TablePrimaryKey<T> = <<T as HasTable>::Table as Table>::PrimaryKey;
type TableQuery<T> = <<T as HasTable>::Table as AsQuery>::Query;
type PrimaryKeyEqId<T> = diesel::dsl::Eq< <<T as HasTable>::Table as Table>::PrimaryKey , <T as Identifiable>::Id>;
type VersionEq<T> = diesel::dsl::Eq<<T as HasVersion>::Version, <T as Versionable>::VersionValue>;
type FindableByVersionAndPrimaryKey<T> = And<PrimaryKeyEqId<T>, VersionEq<T>>;


impl<T> OptimisticLocked for T where
    T: HasTable + Versionable,
    for<'a> &'a T: Identifiable,
    TablePrimaryKey<T>: ExpressionMethods,
    for<'a> <&'a T as Identifiable>::Id: AsExpression<PrimaryKeySqlType<T>>,
    // TableQuery<T>: FilterDsl<FindableByVersionAndPrimaryKey<T>>,
{
    type Output = Self;

    fn update<Conn: Connection>(&self, _conn: &Conn) -> Self::Output {
        let table_struct = T::table();
        let pk_col = T::table().primary_key();
        let id = self.id();
        let pk_eq_cond = pk_col.eq(id);
        // let pk_eq_cond = pk_col.eq(self.id())
        //     .and(Self::version().eq(self.version_value()));
        // let filtered_elem = QueryDsl::filter(table_struct, pk_eq_cond);
        unimplemented!()
    }
}
//
// impl<T> Versionable for T where
//     T: Identifiable + HasTable,
//     <<T as HasTable>::Table as AsQuery>::Query: FilterDsl<diesel::dsl::Eq< <T::Table as Table>::PrimaryKey, T::Id> >,
//     <<<T as HasTable>::Table as diesel::Table>::PrimaryKey as diesel::Expression>::SqlType: SingleValue,
//     <T as diesel::Identifiable>::Id: diesel::Expression<SqlType = <<<T as HasTable>::Table as diesel::Table>::PrimaryKey as diesel::Expression>::SqlType>,
// {
//     fn update(self, conn: &PgConnection) -> Self {
//         // use crate::schema::product_category;
//         let table_struct = T::table();
//         let pk_col = T::table().primary_key();
//         let pk_eq_cond = pk_col.eq(self.id());
//         QueryDsl::filter(T::table(), pk_eq_cond);
//         // let filter = Self::table().filter(Self::table().primary_key().eq(&self.id()));
//         unimplemented!()
//     }
// }


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
