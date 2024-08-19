pub trait EndpointType {}

pub struct UserEnd;

impl EndpointType for UserEnd {}

pub struct AdminEnd;

impl EndpointType for AdminEnd {}

pub struct CDN;

impl EndpointType for CDN {}

pub struct Internal;

impl EndpointType for Internal {}
