use warp::Filter;

use super::QuickServer;

impl QuickServer {
    pub async fn serve(self) {
        let root = warp::fs::dir(self.root);
        let index = warp::fs::file(self.index);

        let routes = root.or(index);

        warp::serve(routes)
            .run(self.host)
            .await;
    }
}
