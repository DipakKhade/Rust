use dotenv::dotenv;
use mongodb::{ 
	bson::{Document, doc},
	Client,
	Collection 
};
use std::env;


#[tokio::main]
async fn main() {
    dotenv().ok();

    let uri = env::var("DB_URL").unwrap_or("mongodb://localhost:27017/".to_string());

    let client = Client::with_uri_str(uri).await.unwrap();

    let database = client.database("Blog");
    let my_coll: Collection<Document> = database.collection("blogdata");

    //find

    // let blog = my_coll.find_one(doc! { "title": "Introduction to Web Development" }).await.unwrap();
    
    // println!("Found a movie:\n{:#?}", blog);

    // insert one

    // let new_blog = doc! {
    //     "title": "non rel db in rust"
    // };

    // let new_doc_result = my_coll.insert_one(new_blog).await;
    // println!("result : --{:?}", new_doc_result);


    //insert many

    // let _ = my_coll.insert_many(vec![doc! {
    //     "title": "non rel db in rust"
    // }, doc! {
    //     "title": "blog title 2"
    // }]).await;


    // update many

    // let _ = my_coll.update_many(doc! {"title":"non rel db in rust"}, doc! {
    //     "$set": doc!{
    //         "title":"MongoDB in Rust", "description":"test desc"
    //     }
    // }).await;

    //delete

    // let _ = my_coll.delete_one(doc! {
    //     "description": "test desc"
    // }).await;


    // aggrigation

    let orders_coll: Collection<Document> = database.collection("orders");
    let result = orders_coll.aggregate([
        doc! {
                "$match": doc!{
                    "totalAmount": doc! {
                        "$gt": 1000
                    }
                }
                
        },
        doc! {
            "$project": doc! {
                    "_id": 1,
                    "email": 1,
                }
        },

        // doc! {
        //     "$sort"
        // }
    ]).await;

    println!("result --- {:#?}", result.unwrap().deserialize_current().unwrap());
}
