use entities as e;

#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Serialize)]
pub struct Entry {
    pub id          : String,
    pub created     : u64,
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
    pub ratings     : Vec<String>,
    pub license     : Option<String>,
}

//oc section
#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Serialize)]
pub struct Effect {
    pub id          : String,
    pub created     : u64,
    pub version     : u64,
    pub title       : String,
    pub description : String,
    pub origin      : OriginResponse,
    pub homepage    : Option<String>,
    pub tags        : Vec<String>,
    pub upstreams   : Vec<UpstreamResponse>,
    pub ratings     : Vec<String>,
    pub license     : Option<String>,
}

#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Serialize)]
pub struct OriginResponse {
    pub value       : Option<String>,
    pub label       : Option<String>,
}

#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Serialize)]
pub struct UpstreamResponse {
    pub upstreamNo            : Option<String>,
    pub upstreamEffect        : Option<String>,
    pub upstreamTransferUnit  : Option<String>,
    pub upstreamAmount        : Option<String>,
    pub upstreamComment       : Option<String>,
}
//end

#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Serialize, Deserialize)]
pub struct Rating {
    pub id          : String,
    pub title       : String,
    pub created     : u64,
    pub value       : i8,
    pub context     : e::RatingContext,
    pub comments    : Vec<Comment>,
    pub source      : String
}

#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Serialize, Deserialize)]
pub struct Comment {
    pub id          : String,
    pub created     : u64,
    pub text        : String,
}

#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Serialize)]
pub struct EntryIdWithCoordinates {
    pub id : String,
    pub lat: f64,
    pub lng: f64,
}

#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Serialize)]
pub struct SearchResponse {
    pub visible   : Vec<EntryIdWithCoordinates>,
    pub invisible : Vec<EntryIdWithCoordinates>,
    pub effects   : Vec<String> //oc line
}

#[derive(Serialize)]
pub struct User {
    pub username: String,
    pub email: String,
}

#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Serialize)]
pub struct BboxSubscription{
    pub id              : String,
    pub south_west_lat  : f64,
    pub south_west_lng  : f64,
    pub north_east_lat  : f64,
    pub north_east_lng  : f64,
}

// Entity -> JSON

#[cfg_attr(rustfmt, rustfmt_skip)]
impl Entry {
    pub fn from_entry_with_ratings(e: e::Entry, ratings: Vec<e::Rating>) -> Entry {
        Entry{
            id          : e.id,
            created     : e.created,
            version     : e.version,
            title       : e.title,
            description : e.description,
            lat         : e.lat,
            lng         : e.lng,
            street      : e.street,
            zip         : e.zip,
            city        : e.city,
            country     : e.country,
            email       : e.email,
            telephone   : e.telephone,
            homepage    : e.homepage,
            categories  : e.categories,
            tags        : e.tags,
            ratings     : ratings.into_iter().map(|r|r.id).collect(),
            license     : e.license,
        }
    }
}

//oc section
impl Effect {
    pub fn from_effect_with_ratings(e: e::Effect, u: Vec<e::Upstream>, ratings: Vec<e::Rating>, o: OriginResponse) -> Effect {
        Effect{
            id          : e.id,
            created     : e.created,
            version     : e.version,
            title       : e.title,
            description : e.description,
            origin      : o,
            homepage    : e.homepage,
            tags        : e.tags,
            upstreams   : u.into_iter().map(|u| UpstreamResponse {
                upstreamNo: u.number.map(|n| n.to_string()),
                upstreamEffect: u.upstream_effect, // and u.ups..effect_id
                upstreamTransferUnit: u.transfer_unit,
                upstreamAmount: u.amount.map(|a| a.to_string()),
                upstreamComment: u.comment,
            }).collect(),
            ratings     : ratings.into_iter().map(|r|r.id).collect(),
            license     : e.license,
        }
    }
}
//end
