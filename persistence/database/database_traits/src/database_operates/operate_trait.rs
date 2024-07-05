
pub trait OperateTrait<'db>{
    type Create;
    type Delete;
    
    type Retrieve;
    type Update;
    
    type Verify;
    
    type Error;
    
    fn create(&self)->Self::Create;
    
    fn delete(&self)->Self::Delete;
    
    fn retrieve(&self)->Self::Retrieve;
    
    fn update(&self)->Self::Update;
    
    fn verify(&self)->Self::Verify;
    
    
}