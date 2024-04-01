use sqlx_core::{
    database::{Database, HasArguments, HasValueRef},
    decode::Decode,
    encode::{Encode, IsNull},
    types::Type,
};

use crate::Brand;

type BoxError = Box<dyn core::error::Error + Send + Sync + 'static>;

impl<Db, Tag, Inner> Type<Db> for Brand<Tag, Inner>
where
    Db: Database,
    Inner: Type<Db>,
{
    fn type_info() -> Db::TypeInfo {
        Inner::type_info()
    }
}

impl<'de, Db, Tag, Inner> Decode<'de, Db> for Brand<Tag, Inner>
where
    Db: Database,
    Inner: for<'a> Decode<'a, Db>,
{
    fn decode(value: <Db as HasValueRef<'de>>::ValueRef) -> Result<Brand<Tag, Inner>, BoxError> {
        let inner = <Inner as Decode<Db>>::decode(value)?;
        Ok(Brand::unchecked_from_inner(inner))
    }
}

impl<'en, Db, Tag, Inner> Encode<'en, Db> for Brand<Tag, Inner>
where
    Db: Database,
    Inner: for<'a> Encode<'a, Db>,
{
    fn encode_by_ref(&self, buf: &mut <Db as HasArguments<'en>>::ArgumentBuffer) -> IsNull {
        self.inner.encode_by_ref(buf)
    }
}
