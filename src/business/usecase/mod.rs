use super::error::{Error, ParameterError, RepoError};
use std::result;
use std::string; //oc
use chrono::*;
use entities::*;
use super::db::Db;
use super::filter;
use super::validate::{self, Validate};
use uuid::Uuid;
use std::collections::HashMap;
use pwhash::bcrypt;
use super::geo;
use super::sort::SortByAverageRating;
use super::filter::InBBox;
use std::fmt::Display; //oc
use std::str::FromStr; //oc
use serde::{de, Deserialize, Deserializer}; //oc
use std::result::Result as OtherResult; //oc

#[cfg(test)]
pub mod tests;

type Result<T> = result::Result<T, Error>;

trait Id {
    fn id(&self) -> String;
}

impl Id for Entry {
    fn id(&self) -> String {
        self.id.clone()
    }
}

impl Id for Effect {
    fn id(&self) -> String {
        self.id.clone()
    }
}

impl Id for Category {
    fn id(&self) -> String {
        self.id.clone()
    }
}

impl Id for Tag {
    fn id(&self) -> String {
        self.id.clone()
    }
}

impl Id for User {
    fn id(&self) -> String {
        self.id.clone()
    }
}

impl Id for Comment {
    fn id(&self) -> String {
        self.id.clone()
    }
}

impl Id for Rating {
    fn id(&self) -> String {
        self.id.clone()
    }
}

impl Id for BboxSubscription {
    fn id(&self) -> String {
        self.id.clone()
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Deserialize, Debug, Clone)]
pub struct NewEntry {
    pub title       : String,
    pub description : String,
    pub lat         : f64,
    pub lng         : f64,
    pub street      : Option<String>,
    pub zip         : Option<String>,
    pub city        : Option<String>,
    pub country     : Option<String>,
    pub email       : Option<String>,
    pub telephone   : Option<String>,
    pub homepage    : Option<String>,
    pub categories  : Vec<String>,
    pub tags        : Vec<String>,
    pub license     : String,
}

//oc section, struct definitions

//oc Serde needs to know explicitly how to deserialize u64 und f64
pub fn string_to_number<'de, T, D>(deserializer: D) -> OtherResult<T, D::Error>
    where T: FromStr,
          T::Err: Display,
          D: Deserializer<'de>
{
String::deserialize(deserializer)?.parse().map_err(de::Error::custom)
}

#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewOrigin {
    pub label       : Option<String>,
    pub value       : Option<String>,
}

#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewUpstreamEffect {
    pub label : Option<String>,
    pub value : Option<String>,
}

#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewUpstream {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    #[serde(deserialize_with = "string_to_number")]
    pub upstreamNo            : u64,
    pub upstreamEffect        : Option<NewUpstreamEffect>,
    pub upstreamTransferUnit  : Option<String>,
    #[cfg_attr(rustfmt, rustfmt_skip)]
    #[serde(deserialize_with = "string_to_number")]
    pub upstreamAmount        : f64,
    pub upstreamComment       : Option<String>,
}

#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Deserialize, Debug, Clone)]
pub struct NewEffect {
    pub title       : String,
    pub description : String,
    pub origin      : Option<NewOrigin>,
    pub homepage    : Option<String>,
    pub tags        : Vec<String>,
    pub upstreams   : Option<Vec<NewUpstream>>,
    pub license     : String,
}

#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateEffect {
    pub id          : String,
    pub version     : u64,
    pub title       : String,
    pub description : String,
    pub origin      : Option<NewOrigin>,
    pub homepage    : Option<String>,
    pub tags        : Vec<String>,
    pub upstreams   : Option<Vec<NewUpstream>>,
}
//end

#[derive(Deserialize, Debug, Clone)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Deserialize, Debug, Clone)]
pub struct Login {
    username: String,
    password: String,
}

#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateEntry {
    pub id          : String,
    pub osm_node    : Option<u64>,
    pub version     : u64,
    pub title       : String,
    pub description : String,
    pub lat         : f64,
    pub lng         : f64,
    pub street      : Option<String>,
    pub zip         : Option<String>,
    pub city        : Option<String>,
    pub country     : Option<String>,
    pub email       : Option<String>,
    pub telephone   : Option<String>,
    pub homepage    : Option<String>,
    pub categories  : Vec<String>,
    pub tags        : Vec<String>,
}

#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Deserialize, Debug, Clone)]
pub struct RateEntry {
    pub entry   : String,
    pub title   : String,
    pub value   : i8,
    pub context : RatingContext,
    pub comment : String,
    pub source  : Option<String>,
    pub user    : Option<String>,
}

#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Debug, Clone)]
pub struct SearchRequest<'a> {
    pub bbox          : Bbox,
    pub categories    : Option<Vec<String>>,
    pub text          : String,
    pub tags          : Vec<String>,
    pub entry_ratings : &'a HashMap<String, f64>,
}

pub fn get_ratings<D: Db>(db: &D, ids: &[String]) -> Result<Vec<Rating>> {
    Ok(db.all_ratings()?
        .iter()
        .filter(|x| ids.iter().any(|id| *id == x.id))
        .cloned()
        .collect())
}

pub fn get_ratings_by_entry_ids<D: Db>(
    db: &D,
    ids: &[String],
) -> Result<HashMap<String, Vec<Rating>>> {
    let ratings = db.all_ratings()?;
    Ok(ids.iter()
        .map(|e_id| {
            (
                e_id.clone(),
                ratings
                    .iter()
                    .filter(|r| r.entry_id == **e_id)
                    .cloned()
                    .collect(),
            )
        })
        .collect())
}

pub fn get_comments_by_rating_ids<D: Db>(
    db: &D,
    ids: &[String],
) -> Result<HashMap<String, Vec<Comment>>> {
    let comments = db.all_comments()?;
    Ok(ids.iter()
        .map(|r_id| {
            (
                r_id.clone(),
                comments
                    .iter()
                    .filter_map(|comment| {
                        if comment.rating_id == *r_id {
                            Some(comment)
                        } else {
                            None
                        }
                    })
                    .cloned()
                    .collect(),
            )
        })
        .collect())
}

pub fn get_entries<D: Db>(db: &D, ids: &[String]) -> Result<Vec<Entry>> {
    let entries = db.all_entries()?
        .into_iter()
        .filter(|e| ids.iter().any(|id| *id == e.id))
        .collect();
    Ok(entries)
}

//oc section
pub fn get_effects<D:Db>(db : &D, ids : &[String]) -> Result<Vec<Effect>> {
    let effects = db.all_effects()?
        .into_iter()
        .filter(|e| ids.iter().any(|id| *id == e.id))
        .collect();
    Ok(effects)
}

pub fn get_upstreams<D:Db>(db : &D, ids : &[String]) -> Result<Vec<Upstream>> {
    let upstreams = db.all_upstreams()?
        .into_iter()
        .filter(|u| ids.iter().any(|id| *id == u.effect_id))
        .collect();
    Ok(upstreams)
}

pub fn create_new_effect<D: Db>(db: &mut D, e: NewEffect) -> Result<String> {
    let mut tags: Vec<_> = e.tags.into_iter().map(|t| t.replace("#", "")).collect();
    tags.dedup();
    let now = Utc::now().timestamp() as u64;
    let new_id = Uuid::new_v4().simple().to_string();
    let mut new_upstreams = vec![];
    if e.upstreams.is_some() {
        //only upstream #[cfg_attr(rustfmt, rustfmt_skip)]
        new_upstreams = e.upstreams.unwrap().into_iter()
            .map(|u| Upstream{
                id                  : Uuid::new_v4().simple().to_string(),
                created             : now.clone(),
                effect_id           : new_id.clone(),
                effect_version      : 0,
                upstream_effect     : u.upstreamEffect.clone()
                    .map_or(None, |ue| ue.label),
                upstream_effect_id  : u.upstreamEffect
                    .map_or(None, |ue| ue.value),
                number              : Some(u.upstreamNo),
                transfer_unit       : u.upstreamTransferUnit,
                amount              : Some(u.upstreamAmount),
                comment             : u.upstreamComment,
            }
        ).collect();
    }
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let new_effect = Effect{
        id          :  new_id,
        created     :  now,
        version     :  0,
        title       :  e.title,
        description :  e.description,
        origin      :  e.origin.clone().map_or(None, |ue| ue.label),
        origin_id   :  e.origin.map_or(None, |ue| ue.value),
        homepage    :  e.homepage,
        tags,
        license     :  Some(e.license)
    };

    //oc: we don't need to val homepage and email yet:
    // new_effect.validate()?;

    for t in &new_effect.tags {
        db.create_tag_if_it_does_not_exist(&Tag { id: t.clone() })?;
    }
    db.create_effect(&new_effect)?;
    for u in &new_upstreams {
        db.create_upstream(u)?;
    }

    Ok(new_effect.id)
}

pub fn update_effect<D: Db>(db: &mut D, e: UpdateEffect) -> Result<()> {
    let old : Effect = db.get_effect(&e.id)?;
    if (old.version + 1) != e.version {
        return Err(Error::Repo(RepoError::InvalidVersion))
    }
    let now = Utc::now().timestamp() as u64;
    let mut tags = e.tags.clone();
    tags.dedup();

    let mut new_upstreams = vec![];
    if (e.upstreams.is_some()) {
        //only experimental #[cfg_attr(rustfmt, rustfmt_skip)]
        new_upstreams = e.upstreams.clone().unwrap().into_iter()
            .map(|u| Upstream{
                id                  : Uuid::new_v4().simple().to_string(),
                created             : now.clone(),
                effect_id           : e.id.clone(),
                effect_version      : e.version.clone(),
                upstream_effect     : u.upstreamEffect.clone().map_or(None, |ue| ue.label),
                upstream_effect_id  : u.upstreamEffect.map_or(None, |ue| ue.value),
                number              : Some(u.upstreamNo),
                transfer_unit       : u.upstreamTransferUnit,
                amount              : Some(u.upstreamAmount),
                comment             : u.upstreamComment,
            }
        ).collect();
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    let new_effect = Effect{
        id          :  e.id,
        created     :  now,
        version     :  e.version,
        title       :  e.title,
        description :  e.description,
        origin      :  e.origin.clone().map_or(None, |ue| ue.label),
        origin_id   :  e.origin.map_or(None, |ue| ue.value),
        homepage    :  e.homepage,
        tags,
        license     :  old.license
    };
    for t in &new_effect.tags {
        db.create_tag_if_it_does_not_exist(&Tag { id: t.clone() })?;
    }
    db.update_effect(&new_effect)?;
    for u in &new_upstreams {
        db.create_upstream(u)?;
    }
    Ok(())
}

//end

pub fn create_new_user<D: Db>(db: &mut D, u: NewUser) -> Result<()> {
    validate::username(&u.username)?;
    validate::password(&u.password)?;
    validate::email(&u.email)?;
    if db.get_user(&u.username).is_ok() {
        return Err(Error::Parameter(ParameterError::UserExists));
    }
    let pw = bcrypt::hash(&u.password)?;
    db.create_user(&User {
        id: Uuid::new_v4().simple().to_string(),
        username: u.username,
        password: pw,
        email: u.email,
        email_confirmed: false,
    })?;
    Ok(())
}

pub fn get_user<D: Db>(
    db: &mut D,
    logged_in_username: &str,
    requested_username: &str,
) -> Result<(String, String)> {
    let u: User = db.get_user(requested_username)?;
    if logged_in_username != requested_username {
        return Err(Error::Parameter(ParameterError::Forbidden));
    }
    Ok((u.username, u.email))
}

pub fn delete_user(db: &mut Db, login_id: &str, u_id: &str) -> Result<()> {
    if login_id != u_id {
        return Err(Error::Parameter(ParameterError::Forbidden));
    }
    db.delete_user(login_id)?;
    Ok(())
}

pub fn login<D: Db>(db: &mut D, login: &Login) -> Result<String> {
    match db.get_user(&login.username) {
        Ok(u) => {
            if bcrypt::verify(&login.password, &u.password) {
                if u.email_confirmed {
                    Ok(login.username.clone())
                } else {
                    Err(Error::Parameter(ParameterError::EmailNotConfirmed))
                }
            } else {
                Err(Error::Parameter(ParameterError::Credentials))
            }
        }
        Err(err) => match err {
            RepoError::NotFound => Err(Error::Parameter(ParameterError::Credentials)),
            _ => Err(Error::Repo(RepoError::Other(Box::new(err)))),
        },
    }
}

pub fn create_new_entry<D: Db>(db: &mut D, e: NewEntry) -> Result<String> {
    let mut tags: Vec<_> = e.tags.into_iter().map(|t| t.replace("#", "")).collect();
    tags.dedup();

    #[cfg_attr(rustfmt, rustfmt_skip)]
    let new_entry = Entry{
        id          :  Uuid::new_v4().simple().to_string(),
        osm_node    :  None,
        created     :  Utc::now().timestamp() as u64,
        version     :  0,
        title       :  e.title,
        description :  e.description,
        lat         :  e.lat,
        lng         :  e.lng,
        street      :  e.street,
        zip         :  e.zip,
        city        :  e.city,
        country     :  e.country,
        email       :  e.email,
        telephone   :  e.telephone,
        homepage    :  e.homepage,
        categories  :  e.categories,
        tags,
        license     :  Some(e.license)
    };
    new_entry.validate()?;
    for t in &new_entry.tags {
        db.create_tag_if_it_does_not_exist(&Tag { id: t.clone() })?;
    }
    db.create_entry(&new_entry)?;
    Ok(new_entry.id)
}

pub fn update_entry<D: Db>(db: &mut D, e: UpdateEntry) -> Result<()> {
    let old: Entry = db.get_entry(&e.id)?;
    if (old.version + 1) != e.version {
        return Err(Error::Repo(RepoError::InvalidVersion));
    }
    let mut tags = e.tags;
    tags.dedup();
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let new_entry = Entry{
        id          :  e.id,
        osm_node    :  None,
        created     :  Utc::now().timestamp() as u64,
        version     :  e.version,
        title       :  e.title,
        description :  e.description,
        lat         :  e.lat,
        lng         :  e.lng,
        street      :  e.street,
        zip         :  e.zip,
        city        :  e.city,
        country     :  e.country,
        email       :  e.email,
        telephone   :  e.telephone,
        homepage    :  e.homepage,
        categories  :  e.categories,
        tags,
        license     :  old.license
    };
    for t in &new_entry.tags {
        db.create_tag_if_it_does_not_exist(&Tag { id: t.clone() })?;
    }
    db.update_entry(&new_entry)?;
    Ok(())
}

pub fn rate_entry<D: Db>(db: &mut D, r: RateEntry) -> Result<()> {
    let e = db.get_entry(&r.entry)?;
    if r.comment.len() < 1 {
        return Err(Error::Parameter(ParameterError::EmptyComment));
    }
    if r.value > 2 || r.value < -1 {
        return Err(Error::Parameter(ParameterError::RatingValue));
    }
    let now = Utc::now().timestamp() as u64;
    let rating_id = Uuid::new_v4().simple().to_string();
    let comment_id = Uuid::new_v4().simple().to_string();
    #[cfg_attr(rustfmt, rustfmt_skip)]
    db.create_rating(&Rating{
        id       : rating_id.clone(),
        entry_id : e.id,
        created  : now,
        title    : r.title,
        value    : r.value,
        context  : r.context,
        source   : r.source
    })?;
    #[cfg_attr(rustfmt, rustfmt_skip)]
    db.create_comment(&Comment {
        id: comment_id.clone(),
        created: now,
        text: r.comment,
        rating_id,
    })?;
    Ok(())
}

pub fn subscribe_to_bbox(coordinates: &[Coordinate], username: &str, db: &mut Db) -> Result<()> {
    if coordinates.len() != 2 {
        return Err(Error::Parameter(ParameterError::Bbox));
    }
    let bbox = Bbox {
        south_west: coordinates[0].clone(),
        north_east: coordinates[1].clone(),
    };
    validate::bbox(&bbox)?;

    // TODO: support multiple subscriptions in KVM (frontend)
    // In the meanwile we just replace existing subscriptions
    // with a new one.
    unsubscribe_all_bboxes_by_username(db, username)?;

    let id = Uuid::new_v4().simple().to_string();
    db.create_bbox_subscription(&BboxSubscription {
        id,
        bbox,
        username: username.into(),
    })?;
    Ok(())
}

pub fn get_bbox_subscriptions(username: &str, db: &Db) -> Result<Vec<BboxSubscription>> {
    Ok(db.all_bbox_subscriptions()?
        .into_iter()
        .filter(|s| s.username == username)
        .collect())
}

pub fn unsubscribe_all_bboxes_by_username(db: &mut Db, username: &str) -> Result<()> {
    let user_subscriptions: Vec<_> = db.all_bbox_subscriptions()?
        .into_iter()
        .filter(|s| s.username == username)
        .map(|s| s.id)
        .collect();
    for s_id in user_subscriptions {
        db.delete_bbox_subscription(&s_id)?;
    }
    Ok(())
}

pub fn bbox_subscriptions_by_coordinate(
    db: &mut Db,
    x: &Coordinate,
) -> Result<Vec<BboxSubscription>> {
    Ok(db.all_bbox_subscriptions()?
        .into_iter()
        .filter(|s| geo::is_in_bbox(&x.lat, &x.lng, &s.bbox))
        .collect())
}

pub fn email_addresses_from_subscriptions(
    db: &mut Db,
    subs: &[BboxSubscription],
) -> Result<Vec<String>> {
    let usernames: Vec<_> = subs.iter().map(|s| &s.username).collect();

    let mut addresses: Vec<_> = db.all_users()?
        .into_iter()
        .filter(|u| usernames.iter().any(|x| **x == u.username))
        .map(|u| u.email)
        .collect();
    addresses.dedup();
    Ok(addresses)
}

pub fn email_addresses_by_coordinate(db: &mut Db, lat: &f64, lng: &f64) -> Result<Vec<String>> {
    let subs = bbox_subscriptions_by_coordinate(
        db,
        &Coordinate {
            lat: *lat,
            lng: *lng,
        },
    )?;
    let addresses = email_addresses_from_subscriptions(db, &subs)?;
    Ok(addresses)
}

const MAX_INVISIBLE_RESULTS: usize = 5;

const BBOX_LAT_EXT: f64 = 0.02;
const BBOX_LNG_EXT: f64 = 0.04;

fn extend_bbox(bbox: &Bbox) -> Bbox {
    let mut extended_bbox = bbox.to_owned();
    extended_bbox.south_west.lat -= BBOX_LAT_EXT;
    extended_bbox.south_west.lng -= BBOX_LNG_EXT;
    extended_bbox.north_east.lat += BBOX_LAT_EXT;
    extended_bbox.north_east.lng += BBOX_LNG_EXT;
    extended_bbox
}

pub fn search<D: Db>(db: &D, req: &SearchRequest) -> Result<(Vec<Entry>, Vec<Entry>, Vec<Effect>)> {
    let mut entries = if req.text.is_empty() && req.tags.is_empty() {
        let extended_bbox = extend_bbox(&req.bbox);
        db.get_entries_by_bbox(&extended_bbox)?
    } else {
        db.all_entries()?
    };

    if let Some(ref cat_ids) = req.categories {
        entries = entries
            .into_iter()
            .filter(&*filter::entries_by_category_ids(cat_ids))
            .collect();
    }

    let mut entries: Vec<_> = entries
        .into_iter()
        .filter(&*filter::entries_by_tags_or_search_text(
            &req.text,
            &req.tags,
        ))
        .collect();

    entries.sort_by_avg_rating(req.entry_ratings);

    let visible_results: Vec<_> = entries
        .iter()
        .filter(|x| x.in_bbox(&req.bbox))
        .cloned()
        .collect();

    let invisible_results = entries
        .into_iter()
        .filter(|x| !x.in_bbox(&req.bbox))
        .take(MAX_INVISIBLE_RESULTS)
        .collect();

//oc section
    let mut effects = db.all_effects()?;

    let mut effects: Vec<_> = effects
        .into_iter()
        .filter(&*filter::effects_by_search_text(
            &req.text,
            //our &req.tags,   convert into one search text
        ))
        .collect();

    let effect_results = effects;
//end

    Ok((visible_results, invisible_results, effect_results))
}
