use entities::*;
use business::error::RepoError;
use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::result;
//use business::db::Db;
use business::dbfx::DbFX;
use super::models;
use super::schema;

type Result<T> = result::Result<T, RepoError>;

//    fn unset_current_on_all_entries(
//      con: &&mut SqliteConnection,
//      id: &str,
//  ) -> result::Result<usize, diesel::result::Error> {
//      use self::schema::entries::dsl;
//      diesel::update(dsl::entries.filter(dsl::id.eq(id)).filter(
//          dsl::current.eq(true),
//      )).set(dsl::current.eq(false))
//          .execute(*con)
//  }

fn unset_current_on_all_effects(
    con: &&mut SqliteConnection,
    id: &str,
) -> result::Result<usize, diesel::result::Error> {
    use self::schema::effects::dsl;
    diesel::update(dsl::effects.filter(dsl::id.eq(id)).filter(
        dsl::current.eq(true),
    )).set(dsl::current.eq(false))
        .execute(*con)
}

impl DbFX for SqliteConnection {
//  fn create_entry(&mut self, e: &Entry) -> Result<()> {
//      let new_entry = models::Entry::from(e.clone());
//      let cat_rels: Vec<_> = e.categories
//          .iter()
//          .cloned()
//          .map(|category_id| {
//              models::EntryCategoryRelation {
//                  entry_id: e.id.clone(),
//                  entry_version: e.version as i32,
//                  category_id,
//              }
//          })
//          .collect();
//      self.transaction::<_, diesel::result::Error, _>(|| {
//          unset_current_on_all_entries(&self, &e.id)?;
//          diesel::insert_into(schema::entries::table)
//              .values(&new_entry)
//              .execute(self)?;
//          diesel::insert_into(schema::entry_category_relations::table)
//              //WHERE NOT EXISTS
//              .values(&cat_rels)
//              .execute(self)?;
//          Ok(())
//      })?;
//      Ok(())
//  }
    fn create_effect(&mut self, e: &Effect) -> Result<()> {
        let new_effect= models::Effect::from(e.clone());
      //let cat_rels: Vec<_> = e.categories
      //    .iter()
      //    .cloned()
      //    .map(|category_id| {
      //        models::EntryCategoryRelation {
      //            entry_id: e.id.clone(),
      //            entry_version: e.version as i32,
      //            category_id,
      //        }
      //    })
      //    .collect();
        self.transaction::<_, diesel::result::Error, _>(|| {
            unset_current_on_all_effects(&self, &e.id)?;
            diesel::insert_into(schema::effects::table)
                .values(&new_effect)
                .execute(self)?;
            //diesel::insert_into(schema::entry_category_relations::table)
                //WHERE NOT EXISTS
              //  .values(&cat_rels)
              //  .execute(self)?;
            Ok(())
        })?;
        Ok(())
    }
//  fn create_tag(&mut self, t: &Tag) -> Result<()> {
//    diesel::insert_into(schema::tags::table)
//          .values(&models::Tag::from(t.clone()))
//          .execute(self)?;
//      Ok(())
//  }
    fn create_effect_triple(&mut self, t: &Triple) -> Result<()> {
        use self::schema::ratings::dsl as r_dsl;
        use self::schema::comments::dsl as c_dsl;
        use self::schema::bbox_subscriptions::dsl as b_dsl;
  
        match t.predicate {
//
//          // (entry)-[is_tagged_with]->(tag)
            Relation::IsTaggedWith => {
                match t.subject {
//                  ObjectId::Entry(ref e_id) => {
//                      match t.object {
//                          ObjectId::Tag(ref t_id) => {
//                              let e = self.get_entry(e_id)?;
//                              diesel::insert_into(schema::entry_tag_relations::table)
//                                  .values(&models::EntryTagRelation {
//                                      entry_id: e.id,
//                                      entry_version: e.version as i32,
//                                      tag_id: t_id.clone(),
//                                  })
//                                  .execute(self)?;
//                              return Ok(());
//                          }
//                          _ => {}
//                      }
//                  }
            // (effect)-[is_tagged_with]->(tag)
                    ObjectId::Effect(ref e_id) => { //our: effect tab req. 
                        match t.object {
                            ObjectId::Tag(ref t_id) => {
                                let e = self.get_effect(e_id)?;
                                //our check:
                                println!("--> values in connections::create_triple: \n effect_id: {:?} \n effect_version: {:?} \n tag_id: {:?}", &e.id, *&e.version as i32, &t_id);
                                diesel::insert_into(schema::effect_tag_relations::table)
                                    .values(&models::EffectTagRelation {
                                        effect_id: e.id,
                                        effect_version: e.version as i32,
                                        tag_id: t_id.clone(),
                                    })
                                    .execute(self)?;
                                return Ok(());
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }

//          // (entry)-[is_rated_with]->(rating)
//          Relation::IsRatedWith => {
//              match t.subject {
//                  ObjectId::Entry(ref e_id) => {
//                      match t.object {
//                          ObjectId::Rating(ref r_id) => {
//                              diesel::update(r_dsl::ratings.find(r_id))
//                                  .set(r_dsl::entry_id.eq(e_id))
//                                  .execute(self)?;
//                              return Ok(());
//                          }
//                          _ => {}
//                      }
//                  }
//                  _ => {}
//              }
//          }
//
//          // (rating)-[is_commented_with]->(comment)
//          Relation::IsCommentedWith => {
//              match t.subject {
//                  ObjectId::Rating(ref r_id) => {
//                      match t.object {
//                          ObjectId::Comment(ref c_id) => {
//                              diesel::update(c_dsl::comments.find(c_id))
//                                  .set(c_dsl::rating_id.eq(r_id))
//                                  .execute(self)?;
//                              return Ok(());
//                          }
//                          _ => {}
//                      }
//                  }
//                  _ => {}
//              }
//          }
//          // (user)-[subscribed_to)->(bbox_subscription)
//          Relation::SubscribedTo => {
//              match t.subject {
//                  ObjectId::User(ref u_id) => {
//                      match t.object {
//                          ObjectId::BboxSubscription(ref s_id) => {
//                              diesel::update(b_dsl::bbox_subscriptions.find(s_id))
//                                  .set(b_dsl::user_id.eq(u_id))
//                                  .execute(self)?;
//                              return Ok(());
//                          }
//                          _ => {}
//                      }
//                  }
//                  _ => {}
//              }
//          }
//          _ => {}
        }
        warn!("did not save triple '{:?}'", t);
        Ok(())
  
    }
//  fn create_user(&mut self, u: &User) -> Result<()> {
//      diesel::insert_into(schema::users::table)
//          .values(&models::User::from(u.clone()))
//          .execute(self)?;
//      Ok(())
//  }
//  fn create_comment(&mut self, c: &Comment) -> Result<()> {
//      diesel::insert_into(schema::comments::table)
//          .values(&models::Comment::from(c.clone()))
//          .execute(self)?;
//      Ok(())
//  }
//  fn create_rating(&mut self, r: &Rating) -> Result<()> {
//      diesel::insert_into(schema::ratings::table)
//          .values(&models::Rating::from(r.clone()))
//          .execute(self)?;
//      Ok(())
//  }
//  fn create_bbox_subscription(&mut self, sub: &BboxSubscription) -> Result<()> {
//      diesel::insert_into(schema::bbox_subscriptions::table)
//          .values(&models::BboxSubscription::from(sub.clone()))
//          .execute(self)?;
//      Ok(())
//  }
//  fn all_users(&self) -> Result<Vec<User>> {
//      use self::schema::users::dsl;
//      Ok(
//          dsl::users
//              .load::<models::User>(self)?
//              .into_iter()
//              .map(User::from)
//              .collect(),
//      )
//  }
//  fn all_bbox_subscriptions(&self) -> Result<Vec<BboxSubscription>> {
//      use self::schema::bbox_subscriptions::dsl;
//      Ok(
//          dsl::bbox_subscriptions
//              .load::<models::BboxSubscription>(self)?
//              .into_iter()
//              .map(BboxSubscription::from)
//              .collect(),
//      )
//  }
//  fn confirm_email_address(&mut self, username: &str) -> Result<User> {
//      use self::schema::users::dsl;
//      diesel::update(dsl::users.find(username))
//          .set(dsl::email_confirmed.eq(true))
//          .execute(self)?;
//      Ok(User::from(self.get_user(username)?))
//  }
//  fn delete_bbox_subscription(&mut self, id: &str) -> Result<()> {
//      use self::schema::bbox_subscriptions::dsl;
//      diesel::delete(dsl::bbox_subscriptions.find(id)).execute(
//          self,
//      )?;
//      Ok(())
//  }
//  fn delete_user(&mut self, user: &str) -> Result<()> {
//      use self::schema::users::dsl::*;
//      diesel::delete(users.find(user)).execute(self)?;
//      Ok(())
//  }
//
//  fn get_entry(&self, e_id: &str) -> Result<Entry> {
//      use self::schema::entries::dsl as e_dsl;
//      use self::schema::entry_category_relations::dsl as e_c_dsl;
//
//      let models::Entry {
//          id,
//          created,
//          version,
//          title,
//          description,
//          lat,
//          lng,
//          street,
//          zip,
//          city,
//          country,
//          email,
//          telephone,
//          homepage,
//          license,
//          ..
//      } = e_dsl::entries
//          .filter(e_dsl::id.eq(e_id))
//          .filter(e_dsl::current.eq(true))
//          .first(self)?;
//
//      let categories = e_c_dsl::entry_category_relations
//          .filter(e_c_dsl::entry_id.eq(&id))
//          .load::<models::EntryCategoryRelation>(self)?
//          .into_iter()
//          .map(|r| r.category_id)
//          .collect();
//
//      Ok(Entry {
//          id,
//          created: created as u64,
//          version: version as u64,
//          title,
//          description,
//          lat: lat as f64,
//          lng: lng as f64,
//          street,
//          zip,
//          city,
//          country,
//          email,
//          telephone,
//          homepage,
//          categories,
//          license,
//      })
//  }



    fn get_effect(&self, e_id: &str) -> Result<Effect> {
        use self::schema::effects::dsl as e_dsl;
        //our: unused import: use self::schema::entry_category_relations::dsl as e_c_dsl;

        let models::Effect {
            id,
            created,
            version,
            title,
            description,
            origin,
            license,
            ..
        } = e_dsl::effects
            .filter(e_dsl::id.eq(e_id))
            .filter(e_dsl::current.eq(true))
            .first(self)?;

        //our: currently unused?
      //let categories = e_c_dsl::entry_category_relations
      //    .filter(e_c_dsl::entry_id.eq(&id))
      //    .load::<models::EntryCategoryRelation>(self)?
      //    .into_iter()
      //    .map(|r| r.category_id)
      //    .collect();

        Ok(Effect {
            id,
            created: created as u64,
            version: version as u64,
            title,
            description,
            origin,
            //categories,
            license,
        })
    }

//  fn get_user(&self, user_id: &str) -> Result<User> {
//      use self::schema::users::dsl::*;
//      let u: models::User = users.find(user_id).first(self)?;
//      Ok(User::from(u))
//  }
//
//  fn all_entries(&self) -> Result<Vec<Entry>> {
//      use self::schema::entries::dsl as e_dsl;
//      use self::schema::entry_category_relations::dsl as e_c_dsl;
//
//      let entries: Vec<models::Entry> =
//          e_dsl::entries.filter(e_dsl::current.eq(true)).load(self)?;
//
//      let cat_rels = e_c_dsl::entry_category_relations
//          .load::<models::EntryCategoryRelation>(self)?;
//
//      Ok(
//          entries
//              .into_iter()
//              .map(|e| {
//                  let cats = cat_rels
//                      .iter()
//                      .filter(|r| r.entry_id == e.id)
//                      .filter(|r| r.entry_version == e.version)
//                      .map(|r| &r.category_id)
//                      .cloned()
//                      .collect();
//                  Entry {
//                      id: e.id,
//                      created: e.created as u64,
//                      version: e.version as u64,
//                      title: e.title,
//                      description: e.description,
//                      lat: e.lat as f64,
//                      lng: e.lng as f64,
//                      street: e.street,
//                      zip: e.zip,
//                      city: e.city,
//                      country: e.country,
//                      email: e.email,
//                      telephone: e.telephone,
//                      homepage: e.homepage,
//                      categories: cats,
//                      license: e.license,
//                  }
//              })
//              .collect(),
//      )
//  }
      fn all_effects(&self) -> Result<Vec<Effect>> {
          use self::schema::effects::dsl as e_dsl;
          //use self::schema::entry_category_relations::dsl as e_c_dsl;
  
          let effects: Vec<models::Effect> =
              e_dsl::effects.filter(e_dsl::current.eq(true)).load(self)?;
  
          // If I understand correctly cat_rels are not used
          //let cat_rels = e_c_dsl::entry_category_relations
          //    .load::<models::EntryCategoryRelation>(self)?;
  
          Ok(
            effects
                .into_iter()
                .map(|e| {
          //          let cats = cat_rels
          //            .iter()
          //            .filter(|r| r.entry_id == e.id)
          //            .filter(|r| r.entry_version == e.version)
          //            .map(|r| &r.category_id)
          //            .cloned()
          //            .collect();
                      Effect {
                          id: e.id,
                          created: e.created as u64,
                          version: e.version as u64,
                          title: e.title,
                          description: e.description,
                          origin: e.origin,
                          license: e.license,
                      }
                  })
                  .collect(),
          )
      }
//    fn all_categories(&self) -> Result<Vec<Category>> {
//        use self::schema::categories::dsl::*;
//        Ok(
//            categories
//                .load::<models::Category>(self)?
//                .into_iter()
//                .map(Category::from)
//                .collect(),
//        )
//    }
//    fn all_tags(&self) -> Result<Vec<Tag>> {
//        use self::schema::tags::dsl::*;
//        Ok(
//            tags.load::<models::Tag>(self)?
//                .into_iter()
//                .map(Tag::from)
//                .collect(),
//        )
//
//    }
      fn all_triples(&self) -> Result<Vec<Triple>> {
//        use self::schema::entry_tag_relations::dsl as e_t_dsl;
          use self::schema::effect_tag_relations::dsl as ef_t_dsl;
//        use self::schema::ratings::dsl as r_dsl;
//        use self::schema::comments::dsl as c_dsl;
//        use self::schema::bbox_subscriptions::dsl as b_dsl;
//
//        // (entry)-[is_tagged_with]->(tag)
//        let mut e_t_triples: Vec<_> = e_t_dsl::entry_tag_relations
//            .load::<models::EntryTagRelation>(self)?
//            .into_iter()
//            .map(Triple::from)
//            .collect();
//
          // (effect)-[is_tagged_with]->(tag)
          let mut ef_t_triples: Vec<_> = ef_t_dsl::effect_tag_relations
              .load::<models::EffectTagRelation>(self)?
              .into_iter()
              .map(Triple::from)
              .collect();
  
//        // (entry)-[is_rated_with]->(rating)
//        let mut e_r_triples: Vec<_> = r_dsl::ratings
//            .load::<models::Rating>(self)?
//            .into_iter()
//            .map(Triple::from)
//            .collect();
//
//        // (rating)-[is_commented_with]->(comment)
//        let mut r_c_triples: Vec<_> = c_dsl::comments
//            .load::<models::Comment>(self)?
//            .into_iter()
//            .map(Triple::from)
//            .collect();
//
//        // (user)-[subscribed_to)->(bbox_subscription)
//        let mut u_b_triples: Vec<_> = b_dsl::bbox_subscriptions
//            .load::<models::BboxSubscription>(self)?
//            .into_iter()
//            .map(Triple::from)
//            .collect();
//
//
          let mut result = vec![];
//        result.append(&mut e_t_triples);
          result.append(&mut ef_t_triples);
//        result.append(&mut e_r_triples);
//        result.append(&mut r_c_triples);
//        result.append(&mut u_b_triples);
//
          Ok(result)
      }
//    fn all_ratings(&self) -> Result<Vec<Rating>> {
//        use self::schema::ratings::dsl::*;
//        Ok(
//            ratings
//                .load::<models::Rating>(self)?
//                .into_iter()
//                .map(Rating::from)
//                .collect(),
//        )
//    }
//    fn all_comments(&self) -> Result<Vec<Comment>> {
//        use self::schema::comments::dsl::*;
//        Ok(
//            comments
//                .load::<models::Comment>(self)?
//                .into_iter()
//                .map(Comment::from)
//                .collect(),
//        )
//    }
//
//    fn update_entry(&mut self, entry: &Entry) -> Result<()> {
//
//        let e = models::Entry::from(entry.clone());
//
//        let cat_rels: Vec<_> = entry
//            .categories
//            .iter()
//            .cloned()
//            .map(|category_id| {
//                models::EntryCategoryRelation {
//                    entry_id: entry.id.clone(),
//                    entry_version: entry.version as i32,
//                    category_id,
//                }
//            })
//            .collect();
//
//        self.transaction::<_, diesel::result::Error, _>(|| {
//            unset_current_on_all_entries(&self, &e.id)?;
//            diesel::insert_into(schema::entries::table)
//                .values(&e)
//                .execute(self)?;
//            diesel::insert_into(schema::entry_category_relations::table)
//                //WHERE NOT EXISTS
//                .values(&cat_rels)
//                .execute(self)?;
//            Ok(())
//        })?;
//        Ok(())
//    }
//
  
      fn update_effect(&mut self, effect: &Effect) -> Result<()> {
  
          let e = models::Effect::from(effect.clone());
  
        //  let cat_rels: Vec<_> = effect
        //    .categories
        //    .iter()
        //    .cloned()
        //    .map(|category_id| {
        //        models::EntryCategoryRelation {
        //            entry_id: effect.id.clone(), //workaround
        //            entry_version: effect.version as i32, //workaround
        //            category_id,
        //        }
        //    })
        //    .collect();
  
          self.transaction::<_, diesel::result::Error, _>(|| {
              unset_current_on_all_effects(&self, &e.id)?;
              diesel::insert_into(schema::effects::table)
                .values(&e)
                .execute(self)?;
        //    diesel::insert_into(schema::entry_category_relations::table)
                //WHERE NOT EXISTS
         //       .values(&cat_rels)
           //     .execute(self)?;
            Ok(())
        })?;
        Ok(())
    }

    fn delete_effect_triple(&mut self, t: &Triple) -> Result<()> {
        use self::schema::entry_tag_relations::dsl as e_t_dsl;
//      use self::schema::ratings::dsl as r_dsl;
//      use self::schema::comments::dsl as c_dsl;
//      use self::schema::bbox_subscriptions::dsl as b_dsl;
  
        match t.predicate {
  
//          // (entry)-[is_tagged_with]->(tag)
//          Relation::IsTaggedWith => {
//              match t.subject {
//                  ObjectId::Entry(ref e_id) => {
//                      match t.object {
//                          ObjectId::Tag(ref t_id) => {
//                              let e = self.get_entry(e_id)?;
//                              diesel::delete(e_t_dsl::entry_tag_relations.find(
//                                  (e.id, e.version as i32, t_id),
//                              )).execute(self)?;
//                              return Ok(());
//                          }
//                          _ => {}
//                      }
//                  }
//                  _ => {}
//              }
//          }
//
            // (effect)-[is_tagged_with]->(tag)
            Relation::IsTaggedWith => {
                match t.subject {
                    ObjectId::Effect(ref e_id) => {
                        match t.object {
                            ObjectId::Tag(ref t_id) => {
                                let e = self.get_effect(e_id)?;
                                diesel::delete(e_t_dsl::effect_tag_relations.find(
                                    (e.id, e.version as i32, t_id),
                                )).execute(self)?;
                                return Ok(());
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
  
//          // (entry)-[is_rated_with]->(rating)
//          Relation::IsRatedWith => {
//              match t.subject {
//                  ObjectId::Entry(_) => {
//                      match t.object {
//                          ObjectId::Rating(ref r_id) => {
//                              diesel::update(r_dsl::ratings.find(r_id))
//                                  .set(&models::RatingUpdate { entry_id: None })
//                                  .execute(self)?;
//                              return Ok(());
//                          }
//                          _ => {}
//                      }
//                  }
//                  _ => {}
//              }
//          }
//
//          // (rating)-[is_commented_with]->(comment)
//          Relation::IsCommentedWith => {
//              match t.subject {
//                  ObjectId::Rating(_) => {
//                      match t.object {
//                          ObjectId::Comment(ref c_id) => {
//                              diesel::update(c_dsl::comments.find(c_id))
//                                  .set(&models::CommentUpdate { rating_id: None })
//                                  .execute(self)?;
//                              return Ok(());
//                          }
//                          _ => {}
//                      }
//                  }
//                  _ => {}
//              }
//          }
//          // (user)-[subscribed_to)->(bbox_subscription)
//          Relation::SubscribedTo => {
//              match t.subject {
//                  ObjectId::User(_) => {
//                      match t.object {
//                          ObjectId::BboxSubscription(ref s_id) => {
//                              diesel::update(b_dsl::bbox_subscriptions.find(s_id))
//                                  .set(&models::BboxSubscriptionUpdate { user_id: None })
//                                  .execute(self)?;
//                              return Ok(());
//                          }
//                          _ => {}
//                      }
//                  }
//                  _ => {}
//              }
//          }
//          _ => {}
        }
        warn!("did not delete triple '{:?}'", t);
        Ok(())
    }
} 
