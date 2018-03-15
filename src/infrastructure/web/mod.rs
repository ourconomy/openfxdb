use rocket::{self, Rocket};
use rocket_contrib::Json;
use rocket::config::{Config, Environment};
use business::db::Db;
use infrastructure::error::AppError;
use business::sort::Rated;
use std::result;
use diesel::r2d2::{self, Pool};
use std::collections::HashMap;
use std::sync::Mutex;

#[cfg(feature = "email")]
use super::mail;

lazy_static! {
    static ref ENTRY_RATINGS: Mutex<HashMap<String, f64>> = Mutex::new(HashMap::new());
}

mod api;
mod util;
pub mod sqlite;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod mockdb;

use self::sqlite::create_connection_pool;

type Result<T> = result::Result<Json<T>, AppError>;


#[get("/effects/<ids>")]
fn get_effect(db: State<DbPool>, ids: String) -> Result<Vec<json::Effect>> {
    let ids = extract_ids(&ids);
    let effects = usecase::get_effects(&*db.get()?, &ids)?;
    let tags = usecase::get_tags_by_effect_ids(&*db.get()?, &ids)?;
    //our debugly
    println!("tags in web::get_effect: {:?}", tags);
    //our: might be needed in scope:
    let ratings = usecase::get_ratings_by_entry_ids(&*db.get()?, &ids)?;
    Ok(Json(effects
        .into_iter()
        .map(|e|{
            let t = tags.get(&e.id).cloned().unwrap_or_else(|| vec![]);
            let r = ratings.get(&e.id).cloned().unwrap_or_else(|| vec![]);
            json::Effect::from_effect_with_tags_and_ratings(e,t,r) 
        })
        .collect::<Vec<json::Effect>>()))
}


#[put("/effects/<id>", format = "application/json", data = "<e>")]
fn put_effect(db: State<DbPool>, id: String, e: Json<usecase::UpdateEffect>) -> Result<String> {
    let e = e.into_inner();
    usecase::update_effect(&mut *db.get()?, e.clone())?;
    //our: let email_addresses = usecase::email_addresses_to_notify(&e.lat, &e.lng, &mut *db.get()?);
    // let all_categories = db.get()?.all_categories()?;
    // notify_update_entry(email_addresses, &e, all_categories);
    Ok(Json(id))
}

#[post("/effects", format = "application/json", data = "<e>")]
fn post_effect(db: State<DbPool>, e: Json<usecase::NewEffect>) -> Result<String> {
    let e = e.into_inner();
    let id = usecase::create_new_effect(&mut *db.get()?, e.clone())?;
    //our: let email_addresses = usecase::email_addresses_to_notify(&e.lat, &e.lng, &mut *db.get()?);
    //let all_categories = db.get()?.all_categories()?;
    //notify_create_entry(email_addresses, &e, &id, all_categories);
    Ok(Json(id))
}


//our: old search api, keep as documentation for now
//  #[get("/search?<search>")]
//  fn get_search(db: State<DbPool>, search: SearchQuery) -> Result<json::SearchResult> {

//      let entries = db.get()?.all_entries()?;

//      let bbox = geo::extract_bbox(&search.bbox)
//          .map_err(Error::Parameter)
//          .map_err(AppError::Business)?;

//      let mut entries: Vec<&Entry> = entries.iter().collect();

//      if let Some(cat_str) = search.categories {
//          let cat_ids = extract_ids(&cat_str);
//          entries = entries
//              .into_iter()
//              .filter(&*filter::entries_by_category_ids(&cat_ids))
//              .collect();
//      }

//      let mut tags = vec![];

//      if let Some(ref txt) = search.text {
//          tags = extract_hash_tags(txt);
//      }


//      if let Some(tags_str) = search.tags {
//          for t in extract_ids(&tags_str) {
//              tags.push(t);
//          }
//      }

//      let triples = db.get()?.all_triples()?;

//      let text = match search.text {
//          Some(txt) => remove_hash_tags(&txt),
//          None => "".into()

//      let entries : Vec<&Entry> = entries
//          .into_iter()
//          .filter(&*filter::entries_by_tags_or_search_text(&text, &tags, &triples))
//          .collect();

//      let mut entries : Vec<Entry> = entries.into_iter().cloned().collect();

//      let all_ratings = db.get()?.all_ratings()?;

//      entries.sort_by_avg_rating(&all_ratings, &triples);

//      let visible_results: Vec<_> = entries
//          .iter()
//          .filter(|x| x.in_bbox(&bbox))
//          .map(|x| &x.id)
//          .cloned()
//          .collect();

//      let invisible_results = entries
//          .iter()
//          .filter(|e| !visible_results.iter().any(|v| *v == e.id))
//          .take(MAX_INVISIBLE_RESULTS)
//          .map(|x| &x.id)
//          .cloned()
//          .collect::<Vec<_>>();

//      let effects = db.get()?.all_effects()?;
//      let effects = effects
//          .iter()
//          .map(|x| &x.id)
//          .cloned()
//          .collect::<Vec<_>>();
//      
//      Ok(Json(json::SearchResult {
//          visible: visible_results,
//          invisible: invisible_results,
//          effects: effects,
//      }))
//  }


fn calculate_all_ratings<D: Db>(db: &D) -> Result<()> {
    let entries = db.all_entries()?;
    let ratings = db.all_ratings()?;
    let mut avg_ratings = match ENTRY_RATINGS.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    for e in entries {
        avg_ratings.insert(e.id.clone(), e.avg_rating(&ratings));
    }
    Ok(Json(()))
}


fn calculate_rating_for_entry<D: Db>(db: &D, e_id: &str) -> Result<()> {
    let ratings = db.all_ratings()?;
    let e = db.get_entry(e_id)?;
    let mut avg_ratings = match ENTRY_RATINGS.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    avg_ratings.insert(e.id.clone(), e.avg_rating(&ratings));
    Ok(Json(()))
}

fn rocket_instance<T: r2d2::ManageConnection>(cfg: Config, pool: Pool<T>) -> Rocket
where
    <T as r2d2::ManageConnection>::Connection: Db,
{
    info!("Calculating the average rating of all entries...");
    calculate_all_ratings(&*pool.get().unwrap()).unwrap();
    rocket::custom(cfg, true)
        .manage(pool)
//      .mount("/",
//             routes![login,
//                     get_effect,
//                     post_effect,
//                     put_effect,
        .mount("/", api::routes())
}

pub fn run(db_url: &str, port: u16, enable_cors: bool) {
    if enable_cors {
        panic!(
            "enable-cors is currently not available until\
             \nhttps://github.com/SergioBenitez/Rocket/pull/141\nis merged :("
        );
    }

    let cfg = Config::build(Environment::Production)
        .address("127.0.0.1")
        .port(port)
        .finalize()
        .unwrap();

    let pool = create_connection_pool(db_url).unwrap();

    rocket_instance(cfg, pool).launch();
}
