use crate::{Deserialize, Serialize};

macro_rules! tuple {
    ($($p:ident: $tp:ident),* $(,)?) => {
        impl<E, $($tp),*> Serialize<E> for ($($tp),*,)
        where
            $($tp: Serialize<E>),*
        {
            fn serialize(self, ser: &mut crate::Serializer) -> Result<(), E> {
                let ($($p),*,) = self;

                $($p.serialize(ser)?);*;

                Ok(())
            }
        }

        impl<E, $($tp),*> Deserialize<E> for ($($tp),*,)
        where
            $($tp: Deserialize<E>),*
        {
            fn deserialize(des: &mut crate::Deserializer) -> Result<Self, E> {
                Ok((
                    $($tp::deserialize(des)?),*,
                ))
            }
        }
    };
}

impl<E> Serialize<E> for () {
    fn serialize(self, _ser: &mut crate::Serializer) -> Result<(), E> {
        Ok(())
    }
}

impl<E> Deserialize<E> for () {
    fn deserialize(_des: &mut crate::Deserializer) -> Result<Self, E> {
        Ok(())
    }
}

tuple!(t1: T1);
tuple!(t1: T1, t2: T2);
tuple!(t1: T1, t2: T2, t3: T3);
tuple!(t1: T1, t2: T2, t3: T3, t4: T4);
tuple!(t1: T1, t2: T2, t3: T3, t4: T4, t5: T5);
tuple!(t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6);
tuple!(t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6, t7: T7);
tuple!(t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6, t7: T7, t8: T8);
tuple!(t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6, t7: T7, t8: T8, t9: T9);
tuple!(t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6, t7: T7, t8: T8, t9: T9, t10: T10);
tuple!(t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6, t7: T7, t8: T8, t9: T9, t10: T10, t11: T11);
tuple!(t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6, t7: T7, t8: T8, t9: T9, t10: T10, t11: T11, t12: T12);
