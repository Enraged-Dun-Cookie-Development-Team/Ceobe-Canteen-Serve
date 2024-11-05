pub trait EndpointType: Default {}

#[derive(Default)]
pub struct UserEnd;

impl EndpointType for UserEnd {}
#[derive(Default)]
pub struct AdminEnd;

impl EndpointType for AdminEnd {}
#[derive(Default)]
pub struct CDN;

impl EndpointType for CDN {}
#[derive(Default)]
pub struct Internal;

impl EndpointType for Internal {}
