#![feature(type_alias_impl_trait)]
mod db_io;
mod encoder;
mod utils;

use std::future::Future;

pub use encoder::{DecodeReq, Decoder, EncodeError, Encoder};
pub use sled::Config;
use tower::ServiceExt;
pub use utils::{and_then::AndThen, echo::Echo};

pub mod db {
    pub use crate::db_io::actor_tree::{ActorTree, SavePair, SavePairError, SavePairSer, ToTree};
    pub use crate::db_io::db::SledDb;
    pub use crate::db_io::tree::SledTree;
}

pub use sled;

pub mod prefab {
    use crate::db_io::actor_tree::ToTree;
    use crate::AndThen;
    use crate::{
        db::{ActorTree, SavePairSer, SledTree},
        Decoder, Echo, Encoder,
    };

    pub type SavePairGenerator = SavePairSer<Echo, Encoder<Echo>>;
    pub type Saver<T = SledTree> = ActorTree<T>;
    pub type Loader<T = SledTree> = Decoder<AndThen<ActorTree<T>>>;

    pub fn new_save_pair() -> SavePairGenerator {
        SavePairSer(Echo, Encoder(Echo))
    }

    pub fn new_saver<T: ToTree>(tree: T) -> Saver<T> {
        ActorTree(tree)
    }

    pub fn new_loader<T: ToTree>(tree: T) -> Loader<T> {
        Decoder(AndThen(ActorTree(tree)))
    }
}

pub async fn do_call<S, Req>(service: &mut S, req: Req) -> <S::Future as Future>::Output
where
    S: tower::Service<Req>,
{
    let ser = service.ready().await?;
    ser.call(req).await
}

#[cfg(test)]
mod test {
    use sled::Config;
    use tower::Service;

    use crate::{
        db_io::{
            actor_tree::{ActorTree, SavePairSer},
            db::SledDb,
            tree::SledTree,
        },
        utils::echo::Echo,
        AndThen, DecodeReq, Decoder, Encoder,
    };

    type SaveP = SavePairSer<Echo, Encoder<Echo>>;
    type Save = ActorTree<SledTree>;
    type Load = Decoder<AndThen<ActorTree<SledTree>>>;
    #[tokio::test]
    pub async fn save() {
        let mut sp: SaveP = SavePairSer(Echo, Encoder(Echo));

        let cfg = Config::default().temporary(true);
        let mut db = SledDb::new_from_config(cfg).unwrap();

        let tree = db.call("New Tree").await.unwrap();
        let etree = tree.clone();
        let mut s: Save = ActorTree(tree);
        let mut l: Load = Decoder(AndThen(ActorTree(etree)));

        let save = sp.call(("aa", "bb")).await.unwrap();
        let res = s.call(save).await.unwrap();
        // let res=s.call(SavePair::new("AAA", "BBB")).await;
        println!("res {:?}", res);

        let res: String = l.call(DecodeReq::new("aa")).await.unwrap();

        println!("res {}", res);
    }
}
