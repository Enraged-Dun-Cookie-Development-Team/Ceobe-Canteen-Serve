use actix::{Actor, Addr, Context, Handler, MessageResult};
use actix_http::ws::Item;
use actix_web::web::{Bytes, BytesMut};

pub struct Continuation(MidData);

impl Actor for Continuation {
    type Context = Context<Self>;
}

impl Continuation {
    pub fn start() -> Addr<Self> {
        Self::create(|_ctx| Self(MidData::Nil))
    }
}

#[derive(actix::Message)]
#[rtype(result = "Option<FullData>")]
pub struct NextIncome(pub Item);

enum MidData {
    Text(BytesMut),
    Bin(BytesMut),
    Nil,
}

pub enum FullData {
    Text(Bytes),
    Bin(Bytes),
}

impl FullData {
    pub(crate) fn unwrap(self) -> Bytes {
        match self {
            FullData::Text(b) | FullData::Bin(b) => b,
        }
    }
}

impl Handler<NextIncome> for Continuation {
    type Result = MessageResult<NextIncome>;

    fn handle(&mut self, msg: NextIncome, _ctx: &mut Self::Context) -> Self::Result {
        let item = msg.0;

        let res = match (item, &mut self.0) {
            (Item::FirstText(n), _) => {
                self.0 = MidData::Text(BytesMut::from_iter(n));
                None
            }
            (Item::FirstBinary(n), _) => {
                self.0 = MidData::Bin(BytesMut::from_iter(n));
                None
            }
            (Item::Continue(new), MidData::Text(old))
            | (Item::Continue(new), MidData::Bin(old)) => {
                old.extend(new);
                None
            }
            (Item::Last(last), MidData::Text(old)) => {
                old.extend(last);
                let data = old.to_owned().into();
                old.clear();
                Some(FullData::Text(data))
            }
            (Item::Last(last), MidData::Bin(old)) => {
                old.extend(last);
                let data = old.to_owned().into();
                old.clear();
                Some(FullData::Bin(data))
            }
            (Item::Continue(_), MidData::Nil) | (Item::Last(_), MidData::Nil) => {
                self.0 = MidData::Nil;
                None
            }
        };

        MessageResult(res)
    }
}
