#[macro_export]
macro_rules! dao_operator {
    ($parent:ty=>$curr:ident($cove:ident, err=$err:ty)) => {
    $crate::paste!{
        pub struct [<$curr Operate>]<'db,Conn>(&'db Conn);
        
        impl <'db, Conn> $crate::SubOperate<'db> for [<$curr Operate>]<'db,Conn>{
            type Parent = $parent<'db,Conn>;
            
            fn from_parent(parent: &'db Self::Parent)->Self{
                Self(parent)
            }
        }
        
        impl <'db, Conn> std::ops::Deref for [<$curr Operate>]<'db,Conn>{
            type Target = Conn;
            
            fn deref(&self) -> &Self::Target { self.0 }
        }
        
        impl<'db, Conn> $parent<'db, Conn> {
            pub fn $cove(&self) -> [<$curr Operate>]<'_, Conn> {
                $crate::SuperOperate::child(self)
            }
        }
        
        pub struct [<$curr Create>]<'db,Conn>(&'db Conn);
        pub struct [<$curr Delete>]<'db,Conn>(&'db Conn);
        pub struct [<$curr Retrieve>]<'db,Conn>(&'db Conn);
        pub struct [<$curr Update>]<'db,Conn>(&'db Conn);
        pub struct [<$curr Verify>]<'db,Conn>(&'db Conn);
        
        impl <'db, Conn> $crate::OperateTrait<'db> for [<$curr Operate>]<'db,Conn>{
            type Create = [<$curr Create>]<'db,Conn>;
            type Delete = [<$curr Delete>]<'db,Conn>;
            type Retrieve = [<$curr Retrieve>]<'db,Conn>;
            type Update = [<$curr Update>]<'db,Conn>;
            type Verify = [<$curr Verify>]<'db,Conn>;
            
            type Error = $err;
    
            fn create(&self)->Self::Create{
                [<$curr Create>](self.0)
            }
    
            fn delete(&self)->Self::Delete{
                [<$curr Delete>](self.0)
            }
    
            fn retrieve(&self)->Self::Retrieve{
                [<$curr Retrieve>](self.0)
            }
    
            fn update(&self)->Self::Update{
                [<$curr Update>](self.0)
            }
    
            fn verify(&self)->Self::Verify{
                [<$curr Verify>](self.0)
            }
        }
        
        impl <'db, Conn> std::ops::Deref for [<$curr Create>]<'db,Conn>{
            type Target = Conn;
            
            fn deref(&self) -> &Self::Target { self.0 }
        }
        impl <'db, Conn> std::ops::Deref for [<$curr Delete>]<'db,Conn>{
            type Target = Conn;
            
            fn deref(&self) -> &Self::Target { self.0 }
        }
        impl <'db, Conn> std::ops::Deref for [<$curr Retrieve>]<'db,Conn>{
            type Target = Conn;
            
            fn deref(&self) -> &Self::Target { self.0 }
        }
        impl <'db, Conn> std::ops::Deref for [<$curr Update>]<'db,Conn>{
            type Target = Conn;
            
            fn deref(&self) -> &Self::Target { self.0 }
        }        
        impl <'db, Conn> std::ops::Deref for [<$curr Verify>]<'db,Conn>{
            type Target = Conn;
            
            fn deref(&self) -> &Self::Target { self.0 }
        }
        
        type Result<T> = core::result::Result<T, $err>;
    }
    };
}